//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

use eventric_stream::{
    error::Error,
    stream::query,
};

// =================================================================================================
// Query
// =================================================================================================

pub trait Query {
    fn query(&self) -> Result<query::Query, Error>;
}
