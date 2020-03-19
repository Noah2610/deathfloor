extern crate proc_macro;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, Fields};

#[proc_macro_derive(Merge)]
pub fn merge_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_merge(&ast)
}

fn impl_merge(ast: &syn::DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let fields = if let Data::Struct(data) = &ast.data {
        if let Fields::Named(fields) = &data.fields {
            fields
        } else {
            unimplemented!("derive(Merge) is only implemented for named fields")
        }
    } else {
        unimplemented!("derive(Merge) is only implemented for structs")
    };

    let gen = quote! {
        impl Merge for #struct_name {
            fn merge(mut self, other: Self) -> Self {
            }
        }
    };

    gen.into()
}
