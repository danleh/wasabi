extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Type, Ident, DataStruct, DataEnum, Fields, FieldsUnnamed, FieldsNamed, Meta, MetaNameValue, Lit};
use quote::Tokens;

#[proc_macro_derive(ParseWasm, attributes(tag))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let data_name = input.ident;

    let impl_body = match input.data {
        Data::Struct(DataStruct { fields, .. }) => recurse_into_fields(data_name, fields),
        Data::Enum(DataEnum { variants, .. }) => {
            let variant_tags: Vec<u8> = variants.iter()
                .map(|variant| {
                    if variant.attrs.len() == 1 {
                        if let Some(Meta::NameValue(MetaNameValue { ident, lit: Lit::Int(lit_int), .. })) = variant.attrs[0].interpret_meta() {
                            if ident.to_string() == "tag" && lit_int.value() < u8::max_value() as u64 {
                                return lit_int.value() as u8;
                            }
                        }
                    }
                    panic!("every enum variant must have exactly one #[tag = <u8>] attribute")
                })
                .collect();

            let variant_create: Vec<Tokens> = variants.into_iter()
                .map(|variant| recurse_into_fields(variant.ident, variant.fields))
                .collect();

            // needs to be repeated for quote repetition with #( ... )*
            let data_name_repeated = vec![data_name; variant_create.len()];

            // match enum variants by tag
            quote! {
                match u8::parse(reader)? {
                    #( #variant_tags => #data_name_repeated::#variant_create, )*
                    byte => wasm_error(format!("expected tag for #data_name, got 0x{:02x}, ", byte))?
                }
            }
        }
        _ => unimplemented!("can only derive(ParseWasm) for structs and enums")
    };

    // boilerplate of impl that is the same for any data type
    let impl_ = quote! {
        impl ParseWasm for #data_name {
            fn parse<R: io::Read>(reader: &mut R) -> io::Result<Self> {
                Ok(#impl_body)
            }
        }
    };

    impl_.into()
}

// extract handling of fields since it is the same for structs and enums
fn recurse_into_fields(name: Ident, fields: Fields) -> Tokens {
    match fields {
        Fields::Unit => quote!(#name),
        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
            let field_tys: Vec<Type> = unnamed.into_iter()
                .map(|field| field.ty)
                .collect();
            quote! {
                #name(
                    #( #field_tys::parse(reader)? ),*
                )
            }
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let (field_names, field_tys): (Vec<Ident>, Vec<Type>) = named.into_iter()
                .map(|field| (field.ident.unwrap(), field.ty))
                .unzip();
            quote! {
                #name {
                    #( #field_names: #field_tys::parse(reader)? ),*
                }
            }
        }
    }
}
