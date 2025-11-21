#![allow(clippy::needless_continue)]

use std::collections::HashMap;

use darling::{
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
        tag::Tag,
    },
    util::List,
};

// =================================================================================================
// Query
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(query), supports(struct_named))]
pub struct Query {
    ident: Ident,
    #[darling(multiple)]
    select: Vec<Selector>,
}

impl Query {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Query {
    #[must_use]
    pub fn query(ident: &Ident, selectors: &[Selector]) -> TokenStream {
        let selector_initialize = selectors
            .iter()
            .map(|selector| SelectorInitialize(ident, selector));

        let query_type = quote! { eventric_stream::stream::query::Query };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::projection::Query for #ident {
                fn query(&self) -> Result<#query_type, #error_type> {
                    #query_type::new([#(#selector_initialize?),*])
                }
            }
        }
    }
}

impl ToTokens for Query {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Query::query(&self.ident, &self.select));
    }
}

// -------------------------------------------------------------------------------------------------

// Selector

#[derive(Debug, FromMeta)]
pub struct Selector {
    pub events: List<Path>,
    #[darling(map = "tag::map")]
    pub filter: Option<HashMap<Ident, List<Tag>>>,
}

// Selector Composites

pub struct SelectorInitialize<'a>(pub &'a Ident, pub &'a Selector);

impl ToTokens for SelectorInitialize<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let SelectorInitialize(ident, selector) = *self;

        let event = selector.events.as_ref();
        let tag = tag::fold(ident, selector.filter.as_ref());

        let selector_type = quote! { eventric_stream::stream::query::Selector };
        let specifier_trait = quote! { eventric_surface::event::Specifier };

        if tag.is_empty() {
            tokens.append_all(quote! {
                #selector_type::specifiers(
                    [#(<#event as #specifier_trait>::specifier()?),*]
                )
            });
        } else {
            tokens.append_all(quote! {
                #selector_type::specifiers_and_tags(
                    [#(<#event as #specifier_trait>::specifier()?),*],
                    [#(#tag?),*]
                )
            });
        }
    }
}
