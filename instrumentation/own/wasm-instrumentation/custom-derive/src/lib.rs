#![recursion_limit = "128"]

extern crate proc_macro;
#[macro_use]
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use quote::{Tokens, ToTokens};
use syn::{parse_quote, Attribute, Data, DataEnum, DataStruct, DeriveInput, Field, Fields, FieldsNamed, FieldsUnnamed, Ident, Index, Lit, Meta, MetaNameValue, Path, PathArguments, PathSegment, Type, TypePath, Variant};

#[proc_macro_derive(WasmBinary, attributes(tag))]
pub fn derive_wasm(input: TokenStream) -> TokenStream {
    let input: DeriveInput = syn::parse(input).unwrap();
    let data_name = input.ident;

    // TODO refactor this mess: e.g., make tag proper Option<u8>, include handling of tags for structs here
    // extract handling of fields since it is the same for structs and enums
    let recurse_into_fields = |name: Ident, tag: Option<u8>, fields: Fields| -> (Tokens, Tokens) {
        match fields {
            Fields::Unit => (quote!(#name), quote!(&#data_name::#name => bytes_written += #tag.encode(writer)?)),
            Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                let (field_idx, field_tys): (Vec<Index>, Vec<Type>) = unnamed.into_iter()
                    .enumerate()
                    .map(|(idx, field)| (Index::from(idx), remove_type_arguments(&field.ty)))
                    .unzip();
                let field_idx_name: Vec<Ident> = (0..field_idx.len()).map(|i| Ident::from(format!("_{}", i))).collect();
                let field_idx_name_2 = field_idx_name.clone();
                (quote! {
                    #name(
                        #( #field_tys::decode(reader)? ),*
                    )
                },
                 quote!(
                    &#data_name::#name(
                        #( ref #field_idx_name ),*
                    ) => {
                        bytes_written += #tag.encode(writer)?;
                        #( bytes_written += #field_idx_name_2.encode(writer)? );*
                    }
                ))
            }
            Fields::Named(FieldsNamed { named, .. }) => {
                let (field_names, field_tys): (Vec<Ident>, Vec<Type>) = named.into_iter()
                    .map(|field| (field.ident.unwrap(), remove_type_arguments(&field.ty)))
                    .unzip();
                let field_names_2 = field_names.clone();
                let field_names_3 = field_names.clone();
                let field_names_4 = field_names.clone();
                (quote! {
                    #name {
                        #( #field_names: #field_tys::decode(reader)? ),*
                    }
                },
                 quote!(
                    &#data_name::#name(
                        #( ref #field_names_3 ),*
                    ) => {
                        bytes_written += #tag.encode(writer)?;
                        #( bytes_written += #field_names_4.encode(writer)? );*
                    }
                ))
            }
        }
    };

    let (decode_body, encode_body) = match input.data {
        Data::Struct(DataStruct { fields, .. }) => {
            let tag: Option<u8> = attributes_to_tag(&input.attrs);

            // decode struct
            let decode_tag = tag.map(|tag| quote! {
                let byte = u8::decode(reader)?;
                if byte != #tag {
                    return Self::error(format!("expected tag for {}, got 0x{:02x}", stringify!(#data_name), byte));
                }
            });
            let decode_fields = decode_fields(&parse_quote!(#data_name), &fields);

            // encode struct
            let encode_tag = tag.map(|tag| quote! {
                bytes_written += #tag.encode(writer)?;
            });

            let encode_fields = match fields {
                Fields::Unit => Vec::new(),
                Fields::Unnamed(_) => (0..fields.iter().count()).map(|i| Index::from(i).into_tokens()).collect(),
                Fields::Named(_) => fields.iter().map(|field| field.ident.unwrap().into_tokens()).collect(),
            };
            let encode_fields = quote! {
                #( bytes_written += self.#encode_fields.encode(writer)?; )*
            };

            // decode and encode body
            (quote!({
                #( #decode_tag )*
                #decode_fields
            }),
             quote! {
                #( #encode_tag )*
                #encode_fields
            })
        }
        Data::Enum(DataEnum { variants, .. }) => {

            let decode_variant = variants.iter().map(|variant| decode_variant(&data_name, variant)).collect::<Vec<_>>();

            let variant_tags: Vec<u8> = variants.iter()
                // FIXME is filter_map correct?
                .filter_map(|variant| attributes_to_tag(&variant.attrs))
                .collect();
            let (variants_decode, variants_encode): (Vec<Tokens>, Vec<Tokens>) = variants.into_iter().enumerate()
                .map(|(idx, variant)| recurse_into_fields(variant.ident, Some(variant_tags[idx]), variant.fields))
                .unzip();

            // needs to be repeated for quote repetition with #( ... )*
            let data_name_repeated = vec![data_name; variant_tags.len()];

            // match enum variants by tag
            (quote! {
                match u8::decode(reader)? {
                    #( #decode_variant )*
                    byte => Self::error(format!("expected tag for {}, got 0x{:02x}", stringify!(#data_name), byte))?
                }
            },
             quote! {
                match self {
                    #( #variants_encode ),*
                };
            })
        }
        _ => unimplemented!("can only derive(Wasm) for structs and enums")
    };

    // boilerplate of impl that is the same for any data type
    let impl_ = quote! {
        impl WasmBinary for #data_name {
            fn decode<R: ::std::io::Read>(reader: &mut R) -> ::std::io::Result<Self> {
                Ok(#decode_body)
            }
            fn encode<W: ::std::io::Write>(&self, writer: &mut W) -> ::std::io::Result<usize> {
                let mut bytes_written = 0;
                #encode_body
                Ok(bytes_written)
            }
        }
    };

    impl_.into()
}

fn decode_variant(super_name: &Ident, variant: &Variant) -> Tokens {
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
    quote!( #( #field_name: )* #field_ty::decode(reader)? )
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

/// Take the first `#[tag = <byte literal>]` attribute and return the value of `byte literal`.
fn attributes_to_tag(attributes: &[Attribute]) -> Option<u8> {
    let attribute = attributes.first()?;
    match attribute.interpret_meta() {
        Some(Meta::NameValue(MetaNameValue { ident, lit: Lit::Int(ref uint), .. }))
        if ident.to_string() == "tag" && uint.value() <= u8::max_value() as u64 =>
            Some(uint.value() as u8),
        _ => panic!("attribute must be of type #[tag = <u8 literal>], got {}", quote!(#attribute))
    }
}
