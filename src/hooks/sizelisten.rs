use dioxus::{signals::Signal, hooks::{UseFuture, use_signal}, events::UseEval};
use dioxus::prelude::*;
use serde::Deserialize;
use std::fmt::Debug;
use std::cell::Ref;

pub struct DomRectSignal {
    pub inner: Signal<ResizeObserverEntry>,
    pub collector: UseFuture,
    pub js: UseEval,
}

impl DomRectSignal {
    pub fn value(&self) -> ResizeObserverEntry{
        self.inner.read().clone()
    } 
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct ResizeObserverEntry {
    content_rect: DOMRectReadOnly,
    border_box_size: Vec<ResizeObserverSize>,
    content_box_size: Vec<ResizeObserverSize>,
    device_pixel_content_box_size: Vec<ResizeObserverSize>,
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct ResizeObserverSize {
    inline_size: f64,
    block_size: f64,
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct DOMRectReadOnly {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    top: f64,
    right: f64,
    bottom: f64,
    left: f64,
}

pub fn use_div_size(id: String) -> DomRectSignal {
    let mut signal = use_signal(|| ResizeObserverEntry::default());

    let mut js = eval(
        r#"
        let id = await dioxus.recv();
        
        console.log('Element is ready');
        console.log(el);
        let div = document.getElementById(id);
        const ro = new ResizeObserver(entries => {
            for (let entry of entries) {
                console.log(entry.contentRect);
                dioxus.send({ 
                    content_rect: entry.contentRect, 
                    border_box_size: entry.borderBoxSize, 
                    content_box_size: entry.contentBoxSize 
                    device_pixel_content_box_size: entry.devicePixelContentBoxSize,
                }); 
            }
        });
        ro.observe(div);
        "#,
    );

    js.send(id.into()).unwrap();

    let collector = use_future(move || async move {
        println!("what the hell");
        loop {
            let div_info = js.recv().await.unwrap();
            println!("{div_info:?}");
            let parsed = serde_json::from_value::<ResizeObserverEntry>(div_info).unwrap();
            *signal.write() = parsed;
        }
    });

    DomRectSignal { inner: signal, js, collector }
}
