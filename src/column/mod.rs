pub mod types;

use crate::cell::{
    RcCell,
    types::datatypes::{
        AnyType,
    },
};
use std::cell::{
    RefCell,
    Ref,
};
use std::rc::{ Rc };
use std::collections::HashMap;

pub struct Column {
    cells: RefCell<Vec<RcCell>>,
    pub name: &'static str,
    grouped_values: HashMap<AnyType, RefCell<Vec<RcCell>>>
}

impl Column {
    pub fn new(name: &'static str) -> Self {
        Self {
            cells: RefCell::new(vec![]),
            name,
            grouped_values: HashMap::new()
        }
    }

    pub fn get_cells(&self) -> Ref<Vec<RcCell>> {
        self.cells.borrow()
    }

    pub fn add_cell(&mut self, cell: RcCell) {
        self.add_to_grouped_values(&cell);
        self.cells.borrow_mut().push(cell);
    }

    pub fn drop_cell(&mut self, cell: RcCell) {
        self.remove_from_grouped_values(&cell);
        self.cells.borrow_mut().retain(|c| *c != cell);
    }

    fn add_to_grouped_values(&mut self, cell: &RcCell) {
        if let Some(entry) = self.grouped_values.get_mut(cell.borrow().get_value()) {
            entry.borrow_mut().push(Rc::clone(cell));
        } else {
            self.grouped_values.insert(cell.borrow().get_value().clone(), RefCell::new(vec![Rc::clone(cell)]));
        }
    }

    fn remove_from_grouped_values(& mut self, cell: &RcCell) {
        if let Some(entry) = self.grouped_values.get_mut(cell.borrow().get_value()) {
            entry.borrow_mut().retain(|c| c != cell);
        }
    }

    pub fn mean(&self) -> Option<AnyType> {
        let cells = self.cells.borrow();
        let cell = cells[0].borrow();
        let value = cell.get_value();
        match value {
            AnyType::Null => None,
            AnyType::Boolean(_) => types::bool::mean(self),
            AnyType::Float32(_) => types::f32::mean(self),
            AnyType::Float64(_) => types::f64::mean(self),
            AnyType::Int8(_) => types::i8::mean(self),
            AnyType::Int16(_) => types::i16::mean(self),
            AnyType::Int32(_) => types::i32::mean(self),
            AnyType::Int64(_) => types::i64::mean(self),
            AnyType::UInt8(_) => types::u8::mean(self),
            AnyType::UInt16(_) => types::u16::mean(self),
            AnyType::UInt32(_) => types::u32::mean(self),
            AnyType::UInt64(_) => types::u64::mean(self),
            AnyType::Utf8(_) => types::utf8::mean(self),
        }
    }
}

pub trait GroupValues {
    fn add_to_grouped_values(&mut self, cell: RcCell) -> HashMap<AnyType, RefCell<Vec<RcCell>>>;
}