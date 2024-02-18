use dioxus::prelude::*;
use dioxus_desktop::use_window;
use dioxus_signals::use_signal;

#[component]
pub fn Header(cx: Scope) -> Element {
    let mut fullscreen = use_signal(cx, || false);
    let window = use_window(cx);

    cx.render(rsx! {
        header { class: "window-header", onmousedown: move |_| window.drag(),
            pre {
               "Window Title"     
            }

            // Set the window to minimized
            button {
                class: "header-button",
                onmousedown: |evt| evt.stop_propagation(),
                onclick: move |_| window.set_minimized(true),
                "🗕"
            }

            // Toggle fullscreen
            button {
                class: "header-button",
                onmousedown: |evt| evt.stop_propagation(),
                onclick: move |_| {
                    window.set_fullscreen(!*fullscreen());
                    window.set_resizable(*fullscreen());
                    fullscreen.toggle();
                },
                if *fullscreen() { "🗗" } else { "🗖" }
            }

            // Close the window
            // If the window is the last window open, the app will close, if you configured the close behavior to do so
            button {
                class: "header-button",
                onmousedown: |evt| evt.stop_propagation(),
                onclick: move |_| window.close(),
                "🗙"
            }
        }
    })
}
