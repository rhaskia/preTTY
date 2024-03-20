use dioxus::{signals::Signal, hooks::{UseFuture, use_signal}, events::UseEval};
use dioxus::prelude::*;
use serde::Deserialize;
use std::fmt::Debug;

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct DomRect {
    pub border_box: f32, 
    pub content_box: f32,
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
    let mut signal = use_signal(|| DomRect { border_box: 0.0, content_box: 0.0 });

    let mut js = eval(
        r#"
        function waitForElm(selector) {
            return new Promise(resolve => {
                if (document.getElementById(selector)) {
                return resolve(document.getElementById(selector));
            }

            const observer = new MutationObserver(mutations => {
                if (document.getElementById(selector)) {
                    observer.disconnect();
                    resolve(document.querySelector(selector));
                }
            });

            // If you get "parameter 1 is not of type 'Node'" error, see https://stackoverflow.com/a/77855838/492336
            observer.observe(document.body, {
                childList: true,
                subtree: true
            });
        });
        }

        let id = await dioxus.recv();
        
        waitForElm('split-0').then((el) => {
            console.log('Element is ready');
            console.log(el);
            let div = document.getElementById(id);
            const ro = new ResizeObserver(entries => {
                for (let entry of entries) {
                    dioxus.send({ border_box: entry.borderBoxSize,
                                  content_box: entry.contentBoxSize });      
                }
            });
            ro.observe(div);
        });

        "#,
    );

    js.send(id.into()).unwrap();

    let collector = use_future(move || async move {
        println!("what the hell");
        loop {
            let div_info = js.recv().await.unwrap();
            println!("{div_info:?}");
            let parsed = serde_json::from_value::<DomRect>(div_info).unwrap();
            *signal.write() = parsed;
        }
    });

    DomRectSignal { inner: signal, js, collector }
}
