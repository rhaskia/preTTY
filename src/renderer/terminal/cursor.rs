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
    let mut line_eval = eval(
        r#"
        let { y, index} = await dioxus.recv();
        let line = document.getElementById("line_" + y);
        console.log(line);
        let cursor = document.getElementById("cursor-" + index);
        cursor.style.top = `calc(${line.offsetTop}px - var(--cell-height))`;
        "#,
    );

    line_eval.send(to_value(CursorInfo { y, index }).unwrap()).unwrap();

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
