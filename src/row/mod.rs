pub mod types;

use std::rc::{ Rc, Weak };
use std::cell::{
    RefCell,
    Ref
};
use crate::cell::{
    RcCell,
    AnyTypeCell
};

pub struct Row {
    cells: RefCell<Vec<Weak<AnyTypeCell>>>,
    pub index: usize,
}

impl Row {
    pub fn new(index: usize) -> RcRow {
        Rc::new(RefCell::new(Self {
            cells: RefCell::new(vec![]),
            index
        }))
    }

    pub fn add_cell(&mut self, cell: &RcCell) {
        self.cells.borrow_mut().push(Rc::downgrade(cell));
    }

    pub fn drop_cell(&mut self, cell: &RcCell) {
        self.cells.borrow_mut().retain(|c| c.upgrade().unwrap() != cell.clone());
    }

    pub fn get_cells(&self) -> Ref<Vec<Weak<AnyTypeCell>>> {
        self.cells.borrow()
    }
}

impl PartialEq for Row {
    fn eq(&self, other: &Self) -> bool {
        self.index == other.index
    }
}
impl Eq for Row {}

pub type RcRow = Rc<RefCell<Row>>;