use eventric_stream::{
    error::Error,
    event::{
        CandidateEvent,
        Data,
        Version,
    },
};
use fancy_constructor::new;

use crate::{
    decision::Context,
    event::Event,
};

// =================================================================================================
// Execute
// =================================================================================================

pub trait Execute: Context
where
    Self::Err: From<Error>,
{
    type Err;
    type Ok;

    fn execute(&mut self, context: &mut Self::Context) -> Result<Self::Ok, Self::Err>;
}

#[derive(new, Debug)]
pub struct Events {
    #[new(default)]
    events: Vec<CandidateEvent>,
}

impl Events {
    pub fn append<E>(&mut self, event: &E) -> Result<(), Error>
    where
        E: Event,
    {
        let data = revision::to_vec(event).map_err(|_| Error::data("serialization error"))?;
        let data = Data::new(data)?;

        let identifier = E::identifier().cloned()?;
        let tags = event.tags()?;
        let version = Version::default();

        let event = CandidateEvent::new(data, identifier, tags, version);

        self.events.push(event);

        Ok(())
    }

    #[must_use]
    pub fn take(self) -> Vec<CandidateEvent> {
        self.events
    }
}
