#![allow(clippy::needless_continue)]

use darling::{
    FromDeriveInput,
    FromMeta,
};
use heck::AsSnakeCase;
use proc_macro2::TokenStream;
use quote::{
    ToTokens,
    TokenStreamExt as _,
    format_ident,
    quote,
};
use syn::{
    DeriveInput,
    ExprClosure,
    Ident,
    Meta,
    Path,
    parse::{
        Parse,
        ParseStream,
    },
    token::{
        At,
        Colon,
    },
};

// =================================================================================================
// Decision
// =================================================================================================

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(decision), supports(struct_named))]
pub struct DecisionDerive {
    ident: Ident,
    #[darling(multiple)]
    projection: Vec<ProjectionDefinition>,
}

impl DecisionDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl DecisionDerive {
    pub fn projection_type(ident: &Ident, projections: &[ProjectionDefinition]) -> TokenStream {
        let projection_type = format_ident!("{ident}Projections");
        let projection_expr = projections.iter().map(ProjectionDefinition::expr);

        let projection_ident = projections
            .iter()
            .map(ProjectionDefinition::ident)
            .collect::<Vec<_>>();

        let projection_path = projections
            .iter()
            .map(ProjectionDefinition::path)
            .collect::<Vec<_>>();

        let identity_fn = quote! { std::convert::identity };

        quote! {
            struct #projection_type {
                #(pub #projection_ident: #projection_path),*
            }

            impl #projection_type {
                fn new(decision: &#ident) -> Self {
                    Self {
                        #(#projection_ident: #identity_fn::<fn(&#ident) -> #projection_path>(#projection_expr)(decision)),*
                    }
                }
            }
        }
    }
}

impl ToTokens for DecisionDerive {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // panic!("self: {self:#?}");

        tokens.append_all(DecisionDerive::projection_type(&self.ident, &self.projection));
    }
}

#[derive(Debug)]
pub struct ProjectionDefinition {
    expr: ExprClosure,
    ident: Option<Ident>,
    path: Path,
}

impl ProjectionDefinition {
    pub fn expr(&self) -> &ExprClosure {
        &self.expr
    }

    pub fn ident(&self) -> Ident {
        self.ident.clone().unwrap_or_else(|| {
            let segment = self.path.segments.last().expect("last ident");
            let ident = format!("{}", AsSnakeCase(segment.ident.to_string()));
            let ident = format_ident!("{ident}");

            ident
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl FromMeta for ProjectionDefinition {
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        let list = meta.require_list()?;
        let list = list.tokens.clone();

        syn::parse2::<ProjectionDefinition>(list).map_err(darling::Error::custom)
    }
}

impl Parse for ProjectionDefinition {
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path = Path::parse(input)?;

        let ident = if input.peek(At) {
            let _ = At::parse(input)?;
            let ident = Ident::parse(input)?;

            Some(ident)
        } else {
            None
        };

        let _ = Colon::parse(input)?;
        let expr = ExprClosure::parse(input)?;

        Ok(Self { expr, ident, path })
    }
}
