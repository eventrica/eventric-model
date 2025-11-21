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
pub struct Projections {
    ident: Ident,
    #[darling(multiple, rename = "projection")]
    projections: Vec<Projection>,
}

impl Projections {
    pub fn new(input: &DeriveInput) -> darling::Result<Self> {
        Self::from_derive_input(input)
    }
}

impl Projections {
    pub fn projections(decision_type: &Ident, projections: &[Projection]) -> TokenStream {
        let projections_type = format_ident!("{decision_type}Projections");

        let projection_field_name = projections.iter().map(|p| &p.field_name);
        let projection_field_type = projections.iter().map(|p| &p.field_type);
        let projection_initializer = projections
            .iter()
            .map(|projection| ProjectionInitializer(decision_type, projection));

        let projections_trait = quote! { eventric_surface::decision::Projections };

        quote! {
            impl #projections_trait for #decision_type {
                type Projections = #projections_type;

                fn projections(&self) -> Self::Projections {
                    Self::Projections::new(self)
                }
            }

            #[derive(Debug)]
            pub struct #projections_type {
                #(pub #projection_field_name: #projection_field_type),*
            }

            impl #projections_type {
                fn new(decision: &#decision_type) -> Self {
                    Self {
                        #(#projection_initializer),*
                    }
                }
            }
        }
    }
}

impl ToTokens for Projections {
    #[rustfmt::skip]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(Projections::projections(&self.ident, &self.projections));
    }
}

// -------------------------------------------------------------------------------------------------

// Projection

#[derive(Debug)]
pub struct Projection {
    pub field_name: Ident,
    pub field_type: Path,
    pub initializer: ExprClosure,
}

impl FromMeta for Projection {
    fn from_meta(meta: &Meta) -> darling::Result<Self> {
        let list = meta.require_list()?;
        let input = list.tokens.clone();

        syn::parse2(input).map_err(darling::Error::custom)
    }
}

impl Parse for Projection {
    #[allow(clippy::match_bool, clippy::single_match_else)]
    fn parse(input: ParseStream<'_>) -> syn::Result<Self> {
        let field_type = Path::parse(input)?;
        let field_name = match input.peek(At) {
            true => At::parse(input).and_then(|_| Ident::parse(input))?,
            _ => {
                let segment = field_type.segments.last().expect("ident");
                let ident = segment.ident.to_string();

                format_ident!("{}", AsSnakeCase(ident).to_string())
            }
        };

        let _ = Colon::parse(input)?;

        let initializer = match ExprClosure::parse(input) {
            Ok(expr) => expr,
            _ => Expr::parse(input).and_then(|expr| syn::parse2(quote! { |this| #expr }))?,
        };

        Ok(Self {
            field_name,
            field_type,
            initializer,
        })
    }
}

// Projection Composites

pub struct ProjectionInitializer<'a>(&'a Ident, &'a Projection);

impl ToTokens for ProjectionInitializer<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let ProjectionInitializer(
            decision_type,
            Projection {
                field_name,
                field_type,
                initializer,
            },
        ) = *self;

        let identity_fn = quote! { std::convert::identity };

        tokens.append_all(quote! {
            #field_name: #identity_fn::<fn(&#decision_type) -> #field_type>(#initializer)(decision)
        });
    }
}
