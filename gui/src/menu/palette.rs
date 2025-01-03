use config::TerminalAction;
use dioxus::prelude::*;
use crate::{handle_action, COMMAND_PALETTE};
use dioxus_document::{Eval, Evaluator, eval};
use pretty_hooks::wait_for_next_render;

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
    let mut raw_selected = use_signal(|| 0.0f64);
    let selected = use_memo(move || raw_selected() as usize);
    use_effect(move || if raw_selected() >= matches.read().len() as f64 {
        if matches.read().len() == 0 { return; }
        *raw_selected.write() = (matches.read().len() - 1) as f64;
    });

    use_future(move || async move {
        wait_for_next_render().await;

        let mut clickoff = eval(
            r#"
            document.addEventListener('click', function(event) {
                const divElement = document.getElementById('commandpalette');
                if (!divElement.hidden && !divElement.contains(event.target)) {
                    dioxus.send(0);
                }
            });
        "#,
        );

        loop {
            clickoff.recv::<i32>().await.ok();
            if COMMAND_PALETTE() { handle_action(TerminalAction::ToggleCommandPalette); }
        }
    });

    rsx! {
        div {
            class: "commandpalette",
            id: "commandpalette",
            hidden: !COMMAND_PALETTE(),
            input {
                class: "commandsearch",
                oninput: move |event| search.set(event.value()),
                onmounted: |_| { eval("document.getElementById('commandsearch').focus();"); },
                tabindex: 0,
                onkeydown: move |e| match e.key() {
                    Key::ArrowUp if selected() != 0 => raw_selected -= 1.0,
                    Key::ArrowUp => *raw_selected.write() = matches.read().len() as f64 - 1.0,
                    Key::ArrowDown if selected() != matches.read().len() - 1 => raw_selected += 1.0,
                    Key::ArrowDown => *raw_selected.write() = 0.0,
                    Key::Enter => {
                        handle_action(matches.read()[selected()].clone());
                        *COMMAND_PALETTE.write() = false;
                    }
                    _ => {}
                }
            }
            select {
                class: "searchresults",
                id: "searchresults",
                size: 999,
                value: raw_selected,
                onmounted: |_| { eval("document.getElementById('searchresults').value = 0;"); },
                for (i, result) in matches().into_iter().enumerate() {
                    option {
                        class: "searchresult",
                        value: i as f64,
                        onclick: move |_| {
                            handle_action(matches.read()[i].clone());
                            *COMMAND_PALETTE.write() = false;
                        },
                        "{result}"
                    }
                }
            }
        }
    }
}
