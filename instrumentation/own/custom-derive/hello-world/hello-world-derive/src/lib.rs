extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::DeriveInput;
use syn::Attribute;

#[proc_macro_derive(HelloWorld, attributes(tag))]
pub fn hello_world(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input: DeriveInput = syn::parse(input).unwrap();

//    println!("{:?}", input);

    if let DeriveInput { data: syn::Data::Enum(syn::DataEnum{ variants, .. }), .. } = input {
        println!("{:?}", variants.into_iter().collect::<Vec<syn::Variant>>());
    }

//    let attrs: Vec<Option<syn::Meta>> = input.attrs.iter().map(Attribute::interpret_meta).collect();
//
//    println!("{:?}", attrs);

    // Build the impl
    let name = input.ident;
    let expanded = quote! {
        impl HelloWorld for #name {
            fn hello_world() {
                println!("My name is {}", stringify!(#name));
            }
        }
    };

    // Return the generated impl
    expanded.into()
}
