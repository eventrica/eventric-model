#![allow(clippy::needless_continue)]

pub(crate) mod projections;

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
};
use syn::{
    DeriveInput,
    Ident,
};

use crate::decision::projections::{
    ProjectionDefinition,
    ProjectionsDerive,
};

// =================================================================================================
// Decision
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(decision), supports(struct_named))]
pub struct DecisionDerive {
    ident: Ident,
    #[darling(multiple)]
    projection: Vec<ProjectionDefinition>,
}

impl DecisionDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl ToTokens for DecisionDerive {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(ProjectionsDerive::projections(&self.ident, &self.projection));
    }
}
