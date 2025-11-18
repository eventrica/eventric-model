use std::collections::HashMap;

use proc_macro2::{
    Span,
    TokenStream,
};
use quote::{
    ToTokens,
    TokenStreamExt as _,
    format_ident,
    quote,
};
use syn::{
    ExprClosure,
    Ident,
    parse::{
        Parse,
        ParseStream,
    },
};

use crate::macros::List;

// =================================================================================================
// Macros
// =================================================================================================

// Tag

#[derive(Debug)]
pub enum Tag {
    Expr(ExprClosure),
    Ident(Ident),
}

impl Parse for Tag {
    fn parse(stream: ParseStream<'_>) -> syn::Result<Self> {
        if let Ok(mut expr) = ExprClosure::parse(stream) {
            let body = &expr.body;
            let body = syn::parse(quote! { { #body }.into() }.into())?;

            *expr.body = body;

            return Ok(Self::Expr(expr));
        }

        if let Ok(ident) = Ident::parse(stream) {
            return Ok(Self::Ident(ident));
        }

        Err(syn::Error::new(Span::call_site(), "Unexpected Tag Format"))
    }
}

// Tag Composites

pub struct IdentPrefixAndTag<'a>(pub &'a Ident, pub &'a Ident, pub &'a Tag);

impl ToTokens for IdentPrefixAndTag<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IdentPrefixAndTag(ident, prefix, tag) = *self;

        let tag_macro = quote! { eventric_stream::event::tag };
        let identity_fn = quote! { std::convert::identity };
        let cow_type = quote! { std::borrow::Cow };

        match tag {
            Tag::Expr(expr) => tokens.append_all(quote! {
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

pub fn tags_map(tags: Option<HashMap<String, List<Tag>>>) -> Option<HashMap<Ident, List<Tag>>> {
    tags.map(|tags| {
        tags.into_iter()
            .map(|(prefix, tags)| (format_ident!("{prefix}"), tags))
            .collect()
    })
}

pub fn tags_fold<'a>(
    ident: &'a Ident,
    tags: Option<&'a HashMap<Ident, List<Tag>>>,
) -> Vec<IdentPrefixAndTag<'a>> {
    tags.as_ref()
        .map(|tags| {
            tags.iter().fold(Vec::new(), |mut acc, (prefix, tags)| {
                for tag in tags.as_ref() {
                    acc.push(IdentPrefixAndTag(ident, prefix, tag));
                }

                acc
            })
        })
        .unwrap_or_default()
}
