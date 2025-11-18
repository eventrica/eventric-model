#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::multiple_crate_versions)]
#![allow(missing_docs)]

// Temporary Event Wrapper

use std::any::Any;

use eventric_stream::{
    error::Error,
    event::PersistentEvent,
};
use eventric_surface::event::Identified;
use fancy_constructor::new;

#[derive(new, Debug)]
pub struct DeserializedPersistentEvent {
    pub deserialized: Box<dyn Any>,
    pub event: PersistentEvent,
}

impl DeserializedPersistentEvent {
    pub fn deserialize_as<T>(&self) -> Result<Option<&T>, Error>
    where
        T: Identified + 'static,
    {
        if self.event.identifier() != T::identifier()? {
            return Ok(None);
        }

        Ok(self.deserialized.downcast_ref::<T>())
    }
}
