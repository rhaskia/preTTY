use std::fmt::Display;
use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[macro_export]
#[proc_macro_derive(Form)]
pub fn create_form(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    println!("{:?}", ast);

    let name = format!("{}", &ast.ident.to_string());
    let gen = quote! {
        #[component]
        pub fn TestForm(value: Signal<i32>) -> dioxus::prelude::Element {
            rsx! { div { #name } }
        }
    };

    match ast.data {
        Data::Struct(data) => {},
        Data::Enum(data) => {},
        Data::Union(data) => {}
    }

    gen.into()
}
