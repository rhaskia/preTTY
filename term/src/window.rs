use termwiz::escape::csi::Window;

/// Trait for handling "window" specific ANSI commands
/// Nothing here is needed at all, but allows for it to
/// be implemented if you wish
pub trait WindowHandler {
    fn csi_window(&mut self, _: Box<Window>) {}
    fn send_notification(&mut self, _: String) {}
    fn bell(&mut self) {}
    fn send_title(&mut self) {}
    fn steal_focus(&mut self) {}
}

impl WindowHandler for () {}
