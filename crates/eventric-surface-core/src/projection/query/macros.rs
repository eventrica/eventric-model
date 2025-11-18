#![allow(clippy::needless_continue)]

use std::collections::{
    HashMap,
    HashSet,
};

use darling::{
    Error,
    FromDeriveInput,
    FromMeta,
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
    Path,
};

use crate::{
    event::{
        tag,
        tag::macros::Tag,
    },
    macros::List,
};

// =================================================================================================
// Query
// =================================================================================================

// Query

#[derive(Debug, FromMeta)]
pub struct Query {
    #[darling(multiple)]
    pub select: Vec<Selector>,
}

impl Query {
    pub fn events(&self) -> HashSet<&Path> {
        self.select.iter().flat_map(|s| s.events.as_ref()).collect()
    }
}

// Query Composites

pub struct IdentAndQuery<'a>(pub &'a Ident, pub &'a Query);

impl ToTokens for IdentAndQuery<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentAndQuery(ident, query) = *self;

        let selector = query
            .select
            .iter()
            .map(|selector| IdentAndSelector(ident, selector));

        let query_type = quote! { eventric_stream::stream::query::Query };

        tokens.append_all(quote! {
            #query_type::new([#(#selector?),*])
        });
    }
}

// -------------------------------------------------------------------------------------------------

// Query Source

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(query_source), supports(struct_named))]
pub struct QuerySource {
    ident: Ident,
    query: Query,
}

impl QuerySource {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
    }
}

impl QuerySource {
    pub fn query_source(ident: &Ident, query: &Query) -> TokenStream {
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

// -------------------------------------------------------------------------------------------------

// Selector

#[derive(Debug, FromMeta)]
pub struct Selector {
    pub events: List<Path>,
    #[darling(map = "tag::macros::tags_map")]
    filter: Option<HashMap<Ident, List<Tag>>>,
}

// Selector Composites

pub struct IdentAndSelector<'a>(pub &'a Ident, pub &'a Selector);

impl ToTokens for IdentAndSelector<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentAndSelector(ident, selector) = *self;

        let event = selector.events.as_ref();
        let tag = tag::macros::tags_fold(ident, selector.filter.as_ref());

        let selector_type = quote! { eventric_stream::stream::query::Selector };
        let specified_trait = quote! { eventric_surface::event::Specified };

        if tag.is_empty() {
            tokens.append_all(quote! {
                #selector_type::specifiers(
                    [#(<#event as #specified_trait>::specifier()?),*]
                )
            });
        } else {
            tokens.append_all(quote! {
                #selector_type::specifiers_and_tags(
                    [#(<#event as #specified_trait>::specifier()?),*],
                    [#(#tag?),*]
                )
            });
        }
    }
}
