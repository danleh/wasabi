extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::{DeriveInput, Data, Type, Ident, DataStruct, DataEnum, Fields, FieldsUnnamed, FieldsNamed, Meta, MetaNameValue, Lit, TypePath, Path, PathSegment, PathArguments, Attribute};
use quote::Tokens;

#[proc_macro_derive(Wasm, attributes(tag))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let data_name = input.ident;

    let impl_body = match input.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let create = recurse_into_fields(data_name, fields);

            // (optionally:) check that tag matches
            if !input.attrs.is_empty() {
                let tag = attributes_to_tag_value(&input.attrs);
                quote!({
                    let byte = u8::parse(reader)?;
                    if byte != #tag {
                        return Self::error(format!("expected tag for {}, got 0x{:02x}", stringify!(#data_name), byte));
                    }
                    #create
                })
            } else {
                create
            }
        }
        Data::Enum(DataEnum { variants, .. }) => {
            let variant_tags: Vec<u8> = variants.iter()
                .map(|variant| attributes_to_tag_value(&variant.attrs))
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
                    byte => Self::error(format!("expected tag for {}, got 0x{:02x}", stringify!(#data_name), byte))?
                }
            }
        }
        _ => unimplemented!("can only derive(Wasm) for structs and enums")
    };

    // boilerplate of impl that is the same for any data type
    let impl_ = quote! {
        impl Wasm for #data_name {
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
                .map(|field| remove_type_arguments(field.ty))
                .collect();
            quote! {
                #name(
                    #( #field_tys::parse(reader)? ),*
                )
            }
        }
        Fields::Named(FieldsNamed { named, .. }) => {
            let (field_names, field_tys): (Vec<Ident>, Vec<Type>) = named.into_iter()
                .map(|field| (field.ident.unwrap(), remove_type_arguments(field.ty)))
                .unzip();
            quote! {
                #name {
                    #( #field_names: #field_tys::parse(reader)? ),*
                }
            }
        }
    }
}

// so that a field: Vec<T> is parsed by Vec::parse() not Vec<T>::parse() (which is not valid syntax)
fn remove_type_arguments(mut ty: Type) -> Type {
    if let Type::Path(TypePath { path: Path { ref mut segments, .. }, .. }) = ty {
        *segments = segments.into_iter().map(|segment| {
            PathSegment {
                arguments: PathArguments::None,
                ..*segment
            }
        }).collect();
    }
    ty
}

fn attributes_to_tag_value(attributes: &[Attribute]) -> u8 {
    if attributes.len() == 1 {
        if let Some(Meta::NameValue(MetaNameValue { ident, lit: Lit::Int(lit_int), .. })) = attributes[0].interpret_meta() {
            if ident.to_string() == "tag" && lit_int.value() < u8::max_value() as u64 {
                return lit_int.value() as u8;
            }
        }
    }
    panic!("structs can have / every enum variant must have exactly one #[tag = <u8>] attribute")
}