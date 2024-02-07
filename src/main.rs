#![feature(if_let_guard)]

// crate imports
mod app;
mod input;
mod palette;
mod render;
mod terminal;
mod utils;

use app::App;
use terminal::Terminal;
use input::InputManager;

use crate::render::WGPUColor;
use std::{
    io::{Read, Write},
    sync::mpsc::{Receiver, Sender},
};

use render::TextRenderer;

use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::event_loop::{self, ControlFlow};
use winit::{
    event::{ElementState, WindowEvent},
    window::WindowBuilder,
};
use winit::{
    event::{Event, KeyEvent},
    keyboard::{KeyCode, ModifiersKeyState},
};

// TODO text layout of characters like 'š, ć, ž, đ' doesn't work correctly.
fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }
    env_logger::init();

    let event_loop = event_loop::EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("wgpu-text: 'simple' example")
        .build(&event_loop)
        .unwrap();
    let window = Arc::new(window);

    let mut app = App::setup(window.clone());

    let mut text_renderer = TextRenderer::new(window.clone());
    let mut input_manager = InputManager::new();

    // All wgpu-text related below:

    // change '60.0' if you want different FPS cap
    let target_framerate = Duration::from_secs_f64(1.0 / 60.0);
    let mut delta_time = Instant::now();

    event_loop
        .run(move |event, elwt| {
            app.update();

            match event {
                Event::LoopExiting => println!("Exiting!"),
                Event::NewEvents(_) => {
                    if target_framerate <= delta_time.elapsed() {
                        window.request_redraw();
                        delta_time = Instant::now();
                    } else {
                        elwt.set_control_flow(ControlFlow::WaitUntil(
                            Instant::now().checked_sub(delta_time.elapsed()).unwrap()
                                + target_framerate,
                        ));
                    }
                }
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(new_size) => {
                        text_renderer.resize_view(new_size);
                        // pair.master.resize(PtySize { rows:
                        //     , cols: (), pixel_width: (), pixel_height: () })

                        // TODO: resize pty
                        // You can also do this!
                        // brush.update_matrix(wgpu_text::ortho(config.width, config.height), &queue);
                    }
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput { event, .. } => app.handle_input(event),
                    WindowEvent::RedrawRequested => text_renderer.render(),

                    _ => (),
                },
                _ => (),
            }
        })
        .unwrap();

    Ok(())
}
