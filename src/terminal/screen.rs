use termwiz::{
    cell::{Blink, Intensity, Underline},
    color::ColorSpec,
    escape::csi::Sgr,
};

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

    pub fn get_screen(&mut self, alt: bool) -> &mut Screen {
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

#[derive(Clone, Debug)]
pub struct CellAttributes {
    pub bg: ColorSpec,
    pub fg: ColorSpec,
    // TODO bitmap these
    pub underline: Underline,
    pub intensity: Intensity,
    pub italic: bool,
    pub strikethrough: bool,
    pub blink: Blink,
    pub underline_fg: ColorSpec,
}

impl CellAttributes {
    pub fn default() -> CellAttributes {
        CellAttributes {
            bg: ColorSpec::Default,
            fg: ColorSpec::Default,
            underline: Underline::None,
            intensity: Intensity::Normal,
            italic: false,
            strikethrough: false,
            blink: Blink::None,
            underline_fg: ColorSpec::Default,
        }
    }
}

// Change to enum to allow for box drawing etc
#[derive(Clone, Debug)]
pub struct Cell {
    pub char: char,
    pub attr: CellAttributes,
}

impl Cell {
    pub fn new(char: char, attr: CellAttributes) -> Cell {
        Cell { char, attr }
    }
}

pub struct Screen {
    pub cells: Vec<Vec<Cell>>,
}

impl Screen {
    pub fn push(&mut self, c: Cell, cursorx: usize, cursory: usize) {
        if cursory as usize >= self.cells.len() { self.cells.push(Vec::new()) } 

        println!("{:?}", c);

        self.cells[cursory as usize].insert(cursorx as usize, c)
    }

    pub fn new() -> Screen {
        Screen { cells: Vec::new() }
    }
}
