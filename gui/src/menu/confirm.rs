use dioxus::prelude::*;

#[component]
pub fn Confirm(open: Signal<bool>, onconfirm: EventHandler<()>, message: String) -> Element {
    let mut choice = use_signal(|| false);
    use_effect(move || {
        if choice() {
            onconfirm.call(());
            open.set(false);
        }
    });

    rsx! {
        div {
            class: "confirmselect",
            p { "{message}" }
            button {
                class: "confirmbutton",
                onclick: move |_| choice.set(true),
                "Confirm"
            }
            button {
                class: "cancelbutton",
                onclick: move |_| open.set(false),
                "Cancel"
            }
        }
    }
}