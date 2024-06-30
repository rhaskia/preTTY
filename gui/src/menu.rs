mod settings;
use dioxus::prelude::*;

#[component]
pub fn Menu(menu_open: Signal<bool>) -> Element {
    // let config = Config::new()
    //     .with_background_color((0, 0, 0, 0))
    //     .with_disable_context_menu(true)
    //     .with_menu(None)
    //     .with_window(WindowBuilder::new().with_transparent(true).with_decorations(false));
    //
    // let err = use_window().new_window(VirtualDom::new(FloatingMenu), config);
    //
    use_future(move || async move {
        wait_for_next_render().await;
        eval(
            r#"
                 dragElement(document.getElementById("menu"));

                function dragElement(elmnt) {
                  var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
                  console.log(elmnt);
                  if (document.getElementById("menuheader")) {
                    // if present, the header is where you move the DIV from:
                    document.getElementById("menuheader").onmousedown = dragMouseDown;
                  } else {
                    // otherwise, move the DIV from anywhere inside the DIV:
                    elmnt.onmousedown = dragMouseDown;
                  }

                  function dragMouseDown(e) {
                    e = e || window.event;
                    e.preventDefault();
                    // get the mouse cursor position at startup:
                    pos3 = e.clientX;
                    pos4 = e.clientY;
                    document.onmouseup = closeDragElement;
                    // call a function whenever the cursor moves:
                    document.onmousemove = elementDrag;
                  }

                  function elementDrag(e) {
                    e = e || window.event;
                    e.preventDefault();
                    // calculate the new cursor position:
                    pos1 = pos3 - e.clientX;
                    pos2 = pos4 - e.clientY;
                    pos3 = e.clientX;
                    pos4 = e.clientY;
                    // set the element's new position:
                    elmnt.style.top = (elmnt.offsetTop - pos2) + "px";
                    elmnt.style.left = (elmnt.offsetLeft - pos1) + "px";
                  }

                  function closeDragElement() {
                    // stop moving when mouse button is released:
                    document.onmouseup = null;
                    document.onmousemove = null;
                  }
                }
         "#,
        );
    });

    rsx! {
        div {
            class: "menu",
            id: "menu",
            div {
                class: "menucontent",
                div { 
                  id: "menuheader", 
                  class: "menuheader",
                  h2 { "Settings" }, 
                  button { 
                    onclick: move |_| menu_open.toggle(),
                    "X"
                  } 
                }
                div { "Font Size" input { r#type: "number" } }
            }
        }
    }
}

#[component]
pub fn FloatingMenu() -> Element {
    rsx! {
        div {
            class: "menu",
            height: "400px",
            width: "40px",
            "hello"
        }
    }
}
