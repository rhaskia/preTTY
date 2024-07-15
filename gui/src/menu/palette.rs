use config::TerminalAction;
use dioxus::prelude::*;

use crate::{COMMAND_PALETTE, handle_action};

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

    use_future(|| async {
        wait_for_next_render().await;

        let mut clickoff = eval(r#"
            document.addEventListener('click', function(event) {
                const divElement = document.getElementById('commandpalette');
                if (divElement && !divElement.contains(event.target)) {
                    dioxus.send({});
                }
            });
        "#);

        loop {
            clickoff.recv().await;
            handle_action(TerminalAction::ToggleCommandPalette);
        }
    });

    rsx! {
        div {
            class: "commandpalette",
            id: "commandpalette",
            input {
                class: "commandsearch",
                oninput: move |event| search.set(event.value()),
                onkeydown: move |e| match e.key() {
                    Key::ArrowUp if selected() != 0 => selected -= 1, 
                    Key::ArrowUp => *selected.write() = matches.read().len() - 1, 
                    Key::ArrowDown if selected() != matches.read().len() - 1 => selected += 1, 
                    Key::ArrowDown => *selected.write() = 0, 
                    _ => {}
                }
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
