#![allow(clippy::needless_continue)]

use std::collections::HashMap;

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

use crate::{
    event::{
        identifier,
        identifier::macros::Identified,
        tag,
        tag::macros::{
            Tag,
            Tagged,
        },
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
