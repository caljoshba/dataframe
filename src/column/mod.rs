pub mod types;

use crate::cell::{
    RcCell
};

use std::cell::RefCell;

pub struct Column {
    pub cells: RefCell<Vec<RcCell>>,
    pub name: &'static str,
}

impl Column {
    pub fn new(name: &'static str) -> Self {
        Self {
            cells: RefCell::new(vec![]),
            name
        }
    }

    pub fn add_cell(&mut self, cell: RcCell) {
        self.cells.borrow_mut().push(cell);
    }
}