//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

pub(crate) mod execute;
pub(crate) mod projections;
pub(crate) mod select;
pub(crate) mod update;

// =================================================================================================
// Decision
// =================================================================================================

pub trait Decision: Execute + Projections + Select + Update {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    execute::{
        Execute,
        Events,
    },
    projections::Projections,
    select::Select,
    update::Update,
};
