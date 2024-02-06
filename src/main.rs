use input::key_event_to_str;
use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::{
    io::{Read, Write},
    sync::mpsc::{Receiver, Sender},
};

use std::{sync::mpsc::channel, thread};

// crate imports
mod input;
mod render;

use render::TextRenderer;

fn read_and_send_chars(mut reader: Box<dyn Read + Send>, tx: Sender<char>) {
    let mut buffer = [0u8; 1]; // Buffer to hold a single character

    loop {
        match reader.read(&mut buffer) {
            Ok(_) => {
                let char = buffer[0] as char;
                tx.send(char).unwrap();
            }
            Err(err) => {
                eprintln!("Error reading from Read object: {}", err);
                break;
            }
        }
    }
}

use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime};
mod utils;
use utils::WgpuUtils;
use wgpu_text::glyph_brush::{BuiltInLineBreaker, Layout, OwnedText, Section, Text, VerticalAlign};
use wgpu_text::BrushBuilder;
use winit::event::{Event, KeyEvent, MouseScrollDelta};
use winit::event_loop::{self, ControlFlow};
use winit::{
    event::{ElementState, WindowEvent},
    window::WindowBuilder,
};

// TODO text layout of characters like 'š, ć, ž, đ' doesn't work correctly.
fn main() -> anyhow::Result<()> {
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "error");
    }
    env_logger::init();

    // Send data to the pty by writing to the master
    let mut pty_system = native_pty_system();

    // Create a new pty
    let pair = pty_system.openpty(PtySize {
        rows: 24,
        cols: 80,
        // Not all systems support pixel_width, pixel_height,
        // but it is good practice to set it to something
        // that matches the size of the selected font.  That
        // is more complex than can be shown here in this
        // brief example though!
        pixel_width: 0,
        pixel_height: 0,
    })?;

    // Spawn a shell into the pty
    let cmd = CommandBuilder::new("bash");
    let child = pair.slave.spawn_command(cmd)?;

    // Read and parse output from the pty with reader
    let mut reader = pair.master.try_clone_reader()?;
    let mut writer = pair.master.take_writer()?;

    let (tx, rx) = channel();

    thread::spawn(move || {
        read_and_send_chars(reader, tx);
    });

    let event_loop = event_loop::EventLoop::new().unwrap();
    let window = WindowBuilder::new()
        .with_title("wgpu-text: 'simple' example")
        .build(&event_loop)
        .unwrap();
    let window = Arc::new(window);

    let mut text_renderer = TextRenderer::new(window.clone());

    // All wgpu-text related below:

    // change '60.0' if you want different FPS cap
    let target_framerate = Duration::from_secs_f64(1.0 / 60.0);
    let mut delta_time = Instant::now();

    event_loop
        .run(move |event, elwt| {

                        loop {
                            match rx.try_recv() {
                                Ok(c) => text_renderer.push_text(c.to_string()),
                                Err(_) => break,
                            }
                        }
            
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
                        // TODO: resize pty
                        // You can also do this!
                        // brush.update_matrix(wgpu_text::ortho(config.width, config.height), &queue);
                    }
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::KeyboardInput {
                        event:
                            KeyEvent {
                                logical_key,
                                state: ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        writer.write_all(key_event_to_str(logical_key).as_bytes());
                    }
                    // WindowEvent::MouseWheel {
                    //     delta: MouseScrollDelta::LineDelta(_, y),
                    //     ..
                    // } => {
                    //     // increase/decrease font size
                    //     let mut size = font_size;
                    //     if y > 0.0 {
                    //         size += (size / 4.0).max(2.0)
                    //     } else {
                    //         size *= 4.0 / 5.0
                    //     };
                    //     font_size = (size.max(3.0).min(25000.0) * 2.0).round() / 2.0;
                    // }
                    WindowEvent::RedrawRequested => {
                        text_renderer.render();
                    }
                    _ => (),
                },
                _ => (),
            }
        })
        .unwrap();

    Ok(())
}
