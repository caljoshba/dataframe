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
            AnyType::ISize(_) => types::isize::mean(self),
            AnyType::UInt8(_) => types::u8::mean(self),
            AnyType::UInt16(_) => types::u16::mean(self),
            AnyType::UInt32(_) => types::u32::mean(self),
            AnyType::UInt64(_) => types::u64::mean(self),
            AnyType::USize(_) => types::usize::mean(self),
            AnyType::Utf8(_) => types::utf8::mean(self),
        }
    }

    pub fn rolling_mean(&self, mean_over: usize) -> Vec<AnyType> {
        let cells = self.cells.borrow();
        if cells.len() < mean_over {
            return vec![AnyType::Null; cells.len()];
        }

        let mut rolling_mean_values: Vec<AnyType> = vec![];

        for slice in cells.windows(mean_over) {
            if slice[0].borrow().get_value() == &AnyType::Null {
                rolling_mean_values.push(AnyType::Null);
            } else {
                let value: Vec<AnyType> = self.sum_slice_values(slice);
                rolling_mean_values.push(value[0] / value[1]);
            }            
        }

        rolling_mean_values
    }

    pub fn cell_rolling_mean(&self, mean_over: usize, cell: &RcCell) -> AnyType {
        let mut rolling_mean = AnyType::Null;
        let cells = &self.cells.borrow();
        if cells.len() < mean_over {
            return rolling_mean;
        } else if let Some(cell_location) = &cells.iter().rposition(|c| c == cell) {
            let column_slice = &cells[(cell_location - (mean_over - 1))..=*cell_location];
            let value: Vec<AnyType> = self.sum_slice_values(column_slice);
            rolling_mean = value[0] / value[1];
        }
        rolling_mean        
    }

    fn sum_slice_values(&self, slice: &[RcCell]) -> Vec<AnyType> {
        slice.iter().fold(vec![AnyType::Null, 0usize.into()], |mut acc, x| if x.borrow().get_value() != &AnyType::Null {
            acc[0] = acc[0] + *x.borrow().get_value();
            acc[1] = acc[1] + 1usize.into();
            acc
        } else {
            acc
        })
    }
}