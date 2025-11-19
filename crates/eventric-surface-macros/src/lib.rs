//! See the `eventric-surface` crate for full documentation, including
//! crate-level documentation.

#![allow(clippy::multiple_crate_versions)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::missing_safety_doc)]
#![allow(missing_docs)]

pub(crate) mod event;
pub(crate) mod projection;
pub(crate) mod util;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

// =================================================================================================
// Eventric Surface Macro
// =================================================================================================

macro_rules! emit_impl_or_error {
    ($e:expr) => {
        match $e {
            Ok(val) => val.into_token_stream(),
            Err(err) => err.write_errors(),
        }
    };
}

// Event

#[proc_macro_derive(Event, attributes(event))]
pub fn event(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(event::EventDerive::new(&parse_macro_input!(input))).into()
}

#[rustfmt::skip]
#[proc_macro_derive(Identifier, attributes(identifier))]
pub fn identifier(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(event::identifier::IdentifierDerive::new(&parse_macro_input!(input))).into()
}

#[rustfmt::skip]
#[proc_macro_derive(Tags, attributes(tags))]
pub fn tags(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(event::tag::TagsDerive::new(&parse_macro_input!(input))).into()
}

// Projection

#[rustfmt::skip]
#[proc_macro_derive(Projection, attributes(projection))]
pub fn projection(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(projection::ProjectionDerive::new(&parse_macro_input!(input))).into()
}

#[rustfmt::skip]
#[proc_macro_derive(Dispatch, attributes(dispatch))]
pub fn dispatch(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(projection::dispatch::DispatchDerive::new(&parse_macro_input!(input))).into()
}

#[rustfmt::skip]
#[proc_macro_derive(Query, attributes(query))]
pub fn query(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(projection::query::QueryDerive::new(&parse_macro_input!(input))).into()
}

#[rustfmt::skip]
#[proc_macro_derive(Recognize, attributes(recognize))]
pub fn recognize(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(projection::recognize::RecognizeDerive::new(&parse_macro_input!(input))).into()
}
