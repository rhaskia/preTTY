use std::fmt::Debug;
use dioxus::prelude::*;
use serde::Deserialize;

pub struct DomRectSignal {
    inner: Signal<Option<ResizeObserverEntry>>,
    _collector: UseFuture,
}

impl DomRectSignal {
    pub fn value(&self) -> Option<ResizeObserverEntry> { self.inner.read().clone() }

    pub fn read(&self) -> ReadableRef<Signal<Option<ResizeObserverEntry>>> { self.inner.read() }
}

#[allow(dead_code)]
#[derive(Deserialize, Default, Clone, Debug)]
pub struct ResizeObserverEntry {
    pub content_rect: DOMRectReadOnly,
    pub border_box_size: Option<ResizeObserverSize>,
    pub content_box_size: Option<ResizeObserverSize>,
    pub device_pixel_content_box_size: Option<ResizeObserverSize>,
}

#[allow(dead_code)]
#[derive(Deserialize, Default, Clone, Debug)]
pub struct ResizeObserverSize {
    pub inline_size: f32,
    pub block_size: f32,
}

#[allow(dead_code)]
#[derive(Deserialize, Default, Clone, Debug)]
pub struct DOMRectReadOnly {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

pub fn resize_observer() -> UseEval {
    eval(
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
    )
}

pub fn on_resize(id: String, callback: impl FnMut(ResizeObserverEntry) + 'static) -> UseFuture {
    let id = use_signal(|| id);
    let mut callback = use_signal(|| callback);

    use_future(move || async move {
        wait_for_next_render().await;

        let mut js = resize_observer();

        js.send(id.read().clone().into()).unwrap();

        loop {
            let div_info = js.recv().await.unwrap();
            let parsed = serde_json::from_value::<ResizeObserverEntry>(div_info).unwrap();
            callback.write().call_mut((parsed,));
        }
    })
}

pub fn use_div_size(id: String) -> DomRectSignal {
    let id = use_signal(|| id);
    let mut signal = use_signal(|| None);

    let _collector = use_future(move || async move {
        wait_for_next_render().await;

        let mut js = resize_observer();

        js.send(id.read().clone().into()).unwrap();

        loop {
            let div_info = js.recv().await.unwrap();
            let parsed = serde_json::from_value::<ResizeObserverEntry>(div_info).unwrap();
            *signal.write() = Some(parsed);
        }
    });

    DomRectSignal {
        inner: signal,
        _collector,
    }
}
