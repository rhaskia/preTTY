#![feature(error_generic_member_access)]
mod serializer;
mod deserializer;
use serde::ser::Serialize;
use dioxus::prelude::*;
use serializer::create_form;

#[component]
pub fn Form<T: Serialize + 'static + PartialEq>(value: Signal<T>) -> Element {
    rsx! {
        form {
            oninput: |i| println!("{i:?}"),
            dangerous_inner_html: create_form(value).ok()?
        }
    }
}
