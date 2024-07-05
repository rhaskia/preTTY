mod settings;
use dioxus::prelude::*;

#[component]
pub fn Menu(active: bool) -> Element {
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
            display: if active { "block" } else { "none" },
            id: "menu",
            div {
                class: "menucontent",
                div { 
                  id: "menuheader", 
                  class: "menuheader",
                  h2 { "Settings" }, 
                }
                div { "Font Size" input { r#type: "number" } }
            }
        }
    }
}