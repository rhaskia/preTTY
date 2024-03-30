use dioxus::{signals::Signal, hooks::{UseFuture, use_signal}, events::UseEval};
use dioxus::prelude::*;
use serde::Deserialize;
use std::fmt::Debug;
use std::cell::Ref;

pub struct DomRectSignal {
    pub inner: Signal<Option<ResizeObserverEntry>>,
    pub collector: UseFuture,
}

impl DomRectSignal {
    pub fn value(&self) -> Option<ResizeObserverEntry> {
        self.inner.read().clone()
    } 

    pub fn read(&self) -> ReadableRef<Signal<Option<ResizeObserverEntry>>> {
        self.inner.read()
    } 
}

#[derive(Deserialize, Default, Clone, Debug)]
pub struct ResizeObserverEntry {
    content_rect: DOMRectReadOnly,
    border_box_size: Option<ResizeObserverSize>,
    content_box_size: Option<ResizeObserverSize>,
    device_pixel_content_box_size: Option<ResizeObserverSize>,
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
    let id = use_signal(|| id);
    let mut signal = use_signal(|| None);

    let collector = use_future(move || async move {
        wait_for_next_render().await;
        println!("rendered");

        let mut js = eval(
            r#"
            let id = await dioxus.recv();
            console.log(id);

            let div = document.getElementById(id);
            if (!div) {
                console.error("Element with id " + id + " not found.");
                return;
            }

            function ex(resizeEntry) {
                console.log(resizeEntry);
                let { inlineSize, blockSize } = resizeEntry[0];
                console.log(inlineSize);
                return { 
                    inline_size: inlineSize,
                    block_size: blockSize,
                };
            }

            const ro = new ResizeObserver(entries => {
                for (let entry of entries) {
                    let send_info = { 
                        content_rect: entry.contentRect, 
                        border_box_size: ex(entry.borderBoxSize), 
                        content_box_size: ex(entry.contentBoxSize),
                        device_pixel_content_box_size: ex(entry.devicePixelContentBoxSize),
                    };
                    console.log(send_info);
                    dioxus.send(send_info); 
                }
            });

            ro.observe(div);
            console.log("div observed");
            "#,
        );

        js.send(id.read().clone().into()).unwrap();

        loop {
            let div_info = js.recv().await.unwrap();
            println!("{div_info:?}");
            let parsed = serde_json::from_value::<ResizeObserverEntry>(div_info).unwrap();
            *signal.write() = Some(parsed);
        }
    });

    DomRectSignal { inner: signal, collector }
}
