use config::TerminalAction;
use dioxus::prelude::*;
use log::log;
use crate::{handle_action, COMMAND_PALETTE};

#[component]
pub fn CommandPalette() -> Element {
    let commands = use_signal(|| TerminalAction::palette_usable());
    let mut search = use_signal(|| String::new());
    let commands_str = use_signal(|| {
        TerminalAction::palette_usable()
            .into_iter()
            .map(|a| a.readable())
    });
    let matches = use_memo(move || {
        commands()
            .into_iter()
            .filter(|c| c.readable().to_lowercase().starts_with(&search().to_lowercase()))
            .collect::<Vec<TerminalAction>>()
    });
    let mut selected = use_signal(|| 0);

    use_future(|| async {
        wait_for_next_render().await;

        let mut clickoff = eval(
            r#"
            document.addEventListener('click', function(event) {
                const divElement = document.getElementById('commandpalette');
                if (divElement && !divElement.contains(event.target)) {
                    dioxus.send({});
                }
            });
        "#,
        );

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
                    Key::Enter => {
                        handle_action(matches.read()[selected()].clone());
                        println!("Called action: {}", matches.read()[selected()].clone());
                        *COMMAND_PALETTE.write() = false;
                    }
                    _ => {}
                }
            }
            div {
                class: "searchresults",
                id: "searchresults",
                for (i, result) in matches().into_iter().enumerate() {
                    div {
                        class: "searchresult",
                        class: if selected() == i { "selected" },
                        // onmounted: move |_| if selected() == i { 
                        //     eval(r#"
                        //          document.getElementsByClassName("selected")[0].scrollIntoViewIfNeeded()
                        //     "#);
                        // },
                        "{result}"
                    }
                }
            }
        }
    }
}
