use super::cell::Cell;
use std::ops::{Index, IndexMut, Deref, DerefMut};

#[derive(Clone, Debug)]
pub struct Line {
    cells: Vec<Cell>
}

impl Line {
    pub fn new() -> Self {
        Line { cells: Vec::new() }
    }

    pub fn with_one() -> Self {
        Line { cells: vec![Cell::default()] }
    }
    
    pub fn set(&mut self, cells: Vec<Cell>) { self.cells = cells; }
}


impl Index<usize> for Line {
    type Output = Cell;

    // Define how indexing operation should behave
    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl IndexMut<usize> for Line {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl Deref for Line {
    type Target = Vec<Cell>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl DerefMut for Line {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}
