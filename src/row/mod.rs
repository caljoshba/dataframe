pub mod types;

use std::rc::{ Rc, Weak };
use std::cell::RefCell;
use crate::cell::{
    RcCell,
    AnyTypeCell
};

pub struct Row {
    cells: RefCell<Vec<Weak<AnyTypeCell>>>
}

impl Row {
    pub fn new() -> RcRow {
        Rc::new(RefCell::new(Self {
            cells: RefCell::new(vec![])
        }))
    }

    pub fn add_cell(&mut self, cell: &RcCell) {
        self.cells.borrow_mut().push(Rc::downgrade(cell));
    }
}

pub type RcRow = Rc<RefCell<Row>>;