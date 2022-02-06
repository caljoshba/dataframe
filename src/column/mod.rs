#![allow(unused_assignments)]

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

pub struct RollingMean {
    should_calculate: bool,
    mean_over: usize
}

impl RollingMean {
    pub fn new(mut should_calculate: bool, mean_over_option: Option<usize>) -> Self {
        let mut mean_over = 0usize;
        if let Some(mean) = mean_over_option {
            mean_over = mean;
        } else if should_calculate {
            should_calculate = false; 
        }
        Self {
            should_calculate,
            mean_over
        }
    }
}

pub struct Column {
    cells: RefCell<Vec<RcCell>>,
    grouped_values: HashMap<AnyType, RefCell<Vec<RcCell>>>,
    pub name: &'static str,
    pub rolling_mean: RollingMean,
}

impl Column {
    pub fn new(name: &'static str, rolling_mean: RollingMean) -> Self {
        Self {
            cells: RefCell::new(vec![]),
            grouped_values: HashMap::new(),
            name,
            rolling_mean,
        }
    }

    pub fn get_cells(&self) -> Ref<Vec<RcCell>> {
        self.cells.borrow()
    }

    pub fn add_cell(&mut self, cell: &RcCell) {
        self.add_to_grouped_values(cell);
        self.cells.borrow_mut().push(Rc::clone(cell));
        if self.rolling_mean.should_calculate {
            self.cell_rolling_mean(self.rolling_mean.mean_over, cell);
        }
    }

    pub fn drop_cell(&mut self, cell: RcCell) {
        self.remove_from_grouped_values(&cell);
        let mut cells = self.cells.borrow_mut();
        let cell_location = cell.borrow().get_row().borrow().index;
        cells.remove(cell_location);
        self.recalculate_rolling_means(&cell_location);
    }

    fn recalculate_rolling_means(&self, cell_location: &usize) {
        if self.rolling_mean.should_calculate {
            let cells_to_recalculate = &self.cells.borrow()[*cell_location..(cell_location + self.rolling_mean.mean_over - 1)];
            for cell in cells_to_recalculate.iter() {
                self.cell_rolling_mean(self.rolling_mean.mean_over, cell);
            }
        }
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

    pub fn update_rolling_mean(&mut self, rolling_mean: RollingMean) {
        if rolling_mean.should_calculate != self.rolling_mean.should_calculate || rolling_mean.mean_over != self.rolling_mean.mean_over {
            self.rolling_mean = rolling_mean;
        }

        for cell in self.cells.borrow().iter() {
            if self.rolling_mean.should_calculate {
                self.cell_rolling_mean(self.rolling_mean.mean_over, &cell);
            } else {
                cell.borrow_mut().set_rolling_mean(None);
            }            
        }
        
    }

    // pub fn rolling_mean(&self, mean_over: usize) -> Vec<AnyType> {
    //     let cells = self.cells.borrow();
    //     if cells.len() < mean_over {
    //         return vec![AnyType::Null; cells.len()];
    //     }

    //     let mut rolling_mean_values: Vec<AnyType> = vec![];

    //     for slice in cells.windows(mean_over) {
    //         if slice[0].borrow().get_value() == &AnyType::Null {
    //             rolling_mean_values.push(AnyType::Null);
    //         } else {
    //             let value: Vec<AnyType> = self.sum_slice_values(slice);
    //             rolling_mean_values.push(value[0] / value[1]);
    //         }            
    //     }

    //     rolling_mean_values
    // }

    pub fn cell_rolling_mean(&self, mean_over: usize, cell: &RcCell) -> Option<AnyType> {
        let mut rolling_mean = None;
        let cells = &self.cells.borrow();
        if cells.len() < mean_over {
            rolling_mean = None;
        } else {
            let cell_location = cell.borrow().get_row().borrow().index;
            let column_slice = &cells[(cell_location - (mean_over - 1))..=cell_location];
            let value: Vec<AnyType> = self.sum_slice_values(column_slice);
            rolling_mean = Some(value[0] / value[1]);
        }
        cell.borrow_mut().set_rolling_mean(rolling_mean);
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

    pub fn get_difference_to_last(&self) -> Vec<AnyType> {
        let mut differences: Vec<AnyType> = vec![];
        let cells = self.cells.borrow();
        for (index, cell) in cells.iter().enumerate() {
            if index == 0 {
                differences.push(AnyType::Null);
                continue;
            }

            let previous_value = cells[index - 1].borrow().get_value().clone();
            let current_value = cell.borrow().get_value().clone();

            differences.push((current_value - previous_value).into());
        }
        differences
    }
}