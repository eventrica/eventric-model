pub(crate) mod macros;
pub(crate) mod query;

use eventric_stream::{
    error::Error,
    stream::query::Query,
};

// =================================================================================================
// Projection
// =================================================================================================

// Projection

pub trait Projection: QuerySource {}

// Query Source

pub trait QuerySource {
    fn query(&self) -> Result<Query, Error>;
}
