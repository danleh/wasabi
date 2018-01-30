extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, DataStruct, Fields, Attribute};

#[proc_macro_derive(ParseWasm, attributes(tag))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = syn::parse(input).unwrap();

    let type_name = input.ident.clone();
    let data = input.data;

    // TODO
    // step 1: tuple struct
    // step 2: struct with fields
    // step 3: enum with unit variants
    // step 4: custom tags on enum variants
    // step 5: enum with struct variants

    match data {
        Data::Struct(DataStruct { fields: Fields::Unnamed(ref unnamed), .. }) => {
            let field_tys = unnamed.unnamed.iter().map(|field| field.clone().ty).collect::<Vec<_>>();
            println!("{:?}", field_tys)
        },
        _ => unimplemented!()
    }

//    println!("{:?}", data);

//    if let DeriveInput { data: syn::Data::Enum(syn::DataEnum{ variants, .. }), .. } = input {
//        println!("{:?}", variants.into_iter().collect::<Vec<syn::Variant>>());
//    }

//    let attrs: Vec<Option<syn::Meta>> = input.attrs.iter().map(Attribute::interpret_meta).collect();
//
//    println!("{:?}", attrs);

    // Build the impl
    let expanded = quote! {
        impl ParseWasm for #type_name {
            fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
                Err(std::io::Error::new(std::io::ErrorKind::InvalidData, ""))
            }
        }
    };

    // Return the generated impl
    expanded.into()
}
