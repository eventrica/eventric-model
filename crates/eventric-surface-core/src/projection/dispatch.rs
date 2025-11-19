use std::any::Any;

use darling::FromDeriveInput;
use syn::{
    Ident,
    Path,
};

use crate::macros::List;

// =================================================================================================
// Dispatch
// =================================================================================================

pub trait Dispatch {
    fn dispatch(&mut self, event: &Box<dyn Any>);
}

// =================================================================================================
// Dispatch Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dispatch), supports(struct_named))]
pub struct DispatchDerive {
    ident: Ident,
    events: List<Path>,
}
