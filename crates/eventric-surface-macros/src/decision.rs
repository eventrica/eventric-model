#![allow(clippy::needless_continue)]

pub(crate) mod projections;
pub(crate) mod update;

use darling::FromDeriveInput;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
    quote,
};
use syn::{
    DeriveInput,
    Ident,
};

use crate::decision::{
    projections::{
        Projection,
        Projections,
    },
    update::Update,
};

// =================================================================================================
// Decision
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(decision), supports(struct_named))]
pub struct Decision {
    ident: Ident,
    #[darling(multiple, rename = "projection")]
    projections: Vec<Projection>,
}

impl Decision {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Decision {
    pub fn decision(ident: &Ident) -> TokenStream {
        quote! {
            impl eventric_surface::decision::Decision for #ident {}
        }
    }
}

impl ToTokens for Decision {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Decision::decision(&self.ident));
        tokens.append_all(Projections::projections(&self.ident, &self.projections));
        tokens.append_all(Update::update(&self.ident, &self.projections));
    }
}
