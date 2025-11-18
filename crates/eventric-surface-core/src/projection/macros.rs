#![allow(clippy::needless_continue)]

use darling::{
    Error,
    FromDeriveInput,
};
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
    format_ident,
    quote,
};
use syn::{
    DeriveInput,
    Ident,
};

use crate::projection::query::macros::{
    Query,
    QuerySource,
};

// =================================================================================================
// Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(projection), supports(struct_named))]
pub struct Projection {
    ident: Ident,
    query: Query,
}

impl Projection {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
    }
}

impl Projection {
    fn projection(ident: &Ident) -> TokenStream {
        quote! {
            impl eventric_surface::projection::Projection for #ident {}
        }
    }

    fn update(ident: &Ident, query: &Query) -> TokenStream {
        let event = query.events().into_iter();
        let ident_update_trait = format_ident!("{ident}Update");

        let update_trait = quote! { eventric_surface::projection::Update };

        quote! {
            trait #ident_update_trait: #(#update_trait<#event>)+* {}

            impl #ident_update_trait for #ident {}
        }
    }
}

impl ToTokens for Projection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Projection::projection(&self.ident));
        tokens.append_all(Projection::update(&self.ident, &self.query));
        tokens.append_all(QuerySource::query_source(&self.ident, &self.query));
    }
}
