extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Type, DataStruct, Fields, FieldsUnnamed, Attribute};

#[proc_macro_derive(ParseWasm, attributes(tag))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = syn::parse(input).unwrap();

    let data_name = input.ident.clone();
    let data = input.data;

    let body = match data {
        // tuple struct
        Data::Struct(DataStruct { fields: Fields::Unnamed(FieldsUnnamed { unnamed, .. }), .. }) => {
            let field_tys: Vec<Type> = unnamed.into_iter().map(|field| field.ty).collect();
            quote! {
                #data_name(#(#field_tys::parse(reader)?),*)
            }
        },
        // TODO
        // step 2: struct with fields
        // step 3: enum with unit variants
        // step 4: custom tags on enum variants
        // step 5: enum with struct variants
        _ => unimplemented!()
    };

    // boilerplate of impl that is the same for any data type
    let impl_ = quote! {
        impl ParseWasm for #data_name {
            fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
                Ok(#body)
            }
        }
    };

    impl_.into()
}
