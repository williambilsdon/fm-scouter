extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(ApplyWeights)]
pub fn apply_weights(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;
    let mut fields_len: usize = 0;
    let apply_weights = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                fields_len = fields.named.len();
                let weighted_values = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    quote! { self.#field_name * weights.#field_name / 100 }
                });

                quote! {
                    [#(#weighted_values),*]
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
            fn apply_weights(&mut self, weights: &#struct_name) -> [u64; #fields_len] {
                let weighted_values = #apply_weights;
                weighted_values
            }
        }
    };

    TokenStream::from(expanded)
}
