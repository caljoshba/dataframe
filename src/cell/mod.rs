pub mod types;

use crate::row::{
    RcRow
};
use std::cell::RefCell;
use types::datatypes::AnyType;
use std::rc::{ Rc };

// #[derive(Display)]
pub struct Cell {
    pub value: AnyType,
    row: RcRow
}

impl Cell {
    pub fn new(value: AnyType, row: &RcRow) -> RcCell {
        Rc::new(RefCell::new(Self {
            value: value,
            row: Rc::clone(&row)
        }))
    }
}

pub type AnyTypeCell = RefCell<Cell>;
pub type RcCell = Rc<AnyTypeCell>;
