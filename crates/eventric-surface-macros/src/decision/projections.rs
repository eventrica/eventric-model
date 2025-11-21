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
    Expr,
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
#[darling(attributes(projections), supports(struct_named))]
pub struct ProjectionsDerive {
    ident: Ident,
    #[darling(multiple)]
    projection: Vec<ProjectionDefinition>,
}

impl ProjectionsDerive {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl ProjectionsDerive {
    pub fn projections(ident: &Ident, projections: &[ProjectionDefinition]) -> TokenStream {
        let projections_type = format_ident!("{ident}Projections");

        let projection_ident = projections.iter().map(|p| &p.ident);
        let projection_path = projections.iter().map(|p| &p.path);
        let projection_initializer = projections
            .iter()
            .map(|projection| IntoProjectionInitializerTokens(ident, projection));

        quote! {
            impl eventric_surface::decision::Projections for #ident {
                type Projections = #projections_type;

                fn projections(&self) -> Self::Projections {
                    Self::Projections::new(self)
                }
            }

            #[derive(Debug)]
            pub struct #projections_type {
                #(pub #projection_ident: #projection_path),*
            }

            impl #projections_type {
                fn new(decision: &#ident) -> Self {
                    Self {
                        #(#projection_initializer),*
                    }
                }
            }
        }
    }
}

impl ToTokens for ProjectionsDerive {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(ProjectionsDerive::projections(&self.ident, &self.projection));
    }
}

// -------------------------------------------------------------------------------------------------

// Projection

#[derive(Debug)]
pub struct ProjectionDefinition {
    expr: ExprClosure,
    ident: Ident,
    path: Path,
}

impl FromMeta for ProjectionDefinition {
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        let list = meta.require_list()?;
        let list = list.tokens.clone();

        syn::parse2(list).map_err(darling::Error::custom)
    }
}

impl Parse for ProjectionDefinition {
    #[allow(clippy::match_bool, clippy::single_match_else)]
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let path = Path::parse(input)?;

        let ident = match input.peek(At) {
            true => At::parse(input).and_then(|_| Ident::parse(input))?,
            _ => {
                let segment = path.segments.last().expect("ident");
                let ident = segment.ident.to_string();

                format_ident!("{}", AsSnakeCase(ident).to_string())
            }
        };

        let _ = Colon::parse(input)?;

        let expr = match ExprClosure::parse(input) {
            Ok(expr) => expr,
            _ => Expr::parse(input).and_then(|expr| syn::parse2(quote! { |this| #expr }))?,
        };

        Ok(Self { expr, ident, path })
    }
}

// Projection Composites

pub struct IntoProjectionInitializerTokens<'a>(&'a Ident, &'a ProjectionDefinition);

impl ToTokens for IntoProjectionInitializerTokens<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let IntoProjectionInitializerTokens(input, projection) = *self;

        let expr = &projection.expr;
        let ident = &projection.ident;
        let path = &projection.path;

        let identity_fn = quote! { std::convert::identity };

        tokens.append_all(quote! {
            #ident: #identity_fn::<fn(&#input) -> #path>(#expr)(decision)
        });
    }
}
