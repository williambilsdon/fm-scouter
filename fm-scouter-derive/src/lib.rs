extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(CalcAttrs)]
pub fn sum_attrs(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields_sum = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|field| &field.ident);
                quote! {
                    #(self.#field_names)+*
                }
            }
            _ => panic!("sum_attrs macro only works with named fields"),
        },
        _ => {
            panic!("sum_attrs macro only works with structs");
        }
    };

    let apply_attributes = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|field| &field.ident);
                quote! {
                    #(self.#field_names *= weights.#field_names;)*

                }
            }
            _ => panic!("apply_weights macro only works with named fields"),
        },
        _ => {
            panic!("apply_weights macro only works with structs");
        }
    };

    let expanded = quote! {
        impl #struct_name {
            fn sum_attrs(&self) -> u64 {
                #fields_sum
            }

            fn apply_weights(&mut self, weights: &#struct_name) -> () {
                #apply_attributes
            }
        }
    };

    TokenStream::from(expanded)
}
