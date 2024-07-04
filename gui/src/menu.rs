mod settings;
use dioxus::prelude::*;

#[component]
pub fn Menu(menu_open: Signal<bool>) -> Element {
    // let config = Config::new()
    //     .with_background_color((0, 0, 0, 0))
    //     .with_disable_context_menu(true)
    //     .with_menu(None)
    //     .with_window(WindowBuilder::new().with_transparent(true).with_decorations(false));
    //
    // let err = use_window().new_window(VirtualDom::new(FloatingMenu), config);
    //

    rsx! {
        div {
            class: "menu",
            id: "menu",
            div {
                class: "menucontent",
                div { 
                  id: "menuheader", 
                  class: "menuheader",
                  h2 { "Settings" }, 
                  button { 
                    onclick: move |_| menu_open.toggle(),
                    "X"
                  } 
                }
                div { "Font Size" input { r#type: "number" } }
            }
        }
    }
}

#[component]
pub fn FloatingMenu() -> Element {
    rsx! {
        div {
            class: "menu",
            height: "400px",
            width: "40px",
            "hello"
        }
    }
}
