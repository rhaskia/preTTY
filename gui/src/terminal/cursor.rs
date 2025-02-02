use std::time::Duration;

use dioxus::prelude::*;
use dioxus_document::{Eval, Evaluator, eval};
use serde::Serialize;
use serde_json::to_value;

#[derive(Serialize)]
pub struct CursorInfo {
    pub y: usize,
    pub index: String,
}

#[component]
pub fn Cursor(cursor_pos: Memo<(usize, usize)>, index: String) -> Element {
    let index = use_signal(|| index);
    let mut line_height = use_signal(|| 18.0);

    use_future(move || async move {
        loop {
            //wait_for_next_render().await;
            tokio::time::sleep(Duration::from_secs_f64(0.1));

            let mut line_eval = eval(
                r#"
                let { y, index} = await dioxus.recv();
                let line = document.getElementById("line_" + y);
                let cursor = document.getElementById("cursor-" + index);

                if (line) {
                    let top = line.getBoundingClientRect().top + cursor.parentElement.scrollTop;
                    await dioxus.send(top);
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

            if let Ok(lh) = line_eval.recv().await {
                line_height.set(lh);
            }
        }
    });

    rsx! {
        div {
            class: "cursor",
            id: "cursor-{index}",
            style: "--column: {cursor_pos().0}; --line-height: {line_height}px",
        }
    }
}
