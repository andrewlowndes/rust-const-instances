use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Generics};

#[proc_macro_derive(ToExpr)]
pub fn to_expr_derive(input: TokenStream) -> TokenStream {
    //apply the to_expr method to all of the parameters
    let input = parse_macro_input!(input);
    let DeriveInput {
        ident,
        data,
        generics: Generics {
            params,
            where_clause,
            ..
        },
        ..
    } = &input;

    let obj = match data {
        syn::Data::Struct(obj) => obj,
        _ => unimplemented!("Only structs are currently implemented for ToExpr calls"),
    };

    let fields = obj
        .fields
        .iter()
        .filter_map(|field| field.ident.clone())
        .collect::<Vec<_>>();

    quote!(
        impl<#params> ::to_expr_core::ToExpr for #ident<#params> #where_clause {
            fn to_expr(&self) -> ::proc_macro2::TokenStream {
                #(let #fields = self.#fields.to_expr();)*

                ::quote::quote!(
                    #ident {
                        #(#fields: ##fields,)*
                    }
                )
            }
        }
    )
    .into()
}
