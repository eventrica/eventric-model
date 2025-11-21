#![allow(clippy::needless_continue)]

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

use crate::util::List;

// =================================================================================================
// Recognize
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(recognize), supports(struct_named))]
pub struct Recognize {
    ident: Ident,
    events: List<Path>,
}

impl Recognize {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Recognize {
    #[must_use]
    pub fn recognize(ident: &Ident, event: &[Path]) -> TokenStream {
        let recognize_match_arm = event.iter().map(RecognizeMatchArm);

        let codec_trait = quote! {eventric_surface::event::Codec };
        let dispatch_event_type = quote! { eventric_surface::projection::DispatchEvent };
        let error_type = quote! { eventric_stream::error::Error };
        let persistent_event_type = quote! { eventric_stream::event::PersistentEvent };
        let recognize_trait = quote! { eventric_surface::projection::Recognize };

        quote! {
            impl #recognize_trait for #ident {
                fn recognize<C>(&self, codec: &C, event: &#persistent_event_type) -> Result<Option<#dispatch_event_type>, #error_type>
                where
                    C: #codec_trait,
                {
                    let event = match event {
                        #(#recognize_match_arm),*
                        _ => None,
                    };

                    Ok(event)
                }
            }
        }
    }
}

impl ToTokens for Recognize {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Self::recognize(&self.ident, self.events.as_ref()));
    }
}

// Recognize Composites

pub struct RecognizeMatchArm<'a>(&'a Path);

impl ToTokens for RecognizeMatchArm<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let RecognizeMatchArm(event) = *self;

        let identifier_trait = quote! { eventric_surface::event::Identifier };
        let dispatch_event_type = quote! { eventric_surface::projection::DispatchEvent };

        tokens.append_all(quote! {
            _ if event.identifier() == <#event as #identifier_trait>::identifier()? => {
                Some(#dispatch_event_type::from_persistent_event::<C, #event>(codec, event)?)
            }
        });
    }
}
