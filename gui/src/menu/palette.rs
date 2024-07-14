use dioxus::prelude::*;

#[component]
pub fn CommandPalette() -> Element {
    let examples = use_signal(|| {
        vec!["Close Tab", "New Tab", "Exit"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
    });
    let search = use_signal(|| String::new());
    let matches = use_memo(move || {
        examples()
            .into_iter()
            .filter(|item| item.starts_with(&search()))
            .collect::<Vec<String>>()
    });

    rsx! {
        div {
            class: "commandpalette",
            input {
                class: "commandsearch",
            }
            div {
                class: "results",
                for result in matches() {
                    div {
                        class: "searchresult",
                        "{result}",
                    }
                }
            }
        }
    }
}
