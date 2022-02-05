pub mod types;

use crate::row::{
    RcRow
};
use std::cell::RefCell;
use types::datatypes::AnyType;
use std::rc::{ Rc };

// #[derive(Display)]
pub struct Cell {
    value: AnyType,
    row: RcRow,
    column_name: &'static str,
}

impl Cell {
    pub fn new(value: AnyType, row: &RcRow, column_name: &'static str) -> RcCell {
        Rc::new(RefCell::new(Self {
            value: value,
            row: Rc::clone(&row),
            column_name
        }))
    }

    fn get_row(&self) -> RcRow {
        Rc::clone(&self.row)
    }

    pub fn get_value(&self) -> &AnyType {
        &self.value
    }

    pub fn clone_row(&self) -> RcRow {
        Rc::clone(&self.row)
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.row.borrow().index == other.row.borrow().index
    }
}
impl Eq for Cell {}

// pub trait RollingMean {
//     fn rolling_mean(&self, mean_over: usize) -> Option<AnyType>;
//     fn sum_values(&self, total_values: usize) -> Option<AnyType>;
// }

pub type AnyTypeCell = RefCell<Cell>;
pub type RcCell = Rc<AnyTypeCell>;
