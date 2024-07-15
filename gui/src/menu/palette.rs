use dioxus::prelude::*;

#[component]
pub fn CommandPalette() -> Element {
    let examples = use_signal(|| {
        vec!["Close Tab", "New Tab", "Exit"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<String>>()
    });
    let mut search = use_signal(|| String::new());
    let matches = use_memo(move || {
        examples()
            .into_iter()
            .filter(|item| item.to_lowercase().starts_with(&search().to_lowercase()))
            .collect::<Vec<String>>()
    });
    let mut selected = use_signal(|| 0);

    rsx! {
        div {
            class: "commandpalette",
            input {
                class: "commandsearch",
                oninput: move |event| search.set(event.value()),
            }
            div {
                class: "results",
                for (i, result) in matches().into_iter().enumerate() {
                    div {
                        class: "searchresult",
                        class: if selected() == i { "selected" },
                        "{result}",
                    }
                }
            }
        }
    }
}
