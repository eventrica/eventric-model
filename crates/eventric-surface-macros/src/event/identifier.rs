#![allow(clippy::needless_continue)]

use darling::FromDeriveInput;
use eventric_stream::event;
use proc_macro2::{
    TokenStream,
    TokenTree,
};
use quote::{
    ToTokens,
    TokenStreamExt as _,
    quote,
};
use syn::{
    DeriveInput,
    Ident,
    Meta,
};

// =================================================================================================
// Identifier
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(identifier), supports(struct_named))]
pub struct Identifier {
    ident: Ident,
    #[darling(with = "parse")]
    identifier: String,
}

impl Identifier {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
            .and_then(|identifier| Self::validate(&identifier.identifier.clone(), identifier))
    }
}

impl Identifier {
    #[must_use]
    pub fn identifier(event_type: &Ident, identifier: &str) -> TokenStream {
        let cell_type = quote! {std::sync::OnceLock };
        let error_type = quote! { eventric_stream::error::Error };
        let identifier_trait = quote! { eventric_surface::event::Identifier };
        let identifier_type = quote! { eventric_stream::event::Identifier };

        quote! {
            impl #identifier_trait for #event_type {
                fn identifier() -> Result<&'static #identifier_type, #error_type> {
                    static IDENTIFIER: #cell_type<#identifier_type> = #cell_type::new();

                    IDENTIFIER.get_or_try_init(|| #identifier_type::new(#identifier))
                }
            }
        }
    }
}

impl Identifier {
    pub fn validate<T>(ident: &str, ok: T) -> darling::Result<T> {
        Self::validate_identifier(ident).map(|()| ok)
    }

    fn validate_identifier(ident: &str) -> darling::Result<()> {
        event::Identifier::new(ident)
            .map(|_| ())
            .map_err(darling::Error::custom)
    }
}

impl ToTokens for Identifier {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Identifier::identifier(&self.ident, &self.identifier));
    }
}

// -------------------------------------------------------------------------------------------------

// Identifier Functions

pub fn parse(meta: &Meta) -> darling::Result<String> {
    let identifier = meta.require_list()?;
    let identifier = identifier.tokens.clone().into_iter().collect::<Vec<_>>();

    match &identifier[..] {
        [TokenTree::Ident(ident)] => Ok(ident.to_string()),
        _ => Err(darling::Error::unsupported_shape("identifier")),
    }
}
