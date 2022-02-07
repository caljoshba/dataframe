pub mod types;

use crate::row::{
    RcRow
};
use std::cell::RefCell;
use types::datatypes::AnyType;
use std::rc::{ Rc };

pub struct Cell {
    value: AnyType,
    row: RcRow,
    column_name: &'static str,
    rolling_mean: Option<AnyType>,
}

impl Cell {
    pub fn new(value: AnyType, row: &RcRow, column_name: &'static str) -> RcCell {
        Rc::new(RefCell::new(Self {
            value: value,
            row: Rc::clone(&row),
            column_name,
            rolling_mean: None,
        }))
    }

    pub fn get_row(&self) -> &RcRow {
        &self.row
    }

    pub fn get_value(&self) -> &AnyType {
        &self.value
    }

    pub fn clone_row(&self) -> RcRow {
        Rc::clone(&self.row)
    }

    pub fn set_rolling_mean(&mut self, rolling_mean: Option<AnyType>) {
        self.rolling_mean = rolling_mean;
    }

    pub fn get_rolling_mean(&self) -> Option<AnyType> {
        self.rolling_mean
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.row.borrow().index == other.row.borrow().index
    }
}
impl Eq for Cell {}

pub type AnyTypeCell = RefCell<Cell>;
pub type RcCell = Rc<AnyTypeCell>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::row::{
        Row,
        RcRow
    };
    #[test]
    fn assign_row() {
        let value: AnyType = 67u16.into();
        let row: RcRow = Row::new(3);
        let cell: RcCell = Cell::new(value, &row, "timmeh");

        assert!(&row == cell.borrow().get_row());
    }

    #[test]
    fn clone_row() {
        let value: AnyType = 67u16.into();
        let row: RcRow = Row::new(3);
        let cell: RcCell = Cell::new(value, &row, "timmeh");

        assert!(row == cell.borrow().clone_row());
    }

    #[test]
    fn get_value() {
        let value: AnyType = 67u16.into();
        let row: RcRow = Row::new(3);
        let cell: RcCell = Cell::new(value, &row, "timmeh");

        assert!(&value == cell.borrow().get_value());
    }
}