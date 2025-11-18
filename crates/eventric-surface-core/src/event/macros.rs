#![allow(clippy::needless_continue)]

use std::collections::HashMap;

use darling::{
    Error,
    FromDeriveInput,
};
use eventric_stream::event;
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

use crate::{
    event::{
        identifier,
        tag,
        tag::macros::Tag,
    },
    macros::List,
};

// =================================================================================================
// Macros
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(event), supports(struct_named))]
pub(crate) struct Event {
    ident: Ident,
    #[darling(with = "identifier::macros::identifier_parse")]
    identifier: String,
    #[darling(map = "tag::macros::tags_map")]
    tags: Option<HashMap<Ident, List<Tag>>>,
}

impl Event {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
            .and_then(|event| Identified::validate(&event.identifier.clone(), event))
    }
}

impl Event {
    fn event(ident: &Ident) -> TokenStream {
        quote! {
            impl eventric_surface::event::Event for #ident {}
        }
    }
}

impl ToTokens for Event {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Event::event(&self.ident));
        tokens.append_all(Identified::identifier(&self.ident, &self.identifier));
        tokens.append_all(Tagged::tags(&self.ident, self.tags.as_ref()));
    }
}

// -------------------------------------------------------------------------------------------------

// Identified

#[derive(Debug, FromDeriveInput)]
#[darling(
    attributes(identified),
    forward_attrs(allow, doc),
    supports(struct_named)
)]
pub(crate) struct Identified {
    ident: Ident,
    #[darling(with = "identifier::macros::identifier_parse")]
    identifier: String,
}

impl Identified {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
            .and_then(|identifier| Identified::validate(&identifier.identifier.clone(), identifier))
    }
}

impl Identified {
    fn identifier(ident: &Ident, identifier: &str) -> TokenStream {
        let cell_type = quote! {std::sync::OnceLock };
        let identifier_type = quote! { eventric_stream::event::Identifier };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::event::Identified for #ident {
                fn identifier() -> Result<&'static #identifier_type, #error_type> {
                    static IDENTIFIER: #cell_type<#identifier_type> = #cell_type::new();

                    IDENTIFIER.get_or_try_init(|| #identifier_type::new(#identifier))
                }
            }
        }
    }
}

impl Identified {
    fn validate<T>(ident: &str, ok: T) -> Result<T, Error> {
        Self::validate_identifier(ident)?;

        Ok(ok)
    }

    fn validate_identifier(ident: &str) -> Result<(), Error> {
        event::Identifier::new(ident)
            .map(|_| ())
            .map_err(Error::custom)
    }
}

impl ToTokens for Identified {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Identified::identifier(&self.ident, &self.identifier));
    }
}

// -------------------------------------------------------------------------------------------------

// Tagged

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tagged), forward_attrs(allow, doc), supports(struct_named))]
pub(crate) struct Tagged {
    ident: Ident,
    #[darling(map = "tag::macros::tags_map")]
    tags: Option<HashMap<Ident, List<Tag>>>,
}

impl Tagged {
    pub fn new(input: &DeriveInput) -> Result<Self, Error> {
        Self::from_derive_input(input)
    }
}

impl Tagged {
    fn tags(ident: &Ident, tags: Option<&HashMap<Ident, List<Tag>>>) -> TokenStream {
        let tag = tag::macros::tags_fold(ident, tags);
        let tag_count = tag.len();

        let tag_type = quote! { eventric_stream::event::Tag };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::event::Tagged for #ident {
                fn tags(&self) -> Result<Vec<#tag_type>, #error_type> {
                    let mut tags = Vec::with_capacity(#tag_count);

                  #(tags.push(#tag?);)*

                    Ok(tags)
                }
            }
        }
    }
}

impl ToTokens for Tagged {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Tagged::tags(&self.ident, self.tags.as_ref()));
    }
}
