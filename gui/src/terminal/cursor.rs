use dioxus::prelude::*;
use dioxus_document::{Eval, Evaluator, eval};
use serde::Serialize;
use serde_json::to_value;
use pretty_hooks::wait_for_next_render;

#[derive(Serialize)]
pub struct CursorInfo {
    pub y: usize,
    pub index: String,
}

#[component]
pub fn Cursor(cursor_pos: Memo<(usize, usize)>, index: String) -> Element {
    let index = use_signal(|| index);
    use_future(move || async move {
        loop {
            wait_for_next_render().await;

            let line_eval = eval(
                r#"
                let { y, index} = await dioxus.recv();
                let line = document.getElementById("line_" + y);
                let cursor = document.getElementById("cursor-" + index);
                if (line) {
                    let top = line.getBoundingClientRect().top;
                    cursor.style.setProperty("--line-height","" + top + "px");
                }
                "#,
            );

            line_eval
                .send(
                    to_value(CursorInfo {
                        y: cursor_pos.read().1,
                        index: index(),
                    })
                    .unwrap(),
                )
                .unwrap();
        }
    });

    rsx! {
        div {
            class: "cursor",
            id: "cursor-{index}",
            style: "--column: {cursor_pos().0}",
        }
    }
}
