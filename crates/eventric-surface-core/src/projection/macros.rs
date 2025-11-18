#![allow(clippy::needless_continue)]

use darling::{
    Error,
    FromDeriveInput,
};
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

use crate::projection::query::macros::{
    IdentAndQuery,
    Query,
};

// =================================================================================================
// Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(projection), supports(struct_named))]
pub(crate) struct Projection {
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
}

impl ToTokens for Projection {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Projection::projection(&self.ident));
        tokens.append_all(QuerySource::query_source(&self.ident, &self.query));
    }
}

// -------------------------------------------------------------------------------------------------

// Query Source

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(query_source), supports(struct_named))]
pub(crate) struct QuerySource {
    ident: Ident,
    query: Query,
}

impl QuerySource {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
    }
}

impl QuerySource {
    fn query_source(ident: &Ident, query: &Query) -> TokenStream {
        let query = IdentAndQuery(ident, query);

        let query_type = quote! { eventric_stream::stream::query::Query };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::projection::QuerySource for #ident {
                fn query(&self) -> Result<#query_type, #error_type> {
                    #query
                }
            }
        }
    }
}

impl ToTokens for QuerySource {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(QuerySource::query_source(&self.ident, &self.query));
    }
}
