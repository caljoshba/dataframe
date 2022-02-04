pub mod types;

use crate::cell::{
    RcCell
};

use std::cell::{
    RefCell,
    Ref
};

pub struct Column {
    cells: RefCell<Vec<RcCell>>,
    pub name: &'static str,
}

impl Column {
    pub fn new(name: &'static str) -> Self {
        Self {
            cells: RefCell::new(vec![]),
            name
        }
    }

    pub fn get_cells(&self) -> Ref<Vec<RcCell>> {
        self.cells.borrow()
    }

    pub fn add_cell(&mut self, cell: RcCell) {
        self.cells.borrow_mut().push(cell);
    }

    pub fn drop_cell(&mut self, cell: RcCell) {
        self.cells.borrow_mut().retain(|c| c.clone() != cell);
    }
}