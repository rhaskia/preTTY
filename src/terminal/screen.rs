mod cell;
mod command;

use cell::{Cell, CellAttributes};
use termwiz::escape::csi::Sgr;

#[derive(Debug)]
pub struct TerminalRenderer {
    pub screen: Screen,
    pub alt_screen: Screen,

    pub attr: CellAttributes,
}

impl TerminalRenderer {
    pub fn new() -> TerminalRenderer {
        TerminalRenderer {
            screen: Screen::new(),
            alt_screen: Screen::new(),
            attr: CellAttributes::default(),
        }
    }

    pub fn reset_attr(&mut self) {
        self.attr = CellAttributes::default();
    }

    pub fn get_screen(&self, alt: bool) -> &Screen {
        if alt {
            &self.alt_screen
        } else {
            &self.screen
        }
    }

    pub fn mut_screen(&mut self, alt: bool) -> &mut Screen {
        if alt {
            &mut self.alt_screen
        } else {
            &mut self.screen
        }
    }

    pub fn handle_sgr(&mut self, sgr: Sgr) {
        match sgr {
            Sgr::Foreground(f) => self.attr.fg = f,
            Sgr::Background(b) => self.attr.bg = b,
            Sgr::Reset => self.reset_attr(),
            Sgr::Blink(b) => self.attr.blink = b,
            Sgr::Underline(u) => self.attr.underline = u,
            Sgr::Intensity(i) => self.attr.intensity = i,
            Sgr::UnderlineColor(colour) => self.attr.underline_fg = colour,
            Sgr::Italic(i) => self.attr.italic = i,
            Sgr::StrikeThrough(s) => self.attr.strikethrough = s,
            _ => println!("{:?}", sgr),
        }
    }
}

#[derive(Debug)]
pub struct Screen {
    pub cells: Vec<Vec<Cell>>,
}

impl Screen {
    pub fn push(&mut self, c: Cell, cursorx: usize, cursory: usize) {
        if cursory >= self.cells.len() {
            let extend_amount = cursory - &self.cells.len();
            self.cells
                .extend(vec![vec![Cell::default()]; extend_amount + 1])
        }

        if cursorx >= self.cells[cursory].len() {
            let extend_amount = cursorx - &self.cells[cursory].len();
            self.cells[cursory].extend(vec![Cell::default(); extend_amount + 1])
        }

        // TODO: add extra if cursor out of index
        self.cells[cursory][cursorx] = c;
    }

    pub fn new() -> Screen {
        Screen { cells: Vec::new() }
    }
}
