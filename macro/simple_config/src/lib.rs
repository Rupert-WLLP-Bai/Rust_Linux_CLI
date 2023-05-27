use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields, Ident};

#[proc_macro_derive(Config)]
pub fn derive_config(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let data = match input.data {
        Data::Struct(data) => data,
        _ => panic!("Only structs are supported"),
    };

    let fields = match data.fields {
        Fields::Named(fields) => fields,
        _ => panic!("Only named fields are supported"),
    };

    let field_names: Vec<Ident> = fields
        .named
        .into_iter()
        .map(|f| f.ident.expect("Field must have a name"))
        .collect();

    let expanded = quote! {
        impl Config for #name {
            fn parse_config(config: &std::collections::HashMap<String, String>) -> Result<Self, String> {
                let mut result = Self {
                    #(
                        #field_names: config
                            .get(stringify!(#field_names))
                            .ok_or_else(|| format!("Missing field: {}", stringify!(#field_names)))?
                            .parse()
                            .map_err(|_| format!("Failed to parse field: {}", stringify!(#field_names)))?
                    ),*
                };

                Ok(result)
            }
        }
    };

    expanded.into()
}
