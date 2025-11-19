use eventric_stream::{
    error::Error,
    event::PersistentEvent,
};

use crate::{
    event::codec::Codec,
    projection::dispatch::DispatchEvent,
};

// =================================================================================================
// Recognise
// =================================================================================================

pub trait Recognize {
    fn recognize<C>(codec: &C, event: &PersistentEvent) -> Result<Option<DispatchEvent>, Error>
    where
        C: Codec;
}
