#![recursion_limit = "128"]

// TODO update to Rust 2018
// TODO update to syn and quote >= 1.0

extern crate proc_macro;
#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use proc_macro::TokenStream;

use quote::Tokens;
use syn::{Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, Ident, Lit, Meta, MetaNameValue, Path, PathArguments, PathSegment, Type, TypePath, Variant};

#[proc_macro_derive(WasmBinary, attributes(tag))]
pub fn derive_wasm(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let data_name = input.ident;

    let decode_expr = match input.data {
        Data::Struct(DataStruct { ref fields, .. }) => {
            // Structs can also be annotated with a tag attribute, which means that the struct
            // _must_ be preceded by the given byte.
            let tag_constant: Option<u8> = attributes_to_tag(&input.attrs);

            let check_tag = tag_constant.map(|tag_constant| quote! {
                let offset_before = state.current_offset;
                let tag = u8::decode(reader, state).set_err_elem::<Self>()?;
                if tag != #tag_constant {
                    return Err(crate::error::Error::invalid_tag::<#data_name>(offset_before, tag));
                }
            });
            let decode_fields = decode_fields(&parse_quote!(#data_name), &fields);

            quote!({
                #( #check_tag )*
                #decode_fields
            })
        }
        Data::Enum(DataEnum { ref variants, .. }) => {
            let decode_variants = variants.iter().map(|variant| decode_variant(data_name, variant));

            quote!({
                let offset_before = state.current_offset;
                let tag = u8::decode(reader, state).set_err_elem::<Self>()?;
                match tag {
                    #( #decode_variants )*
                    byte => Err(crate::error::Error::invalid_tag::<#data_name>(offset_before, byte))?
                }
            })
        }
        _ => unimplemented!("can only derive(WasmBinary) for structs and enums")
    };

    let encode_match_arms = match input.data {
        Data::Struct(DataStruct { ref fields, .. }) => {
            let tag: Option<u8> = attributes_to_tag(&input.attrs);
            encode_fields(&parse_quote!(#data_name), tag, &fields)
        }
        Data::Enum(DataEnum { ref variants, .. }) => {
            let encode_variants = variants.iter().map(|variant| encode_variant(data_name, variant));
            quote!( #( #encode_variants ),* )
        }
        _ => unimplemented!("can only derive(WasmBinary) for structs and enums")
    };

    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    quote!(
        impl #impl_generics WasmBinary for #data_name #ty_generics #where_clause {
            fn decode<R: ::std::io::Read>(reader: &mut R, state: &mut crate::binary::DecodeState) -> ::std::result::Result<Self, crate::error::Error> {
                use crate::error::SetErrElem;
                Ok(#decode_expr)
            }
            fn encode<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<usize> {
                let mut bytes_written = 0;
                match *self {
                    #encode_match_arms
                };
                Ok(bytes_written)
            }
        }
    ).into()
}

/// Take the first `#[tag = <byte literal>]` attribute and return the value of `byte literal`.
fn attributes_to_tag(attributes: &[Attribute]) -> Option<u8> {
    let attribute = attributes.first()?;
    match attribute.interpret_meta() {
        Some(Meta::NameValue(MetaNameValue { ident, lit: Lit::Int(ref uint), .. }))
        if ident == "tag" && uint.value() <= u8::max_value() as u64 =>
            Some(uint.value() as u8),
        _ => panic!("attribute must be of type #[tag = <u8 literal>], got {}", quote!(#attribute))
    }
}

/* for decode() */

fn decode_variant(super_name: Ident, variant: &Variant) -> Tokens {
    #[allow(clippy::expect_fun_call)]
    let tag = attributes_to_tag(&variant.attrs)
        .expect(&format!("every enum variant needs a tag, but {} does not have one", variant.ident));
    let variant_name = &variant.ident;
    let name = parse_quote!(#super_name::#variant_name);

    let decode_fields = decode_fields(&name, &variant.fields);
    quote!( #tag => #decode_fields, )
}

fn decode_fields(name: &TypePath, fields: &Fields) -> Tokens {
    let decoded_fields = fields.iter().map(decode_field);
    match *fields {
        Fields::Unit => quote!(#name),
        Fields::Unnamed(_) => quote!(#name( #( #decoded_fields ),* )),
        Fields::Named(_) => quote!(#name { #( #decoded_fields ),* }),
    }
}

fn decode_field(field: &Field) -> Tokens {
    let field_name = field.ident;
    let field_ty = remove_type_arguments(&field.ty);
    quote!( #( #field_name: )* #field_ty::decode(reader, state)? )
}

/// Transform, e.g., Vec<T> into just Vec. Useful when calling trait methods on a generic type, i.e.,
/// Vec<T>::decode() is not valid syntax but Vec::decode() is.
fn remove_type_arguments(ty: &Type) -> Type {
    let mut ty = ty.clone();
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

/* for encode() */

fn encode_variant(super_name: Ident, variant: &Variant) -> Tokens {
    let tag = attributes_to_tag(&variant.attrs);
    let variant_name = &variant.ident;
    let name = parse_quote!(#super_name::#variant_name);

    encode_fields(&name, tag, &variant.fields)
}

fn encode_fields(name: &TypePath, tag: Option<u8>, fields: &Fields) -> Tokens {
    let field_names: &Vec<_> = &fields.iter().enumerate().map(encode_field_name).collect();
    let body = quote!({
        #( bytes_written += #tag.encode(writer)?; )*
        #( bytes_written += #field_names.encode(writer)? );*
    });
    match *fields {
        Fields::Unit => quote!(#name => #body),
        Fields::Unnamed(_) => quote!( #name( #( ref #field_names ),* ) => #body),
        Fields::Named(_) => quote!( #name { #( ref #field_names ),* } => #body),
    }
}

fn encode_field_name((i, field): (usize, &Field)) -> Ident {
    field.ident.unwrap_or_else(|| Ident::from(format!("_{}", i)))
}