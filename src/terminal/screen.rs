use std::collections::VecDeque;
use std::ops::Range;

use super::{
    cell::{Cell, CellAttributes},
    command::{CommandSlice, CommandSlicer},
};
use termwiz::escape::csi::Sgr;

#[derive(Debug)]
pub struct TerminalRenderer {
    pub screen: Screen,
    pub alt_screen: Screen,
    pub commands: CommandSlicer,

    pub attr: CellAttributes,
}

impl TerminalRenderer {
    pub fn new(rows: usize, cols: usize) -> TerminalRenderer {
        TerminalRenderer {
            screen: Screen::new(rows, cols, true),
            alt_screen: Screen::new(rows, cols, false),
            attr: CellAttributes::default(),
            commands: CommandSlicer::new(),
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

pub type Line = Vec<Cell>;

#[derive(Debug)]
pub struct Screen {
    pub cells: VecDeque<Line>,
    max_scrollback: usize,
    scrollback_offset: usize,

    physical_rows: usize,
    physical_columns: usize,

    scrollback_allowed: bool,
}

impl Screen {
    pub fn new(rows: usize, columns: usize, sc_allow: bool) -> Screen {
        Screen {
            cells: VecDeque::new(),
            max_scrollback: 100,
            physical_rows: rows,
            physical_columns: columns,
            scrollback_allowed: sc_allow,
            scrollback_offset: 0,
        }
    }

    /// Scrolls a line out of the visible screen
    pub fn scrollback(&mut self) {
        self.scrollback_offset += 1;
    }

    /// If the screen has the ability to use scrollback
    pub fn can_scroll(&self) -> bool {
        self.scrollback_allowed
    }

    /// Pushes a cell at a certain cursor location
    /// Manages any scrollback on its own
    pub fn push(&mut self, cell: Cell, cursor_x: usize, cursor_y: usize) {
        let cursor_y = self.visible_start() + cursor_y;
        self.ensure_lines(cursor_y);

        if cursor_x >= self.cells[cursor_y].len() {
            let extend_amount = cursor_x - &self.cells[cursor_y].len();
            self.cells[cursor_y].extend(vec![Cell::default(); extend_amount + 1])
        }

        // TODO: add extra if cursor out of index
        self.cells[cursor_y][cursor_x] = cell;
    }

    /// Extends the cell lines if there are not enough
    pub fn ensure_lines(&mut self, index: usize) {
        if index >= self.cells.len() {
            let extend_amount = index - &self.cells.len();
            self.cells
                .extend(vec![vec![Cell::default()]; extend_amount + 1]);
        }
    }

    /// Bad bad bad bad
    pub fn scroll_range(&self, back: usize) -> Range<usize> {
        self.scrollback_offset..self.cells.len()
    }

    /// Length of whole scrollback
    pub fn scrollback_len(&self) -> usize {
        self.cells.len()
    }

    /// Sets a cell at a position within the visible screen
    pub fn cell(&self, x: usize, y: usize) -> Cell {
        let vis_y = self.visible_start() + y;
        self.cells[vis_y][x].clone()
    }

    /// Erases scrollback and visible screen
    pub fn erase_all(&mut self) {
        self.cells = VecDeque::new();
    }

    /// Length of the visible screen
    pub fn len(&self) -> usize {
        self.cells.len()
    }

    // Mutable reference to a line within the visible screen
    pub fn mut_line(&mut self, index: usize) -> &mut Line {
        let vis_index = self.visible_start() + index;
        self.ensure_lines(vis_index);
        &mut self.cells[vis_index]
    }

    pub fn phys_line(&self, index: usize) -> usize {
    self.visible_start() + index
    }

    /// Reference to a line within the visible screen
    pub fn line(&self, index: usize) -> &Line {
        let vis_index = self.visible_start() + index;
        &self.cells[index]
    }

    /// Pushes a new line onto the screen
    pub fn new_line(&mut self) {
        self.cells.push_back(vec![Cell::default()]);
        let len = self.cells.len();
        if len > self.max_scrollback {
            self.cells.drain(..len - self.max_scrollback);
        }
    }

    /// Sets the value of a Line on the visible screen
    pub fn set_line(&mut self, index: usize, line: Line) {
        let vis_index = self.visible_start() + index;
        self.cells[vis_index] = line;
    }

    /// The index at which the visible screen starts in the scrollback buffer
    pub fn visible_start(&self) -> usize {
        self.scrollback_offset
    }

    /// Whether the visible screen is filled
    /// Used for knowing whether to move the cursor or not
    pub fn is_filled(&self) -> bool {
        self.cells.len() > self.physical_rows
    }
}
