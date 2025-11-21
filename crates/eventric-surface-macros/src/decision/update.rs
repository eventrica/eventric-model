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
};

use crate::decision::projections::Projection;

// =================================================================================================
// Decision
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(projections), supports(struct_named))]
pub struct Update {
    ident: Ident,
    #[darling(multiple, rename = "projection")]
    projections: Vec<Projection>,
}

impl Update {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Update {
    pub fn update(decision_type: &Ident, projections: &[Projection]) -> TokenStream {
        let projection_field_name = projections.iter().map(|p| &p.field_name);

        let codec_trait = quote! { eventric_surface::event::Codec };
        let dispatch_trait = quote! { eventric_surface::projection::Dispatch };
        let error_type = quote! { eventric_stream::error::Error };
        let event_type = quote! { eventric_stream::event::PersistentEvent };
        let recognize_trait = quote! { eventric_surface::projection::Recognize };
        let update_trait = quote! { eventric_surface::decision::Update };

        quote! {
            impl #update_trait for #decision_type {
                fn update<C>(&self, codec: &C, event: &#event_type, projections: &mut Self::Projections) -> Result<(), #error_type>
                where
                    C: #codec_trait,
                {
                    let mut dispatch_event = None;

                    #({
                        if dispatch_event.is_none() {
                            dispatch_event = #recognize_trait::recognize(
                                &projections.#projection_field_name,
                                codec,
                                event,
                            )?;
                        }

                        if let Some(dispatch_event) = dispatch_event {
                            #dispatch_trait::dispatch(
                                &mut projections.#projection_field_name,
                                &dispatch_event,
                            );
                        }
                    })*

                    Ok(())
                }
            }
        }
    }
}

impl ToTokens for Update {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Update::update(&self.ident, &self.projections));
    }
}
