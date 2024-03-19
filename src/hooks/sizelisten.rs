use dioxus::{signals::Signal, hooks::{UseFuture, use_signal}, events::UseEval};
use dioxus::prelude::*;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct DomRect {
    pub x: f32, 
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

pub struct DomRectSignal {
    pub inner: Signal<DomRect>,
    pub collector: UseFuture,
    pub js: UseEval,
}

impl DomRectSignal {
    pub fn value(&self) -> Signal<DomRect> {
        self.inner.clone()
    } 
}

pub fn use_div_size(id: String) -> DomRectSignal {
    let mut signal = use_signal(|| DomRect { x: 0.0, y: 0.0, width: 0.0, height: 0.0 });

    let mut js = eval(
        r#"
        let id = await dioxus.recv();
        let div = document.getElementById(id);
        console.log("Listening to resize of: ", div);
        const ro = new ResizeObserver(entries => {
            for (let entry of entries) {
                dioxus.send({ borderBox: entry.borderBoxSize,
                              contentBox: entry.contentBoxSize });      
            }
        });
        ro.observe(div);
        "#,
    );

    js.send(id.into()).unwrap();

    let collector = use_future(move || async move {
        loop {
            let div_info = js.recv().await.unwrap();
            let parsed = serde_json::from_value::<DomRect>(div_info).unwrap();
            *signal.write() = parsed;
        }
    });

    DomRectSignal { inner: signal, js, collector }
}
