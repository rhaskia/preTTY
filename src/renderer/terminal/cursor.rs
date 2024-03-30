use dioxus::prelude::*;
use serde::Serialize;
use serde_json::to_value;

#[derive(Serialize)]
pub struct CursorInfo {
    pub y: usize,
    pub index: usize,
}

#[component]
pub fn Cursor(x: usize, y: usize, index: usize) -> Element {
    use_future(move || async move {
        wait_for_next_render().await;
        println!("cursor rendered");

        let mut line_eval = eval(
            r#"
            let { y, index} = await dioxus.recv();
            let line = document.getElementById("line_" + y);
            console.log(line, y);
            let cursor = document.getElementById("cursor-" + index);
            if (line) {
                cursor.style.top = `calc(${line.offsetTop}px - var(--cell-height))`;
            }
            "#,
        );

        line_eval.send(to_value(CursorInfo { y, index }).unwrap()).unwrap();
    });

    rsx! {
        div {
            class: "cursor",
            id: "cursor-{index}",
            left: "calc({x} * var(--cell-width) + var(--padding))",
            height: "var(--cell-height)",
            width: "var(--cell-width)",
        }
    }
}
