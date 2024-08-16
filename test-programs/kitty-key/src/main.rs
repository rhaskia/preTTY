use std::io::{stdout, Write};

use crossterm::event::{
    KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags, read
};
use crossterm::execute;
use crossterm::event::Event;

fn main() {
    let mut stdout = stdout();

    execute!(
        stdout,
        PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES)
    ).unwrap();
    
    loop {
        // `read()` blocks until an `Event` is available
        match read().unwrap() {
            Event::Key(event) => match event {
                _ => println!("{:?}", event),
            } 
            _ => {}
        }
    }

    execute!(stdout, PopKeyboardEnhancementFlags).unwrap();
}