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
    dispatch::Dispatch,
    query::{
        Query,
        Selector,
    },
    recognize::Recognize,
};

// =================================================================================================
// Projection
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(projection), supports(struct_named))]
pub struct Projection {
    ident: Ident,
    #[darling(multiple)]
    select: Vec<Selector>,
}

impl Projection {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Projection {
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

    pub fn selectors(&self) -> &Vec<Selector> {
        &self.select
    }
}

impl Projection {
    fn projection(ident: &Ident) -> TokenStream {
        quote! {
            impl eventric_surface::projection::Projection for #ident {}
        }
    }
}

impl ToTokens for Projection {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Projection::projection(&self.ident));
        tokens.append_all(Dispatch::dispatch(&self.ident, &self.events()));
        tokens.append_all(Query::query(&self.ident, self.selectors()));
        tokens.append_all(Recognize::recognize(&self.ident, &self.events()));
    }
}
