use termwiz::{cell::{Blink, Intensity, Underline}, color::ColorSpec, escape::csi::Sgr};

pub struct TerminalRenderer {
    pub screen: Screen,
    pub alt_screen: AltScreen,

    pub attr: CellAttributes,
}

impl TerminalRenderer {
    pub fn new(size: (u32, u32)) -> TerminalRenderer {
        TerminalRenderer {
            screen: Screen::new(),
            alt_screen: AltScreen::new(size),
            attr: CellAttributes::default(),
        }
    }

    pub fn reset_attr(&mut self) {
        self.attr = CellAttributes::default();
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

#[derive(Clone)]
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
#[derive(Clone)]
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
    pub cells: Vec<Cell>,
}

impl Screen {
    pub fn push(&mut self, c: Cell) {
        self.cells.push(c);
    }

    pub fn new() -> Screen {
        Screen { cells: Vec::new() }
    }
}

pub struct AltScreen {
    pub screen: Vec<Vec<Cell>>,
}

impl AltScreen {
    pub fn new(size: (u32, u32)) -> AltScreen {
        AltScreen {
            screen: create_empty_screen(size),
        }
    }
}

fn create_empty_screen(size: (u32, u32)) -> Vec<Vec<Cell>> {
    (0..size.1)
        .map(|_| {
            (0..size.0)
                .map(|_| Cell::new(' ', CellAttributes::default()))
                .collect()
        })
        .collect()
}
