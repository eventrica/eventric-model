#![allow(clippy::needless_continue)]

pub(crate) mod dispatch;
pub(crate) mod query;
pub(crate) mod recognize;

use std::collections::HashSet;

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
    Path,
};

use crate::projection::{
    dispatch::DispatchDerive,
    query::{
        QueryDerive,
        SelectorDefinition,
    },
    recognize::RecognizeDerive,
};

// =================================================================================================
// Projection
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(projection), supports(struct_named))]
pub struct ProjectionDerive {
    ident: Ident,
    #[darling(multiple)]
    select: Vec<SelectorDefinition>,
}

impl ProjectionDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl ProjectionDerive {
    #[must_use]
    pub fn events(&self) -> Vec<Path> {
        self.select
            .iter()
            .flat_map(|selector| selector.events.as_ref())
            .cloned()
            .collect::<HashSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn selectors(&self) -> &Vec<SelectorDefinition> {
        &self.select
    }
}

impl ProjectionDerive {
    fn projection(ident: &Ident) -> TokenStream {
        quote! {
            impl eventric_surface::projection::Projection for #ident {}
        }
    }
}

impl ToTokens for ProjectionDerive {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(ProjectionDerive::projection(&self.ident));
        tokens.append_all(DispatchDerive::dispatch(&self.ident, &self.events()));
        tokens.append_all(QueryDerive::query(&self.ident, self.selectors()));
        tokens.append_all(RecognizeDerive::recognize(&self.ident, &self.events()));
    }
}
