//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

pub mod query;

pub(crate) mod dispatch;
pub(crate) mod recognize;
pub(crate) mod update;

use crate::projection::query::Query;

// =================================================================================================
// Projection
// =================================================================================================

// Projection

pub trait Projection: Dispatch + Recognize + Query {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    dispatch::{
        Dispatch,
        DispatchEvent,
    },
    recognize::Recognize,
    update::{
        Update,
        UpdateEvent,
    },
};
