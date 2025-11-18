pub(crate) mod macros;
pub(crate) mod query;

use std::any::Any;

use eventric_stream::{
    error::Error,
    event::PersistentEvent,
    stream::query::Query,
};

use crate::event::{
    Codec,
    Event,
};

// =================================================================================================
// Projection
// =================================================================================================

// Projection

pub trait Projection: QuerySource {}

// Dispatch

pub trait Dispatch {
    fn dispatch(&mut self, event: &Box<dyn Any>);
}

// Recognise

pub trait Recognize {
    fn recognize<C>(codec: &C, event: &PersistentEvent) -> Result<Option<Box<dyn Any>>, Error>
    where
        C: Codec;
}

// Query Source

pub trait QuerySource {
    fn query(&self) -> Result<Query, Error>;
}

// Update

pub trait Update<E>
where
    E: Event,
{
    fn update(&mut self, event: &E);
}
