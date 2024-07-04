use std::ops::{Deref, DerefMut, Index, IndexMut};

use super::cell::Cell;

#[derive(Clone, Debug, Default)]
pub struct Line {
    cells: Vec<Cell>,
    pub width: bool,
    pub height: bool,
}

impl Line {
    pub fn new() -> Self {
        Line {
            cells: Vec::new(),
            width: false,
            height: false,
        }
    }

    pub fn with_one() -> Self {
        Line {
            cells: vec![Cell::default()],
            width: false,
            height: false,
        }
    }

    pub fn set(&mut self, cells: Vec<Cell>) { self.cells = cells; }

    pub fn set_width(&mut self, double: bool) { self.width = double }
    pub fn set_height(&mut self, double: bool) { self.height = double }
    pub fn set_double(&mut self, double: bool) {
        self.height = double;
        self.width = double;
    }

    pub fn double_width(&self) -> bool { self.width && !self.height }
    pub fn double_height(&self) -> bool { !self.width && self.height }
    pub fn double_size(&self) -> bool { self.width && self.height }
}

impl Index<usize> for Line {
    type Output = Cell;

    // Define how indexing operation should behave
    fn index(&self, index: usize) -> &Self::Output { &self.cells[index] }
}

impl IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { &mut self.cells[index] }
}

impl Deref for Line {
    type Target = Vec<Cell>;

    fn deref(&self) -> &Self::Target { &self.cells }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.cells }
}
