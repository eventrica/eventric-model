#![allow(clippy::needless_continue)]

use std::collections::HashMap;

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
    Expr,
    ExprClosure,
    Ident,
    parse::{
        Parse,
        ParseStream,
    },
};

use crate::util::List;

// =================================================================================================
// Tag
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(tags), supports(struct_named))]
pub struct Tags {
    ident: Ident,
    #[darling(map = "map")]
    tags: Option<HashMap<Ident, List<Tag>>>,
}

impl Tags {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Tags {
    #[must_use]
    pub fn tags(ident: &Ident, tags: Option<&HashMap<Ident, List<Tag>>>) -> TokenStream {
        let tag = fold(ident, tags);
        let tag_count = tag.len();

        let tag_type = quote! { eventric_stream::event::Tag };
        let error_type = quote! { eventric_stream::error::Error };

        quote! {
            impl eventric_surface::event::Tags for #ident {
                fn tags(&self) -> Result<Vec<#tag_type>, #error_type> {
                    let mut tags = Vec::with_capacity(#tag_count);

                  #(tags.push(#tag?);)*

                    Ok(tags)
                }
            }
        }
    }
}

impl ToTokens for Tags {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Tags::tags(&self.ident, self.tags.as_ref()));
    }
}

// -------------------------------------------------------------------------------------------------

// Tag

#[derive(Debug)]
pub enum Tag {
    ExprClosure(ExprClosure),
    Ident(Ident),
}

impl Parse for Tag {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        if let Ok(mut expr) = ExprClosure::parse(input) {
            let body = &expr.body;
            let body = syn::parse2(quote! { { #body }.into() })?;

            *expr.body = body;

            return Ok(Self::ExprClosure(expr));
        }

        if let Ok(ident) = Ident::parse(input) {
            return Ok(Self::Ident(ident));
        }

        Expr::parse(input).and_then(|expr| {
            Ok(Self::ExprClosure(syn::parse2(
                quote! { |this| { #expr }.into() },
            )?))
        })
    }
}

// Tag Composites

pub struct TagInitialize<'a>(pub &'a Ident, pub &'a Ident, pub &'a Tag);

impl ToTokens for TagInitialize<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let TagInitialize(ident, prefix, tag) = *self;

        let tag_macro = quote! { eventric_stream::event::tag };
        let identity_fn = quote! { std::convert::identity };
        let cow_type = quote! { std::borrow::Cow };

        match tag {
            Tag::ExprClosure(expr) => tokens.append_all(quote! {
                #tag_macro!(#prefix, #identity_fn::<for<'a> fn(&'a #ident) -> #cow_type<'a, _>>(#expr)(&self))
            }),
            Tag::Ident(ident) => tokens.append_all(quote! {
                #tag_macro!(#prefix, &self.#ident)
            }),
        }
    }
}

// -------------------------------------------------------------------------------------------------

// Tag Functions

pub fn map(tags: Option<HashMap<String, List<Tag>>>) -> Option<HashMap<Ident, List<Tag>>> {
    tags.map(|tags| {
        tags.into_iter()
            .map(|(prefix, tags)| (format_ident!("{prefix}"), tags))
            .collect()
    })
}

pub fn fold<'a>(
    ident: &'a Ident,
    tags: Option<&'a HashMap<Ident, List<Tag>>>,
) -> Vec<TagInitialize<'a>> {
    tags.as_ref()
        .map(|tags| {
            tags.iter().fold(Vec::new(), |mut acc, (prefix, tags)| {
                for tag in tags.as_ref() {
                    acc.push(TagInitialize(ident, prefix, tag));
                }

                acc
            })
        })
        .unwrap_or_default()
}
