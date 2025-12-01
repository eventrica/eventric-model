use std::sync::Arc;

use eventric_stream::{
    error::Error,
    event::CandidateEvent,
};
use fancy_constructor::new;

use crate::{
    decision::Projections,
    event::{
        Codec,
        Event,
    },
};

// =================================================================================================
// Execute
// =================================================================================================

pub trait Execute: Projections {
    fn execute<C>(
        &mut self,
        events: &mut Events<C>,
        projections: &Self::Projections,
    ) -> Result<(), Error>
    where
        C: Codec;
}

#[derive(new, Debug)]
pub struct Events<C>
where
    C: Codec,
{
    codec: Arc<C>,
    #[new(default)]
    events: Vec<CandidateEvent>,
}

impl<C> Events<C>
where
    C: Codec,
{
    pub fn append<E>(&mut self, event: E) -> Result<(), Error>
    where
        E: Event,
    {
        let event = self.codec.encode(event)?;

        self.events.push(event);

        Ok(())
    }

    #[must_use]
    pub fn take(self) -> Vec<CandidateEvent> {
        self.events
    }
}
