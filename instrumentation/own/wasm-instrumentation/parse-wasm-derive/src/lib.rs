extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Type, Ident, DataStruct, DataEnum, Fields, FieldsUnnamed, FieldsNamed, Attribute, Meta, MetaNameValue, Lit};

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
                #data_name(
                    #( #field_tys::parse(reader)? ),*
                )
            }
        }

        // struct with fields
        Data::Struct(DataStruct { fields: Fields::Named(FieldsNamed { named, .. }), .. }) => {
            let (field_names, field_tys): (Vec<Ident>, Vec<Type>) = named.into_iter().map(|field| (field.ident.unwrap(), field.ty)).unzip();
            quote! {
                #data_name {
                    #( #field_names: #field_tys::parse(reader)? ),*
                }
            }
        }

        // enum with unit variants (all tagged)
        Data::Enum(DataEnum { variants, .. }) => {
            let (variant_attributes, variant_names): (Vec<Vec<Attribute>>, Vec<Ident>) =
                variants.into_iter().map(|variant| (variant.attrs, variant.ident)).unzip();

            let variant_tags: Vec<u8> = variant_attributes.into_iter().map(|attributes| {
                if attributes.len() == 1 {
                    if let Some(Meta::NameValue(MetaNameValue { ident, lit: Lit::Int(lit_int), .. })) = attributes[0].interpret_meta() {
                        if ident.to_string() == "tag" && lit_int.value() < u8::max_value() as u64 {
                            return lit_int.value() as u8
                        }
                    }
                }
                panic!("every enum variant must have exactly one #[tag = <u8>] attribute")
            }).collect();

            // needs to be repeated for quote repetition
            let data_name_repeated = vec![data_name; variant_names.len()];

            quote! {
                match reader.read_byte()? {
                    #( #variant_tags => #data_name_repeated::#variant_names, )*
                    byte => wasm_error(format!("expected tag for #data_name, got 0x{:02x}, ", byte))?
                }
            }
        }

        // TODO
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
