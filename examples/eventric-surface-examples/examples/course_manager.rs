#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]
#![feature(once_cell_try)]
#![feature(if_let_guard)]

use std::{
    any::Any,
    sync::OnceLock,
};

use derive_more::Debug;
use eventric_stream::{
    error::Error,
    event::{
        Data,
        EphemeralEvent,
        Identifier,
        PersistentEvent,
        Tag,
        Version,
    },
    stream::{
        Stream,
        append,
        query::{
            self,
            Query,
            Selector,
        },
    },
};
use eventric_surface_examples::{
    Decision,
    DeserializedPersistentEvent,
    GetIdentifier,
    GetQuery,
    GetSpecifier as _,
    GetTags,
    Update,
};
use fancy_constructor::new;
use serde::{
    Deserialize,
    Serialize,
};

// NOTES

// At least initially, event versioning will be ignored entirely (all versions
// will be set to zero for now, until a meaningful model is in place).

// -------------------------------------------------------------------------------------------------

// Theoretically Generated...

static COURSE_REGISTERED_IDENTIFIER: OnceLock<Identifier> = OnceLock::new();

impl TryFrom<CourseRegistered> for EphemeralEvent {
    type Error = Error;

    fn try_from(event: CourseRegistered) -> Result<Self, Self::Error> {
        let data = serde_json::to_vec(&event).map_err(|_| Error::data("serialization"))?;
        let data = Data::new(data)?;

        let identifier = CourseRegistered::identifier().cloned()?;
        let tags = event.tags()?;
        let version = Version::default();

        Ok(EphemeralEvent::new(data, identifier, tags, version))
    }
}

impl TryFrom<&PersistentEvent> for CourseRegistered {
    type Error = Error;

    fn try_from(event: &PersistentEvent) -> Result<Self, Self::Error> {
        serde_json::from_slice(event.data().as_ref()).map_err(|_| Error::data("deserialization"))
    }
}

impl GetIdentifier for CourseRegistered {
    fn identifier() -> Result<&'static Identifier, Error> {
        COURSE_REGISTERED_IDENTIFIER.get_or_try_init(|| Identifier::new("course_registered"))
    }
}

impl GetTags for CourseRegistered {
    fn tags(&self) -> Result<Vec<Tag>, Error> {
        [Tag::new(format!("course_id:{}", self.id))]
            .into_iter()
            .collect()
    }
}

#[derive(Debug)]
pub enum CourseExistsEvent<'a> {
    CourseRegistered(&'a CourseRegistered),
}

impl<'a> Decision<'a> for CourseExists {
    type Event = CourseExistsEvent<'a>;

    fn filter_deserialize(event: &'a PersistentEvent) -> Result<Option<Box<dyn Any>>, Error> {
        let event = match event.identifier() {
            identifier if identifier == CourseRegistered::identifier()? => {
                let event: CourseRegistered = event.try_into()?;
                let event = Box::new(event) as Box<dyn Any>;

                Some(event)
            }
            _ => None,
        };

        Ok(event)
    }

    fn filter_map(event: &'a DeserializedPersistentEvent) -> Result<Option<Self::Event>, Error> {
        let event = match event {
            event if let Some(event) = event.deserialize_as::<CourseRegistered>()? => {
                let event = Self::Event::CourseRegistered(event);

                Some(event)
            }
            _ => None,
        };

        Ok(event)
    }
}

// -------------------------------------------------------------------------------------------------

// Events

#[derive(new, Debug, Deserialize, Serialize)]
pub struct CourseRegistered {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

// Decisions

#[derive(new, Debug)]
pub struct CourseExists {
    #[new(default)]
    pub exists: bool,
    #[new(into)]
    pub id: String,
}

impl GetQuery for CourseExists {
    fn query(&self) -> Result<Query, Error> {
        Query::new([Selector::specifiers_and_tags(
            [CourseRegistered::specifier()?],
            [Tag::new(format!("course_id:{}", self.id))?],
        )?])
    }
}

impl Update<'_> for CourseExists {
    fn update(&mut self, event: Self::Event) {
        match event {
            Self::Event::CourseRegistered(_event) => self.exists = true,
        }
    }
}

// Example...

pub fn main() -> Result<(), Error> {
    let mut stream = Stream::builder("./temp").temporary(false).open()?;

    let course_id = "some_course";

    println!("creating new decision");

    let mut decision = CourseExists::new(course_id);

    println!("current decision state: {decision:#?}");

    let query = decision.query()?;
    let condition = query::Condition::default().matches(&query);

    let mut position = None;

    println!("running decision query");

    for event in stream.query(&condition, None) {
        let event = event?;

        position = Some(*event.position());

        if let Some(deserialized) = CourseExists::filter_deserialize(&event)? {
            let event = DeserializedPersistentEvent::new(deserialized, event);

            if let Some(event) = CourseExists::filter_map(&event)? {
                println!("applying update to decision: {event:#?}");

                decision.update(event);

                println!("current decision state: {decision:#?}");
            }
        }
    }

    println!("making decision");
    println!("current decision state: {decision:#?}");

    if decision.exists {
        println!("decision invalid, course already exists");
    } else {
        println!("decision valid, creating condition to append");

        let mut condition = append::Condition::new(&query);

        if let Some(position) = position {
            println!("extending append condition with after position");

            condition = condition.after(position);
        }

        println!("appending new events");

        let event = CourseRegistered::new(course_id, "My Course", 30);
        let event = event.try_into()?;

        stream.append([&event], Some(&condition))?;
    }

    Ok(())
}
