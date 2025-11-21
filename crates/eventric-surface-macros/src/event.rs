#![allow(clippy::needless_continue)]

pub(crate) mod identifier;
pub(crate) mod tag;

use std::collections::HashMap;

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

use crate::{
    event::{
        identifier::Identifier,
        tag::Tags,
    },
    util::List,
};

// =================================================================================================
// Event
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(event), supports(struct_named))]
pub struct Event {
    ident: Ident,
    #[darling(with = "identifier::parse")]
    identifier: String,
    #[darling(map = "tag::map")]
    tags: Option<HashMap<Ident, List<tag::Tag>>>,
}

impl Event {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
            .and_then(|event| Identifier::validate(&event.identifier.clone(), event))
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
        tokens.append_all(Identifier::identifier(&self.ident, &self.identifier));
        tokens.append_all(Tags::tags(&self.ident, self.tags.as_ref()));
    }
}
