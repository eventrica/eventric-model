#![allow(clippy::needless_continue)]

use darling::FromDeriveInput;
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
    Path,
};

use crate::util::List;

// =================================================================================================
// Dispatch
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(dispatch), supports(struct_named))]
pub struct Dispatch {
    ident: Ident,
    events: List<Path>,
}

impl Dispatch {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Dispatch {
    #[must_use]
    pub fn dispatch(ident: &Ident, event: &Vec<Path>) -> TokenStream {
        let dispatch_trait = format_ident!("{ident}Dispatch");

        let event_type = quote! { eventric_surface::projection::DispatchEvent };
        let update_trait = quote! { eventric_surface::projection::Update };

        quote! {
            pub trait #dispatch_trait: #(#update_trait<#event>)+* {}

            impl #dispatch_trait for #ident {}
            impl eventric_surface::projection::Dispatch for #ident {
                fn dispatch(&mut self, event: &#event_type) {
                    match event {
                      #(_ if let Some(event) = event.as_update_event::<#event>() => self.update(event),)*
                        _ => {}
                    }
                }
            }
        }
    }
}

impl ToTokens for Dispatch {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Self::dispatch(&self.ident, self.events.as_ref()));
    }
}
