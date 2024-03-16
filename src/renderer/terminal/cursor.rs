use dioxus::prelude::*;

#[component]
pub fn Cursor(x: usize, y: usize) -> Element {
    let mut line_eval = eval(
        r#"
        let y = await dioxus.recv();
        let line = document.getElementById("line_" + y);
        let cursor = document.getElementById("cursor");
        cursor.style.top = `calc(${line.offsetTop}px - var(--cell-height))`;
        "#,
    );

    line_eval.send(y.to_string().into()).unwrap();

    rsx! {
        div {
            class: "cursor",
            id: "cursor",
            left: "calc({x} * var(--cell-width) + var(--padding))",
            height: "var(--cell-height)",
            width: "var(--cell-width)",
        }
    }
}
