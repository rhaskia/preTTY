use dioxus::prelude::*;
use dioxus_desktop::tao::event::Event;
use crate::{renderer::terminal::TerminalApp};
use crate::renderer::header::Header;
use dioxus_desktop::{use_wry_event_handler, use_window};

pub fn app(cx: Scope) -> Element {
    let window = use_window(cx);

    use_wry_event_handler(cx, move |event, _t| match event {
        Event::NewEvents(_) => {},
        Event::UserEvent(_) => {},
        Event::Suspended => {},
        Event::Resumed => {},
        Event::MainEventsCleared => {},
        Event::RedrawRequested(_) => {},
        Event::RedrawEventsCleared => {},
        Event::LoopDestroyed => {},
        Event::DeviceEvent {   .. } => {},
        _ => println!("{event:?}"),
    });

    cx.render(rsx! {
        style { include_str!("style.css") }
        //Header {}
        TerminalApp {}
    })
}
