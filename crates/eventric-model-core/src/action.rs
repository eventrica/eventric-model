//! See the `eventric-surface` crate for full documentation, including
//! module-level documentation.

pub(crate) mod act;
pub(crate) mod context;
pub(crate) mod select;
pub(crate) mod update;

// =================================================================================================
// Action
// =================================================================================================

pub trait Action: Act + Context + Select + Update {}

// -------------------------------------------------------------------------------------------------

// Re-Exports

pub use self::{
    act::Act,
    context::{
        Context,
        Events,
    },
    select::Select,
    update::Update,
};
