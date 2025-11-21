//! See the `eventric-surface` crate for full documentation, including
//! crate-level documentation.

#![allow(clippy::multiple_crate_versions)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(clippy::missing_safety_doc)]
#![allow(missing_docs)]

pub(crate) mod decision;
pub(crate) mod event;
pub(crate) mod projection;
pub(crate) mod util;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::parse_macro_input;

use crate::{
    decision::{
        Decision,
        projections::Projections,
        update::Update,
    },
    event::{
        Event,
        identifier::Identifier,
        tag::Tags,
    },
    projection::{
        Projection,
        dispatch::Dispatch,
        query::Query,
        recognize::Recognize,
    },
};

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
    emit_impl_or_error!(Event::new(&parse_macro_input!(input))).into()
}

#[proc_macro_derive(Identifier, attributes(identifier))]
pub fn identifier(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Identifier::new(&parse_macro_input!(input))).into()
}

#[proc_macro_derive(Tags, attributes(tags))]
pub fn tags(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Tags::new(&parse_macro_input!(input))).into()
}

// Decision

#[proc_macro_derive(Decision, attributes(decision))]
pub fn decision(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Decision::new(&parse_macro_input!(input))).into()
}

#[proc_macro_derive(Projections, attributes(projections))]
pub fn projections(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Projections::new(&parse_macro_input!(input))).into()
}

#[proc_macro_derive(Update, attributes(update))]
pub fn update(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Update::new(&parse_macro_input!(input))).into()
}

// Projection

#[proc_macro_derive(Projection, attributes(projection))]
pub fn projection(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Projection::new(&parse_macro_input!(input))).into()
}

#[proc_macro_derive(Dispatch, attributes(dispatch))]
pub fn dispatch(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Dispatch::new(&parse_macro_input!(input))).into()
}

#[proc_macro_derive(Query, attributes(query))]
pub fn query(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Query::new(&parse_macro_input!(input))).into()
}

#[proc_macro_derive(Recognize, attributes(recognize))]
pub fn recognize(input: TokenStream) -> TokenStream {
    emit_impl_or_error!(Recognize::new(&parse_macro_input!(input))).into()
}
