extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(SumAttrs)]
pub fn sum_attrs(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // Extract the name of the struct
    let struct_name = &input.ident;

    // Extract the fields from the struct
    let fields_sum = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields) => {
                let field_names = fields.named.iter().map(|field| &field.ident);
                quote! {
                    #(self.#field_names)+*
                }
            }
            Fields::Unnamed(fields) => {
                let field_indices = (0..fields.unnamed.len()).map(|i| syn::Index::from(i));
                quote! {
                    #(self.#field_indices)+*
                }
            }
            Fields::Unit => {
                quote! {}
            }
        },
        _ => {
            panic!("sum_attrs macro only works with structs");
        }
    };

    // Generate the implementation of the SumFields trait
    let expanded = quote! {
        impl #struct_name {
            fn sum_attrs(&self) -> u64 {
                #fields_sum
            }
        }
    };

    // Convert the generated code into a TokenStream and return it
    TokenStream::from(expanded)
}
