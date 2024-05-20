use dioxus::prelude::*;
use serde::Serialize;
use serde_json::to_value;

#[derive(Serialize)]
pub struct CursorInfo {
    pub y: usize,
    pub index: usize,
}

#[component]
pub fn Cursor(cursor_pos: Memo<(usize, usize)>, index: usize) -> Element {
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
                    cursor.style.top = `calc(${top}px - var(--cell-height))`;
                }
                "#,
            );

            line_eval
                .send(
                    to_value(CursorInfo {
                        y: cursor_pos.read().1,
                        index,
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
            left: "calc({cursor_pos().0} * var(--cell-width) + var(--padding))",
            height: "var(--cell-height)",
            width: "var(--cell-width)",
        }
    }
}
