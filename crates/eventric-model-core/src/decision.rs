//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

pub(crate) mod context;
pub(crate) mod execute;
pub(crate) mod select;
pub(crate) mod update;

// =================================================================================================
// Decision
// =================================================================================================

pub trait Decision: Execute + Context + Select + Update {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    context::Context,
    execute::{
        Events,
        Execute,
    },
    select::Select,
    update::Update,
};
