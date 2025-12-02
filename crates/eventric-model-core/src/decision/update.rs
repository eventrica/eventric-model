use eventric_stream::{
    error::Error,
    stream::select::EventMasked,
};

use crate::decision::context::Context;

// =================================================================================================
// Update
// =================================================================================================

pub trait Update: Context {
    fn update(&self, context: &mut Self::Context, event: &EventMasked) -> Result<(), Error>;
}
