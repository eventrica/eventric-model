use eventric_stream::{
    error::Error,
    stream::select::Selections,
};

use crate::decision::context::Context;

// =================================================================================================
// Selection
// =================================================================================================

pub trait Select: Context {
    fn select(&self, context: &Self::Context) -> Result<Selections, Error>;
}
