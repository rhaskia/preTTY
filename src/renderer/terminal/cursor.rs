use dioxus::prelude::*;

#[component]
pub fn Cursor(x: usize, y: usize, index: usize) -> Element {
    let mut line_eval = eval(
        r#"
        let y = await dioxus.recv();
        let line = document.getElementById("line_" + y);
        let index = await dioxus.recv();
        let cursor = document.getElementById("cursor-" + index);
        cursor.style.top = `calc(${line.offsetTop}px - var(--cell-height))`;
        "#,
    );

    line_eval.send(y.to_string().into()).unwrap();
    line_eval.send(index.to_string().into()).unwrap();

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
