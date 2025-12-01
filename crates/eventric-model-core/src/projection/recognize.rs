use std::sync::Arc;

use eventric_stream::{
    error::Error,
    stream::select::EventMasked,
};

use crate::{
    event::codec::Codec,
    projection::dispatch::DispatchEvent,
};

// =================================================================================================
// Recognise
// =================================================================================================

pub trait Recognize {
    fn recognize<C>(
        &self,
        codec: Arc<C>,
        event: &EventMasked,
    ) -> Result<Option<DispatchEvent>, Error>
    where
        C: Codec;
}
