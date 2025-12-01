#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]
#![feature(once_cell_try)]
#![feature(if_let_guard)]

use std::sync::Arc;

use derive_more::Debug;
use eventric_model::{
    decision::{
        Decision,
        Events,
        Execute,
    },
    event::{
        Codec,
        Event,
        json,
    },
    projection::{
        Projection,
        Update,
        UpdateEvent,
    },
};
use eventric_stream::{
    error::Error,
    stream::{
        Stream,
        append::AppendSelect,
        iterate::IterateSelect,
    },
};
use fancy_constructor::new;
use serde::{
    Deserialize,
    Serialize,
};

// =================================================================================================
// Course Manager
// =================================================================================================

// Events

#[derive(new, Debug, Deserialize, Event, Serialize)]
#[event(identifier(course_registered), tags(course(&this.id)))]
pub struct CourseRegistered {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

#[derive(new, Debug, Deserialize, Event, Serialize)]
#[event(identifier(course_withdrawn), tags(course(&this.id)))]
pub struct CourseWithdrawn {
    #[new(into)]
    pub id: String,
}

// Projections

#[derive(new, Debug, Projection)]
#[projection(select(events(CourseRegistered, CourseWithdrawn), filter(course(&this.id))))]
pub struct CourseExists {
    #[new(default)]
    pub exists: bool,
    #[new(into)]
    pub id: String,
}

impl Update<CourseRegistered> for CourseExists {
    fn update(&mut self, _: UpdateEvent<'_, CourseRegistered>) {
        self.exists = true;
    }
}

impl Update<CourseWithdrawn> for CourseExists {
    fn update(&mut self, _: UpdateEvent<'_, CourseWithdrawn>) {
        self.exists = false;
    }
}

// Decisions

#[derive(new, Debug, Decision)]
#[decision(projection(CourseExists: CourseExists::new(&this.id)))]
pub struct RegisterCourse {
    #[new(into)]
    pub id: String,
    #[new(into)]
    pub title: String,
    pub capacity: u8,
}

impl Execute for RegisterCourse {
    fn execute<C>(
        &mut self,
        context: &mut Events<C>,
        projections: &Self::Projections,
    ) -> Result<(), Error>
    where
        C: Codec,
    {
        if !projections.course_exists.exists {
            context.append(CourseRegistered::new(&self.id, &self.title, self.capacity))?;
        }

        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

// Experimental...

#[derive(new, Debug)]
pub struct DecisionContext<'a, C>
where
    C: Codec,
{
    codec: Arc<C>,
    stream: &'a mut Stream,
}

impl<C> DecisionContext<'_, C>
where
    C: Codec,
{
    pub fn execute<D>(&mut self, mut decision: D) -> Result<(), Error>
    where
        D: Decision,
    {
        let mut after = None;
        let mut projections = decision.projections();

        let selections = decision.select(&projections)?;

        let (events, select) = self.stream.iter_select(selections, None);

        for event in events {
            let event = event?;
            let codec = self.codec.clone();
            let position = *event.position();

            after = Some(position);

            decision.update(codec, &event, &mut projections)?;
        }

        let codec = self.codec.clone();

        let mut events = Events::new(codec);

        decision.execute(&mut events, &projections)?;

        let events = events.take();

        if !events.is_empty() {
            self.stream.append_select(events, select, after)?;
        }

        Ok(())
    }
}

// -------------------------------------------------------------------------------------------------

// Temporary Example Logic...

pub fn main() -> Result<(), Error> {
    let codec = Arc::new(json::Codec);

    let mut stream = Stream::builder("./temp").temporary(false).open()?;
    let mut context = DecisionContext::new(codec, &mut stream);

    context.execute(RegisterCourse::new("my_course", "My Course", 30))
}
