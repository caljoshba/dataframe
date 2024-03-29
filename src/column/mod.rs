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
use chrono::{
    DateTime,
    Utc
};

#[derive(Debug)]
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

#[derive(Debug)]
pub struct Returns {
    pub should_calculate: bool,
    pub column_name: Option<&'static str>
}

impl Returns {
    pub fn new(should_calculate: bool, column_name: Option<&'static str>) -> Self {
        Self {
            should_calculate,
            column_name
        }
    } 
}

#[derive(Debug)]
pub struct Column {
    cells: RefCell<Vec<RcCell>>,
    grouped_values: HashMap<AnyType, RefCell<Vec<RcCell>>>,
    pub name: &'static str,
    pub rolling_mean: RollingMean,
    pub returns: Returns
}

impl Column {
    pub fn new(name: &'static str, rolling_mean: RollingMean, returns: Returns) -> Self {
        Self {
            cells: RefCell::new(vec![]),
            grouped_values: HashMap::new(),
            name,
            rolling_mean,
            returns
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
        // drop before calling recalculate so we can free the vec which needs to be borrowed later on
        drop(cells);
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

    pub fn get_grouped_values(&self, value: AnyType) -> Option<&RefCell<Vec<RcCell>>> {
        self.grouped_values.get(&value)
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

    pub fn update_returns(&mut self, returns: Returns) -> Option<Vec<AnyType>> {
        if returns.should_calculate != self.returns.should_calculate {
            self.returns = returns;
        }

        if self.returns.should_calculate {
            let differences = self.get_all_difference_to_last();
            return Some(differences);
        } else {
            return None;
        }
        
    }

    pub fn cell_rolling_mean(&self, mean_over: usize, cell: &RcCell) -> Option<AnyType> {
        let mut rolling_mean = None;
        let cells = &self.cells.borrow();
        let cell_location = cell.borrow().get_row().borrow().index;
        if cells.len() < mean_over || cell_location == 0 {
            rolling_mean = None;
        } else {            
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

    pub fn get_all_difference_to_last(&self) -> Vec<AnyType> {
        let mut differences: Vec<AnyType> = vec![];
        let cells = self.cells.borrow();
        for (index, _) in cells.iter().enumerate() {
            let value = self.get_difference_to_last(index);
            differences.push(value);
        }
        differences
    }

    pub fn get_difference_to_last(&self, index: usize) -> AnyType {
        let cells = self.cells.borrow();
        if index == 0 {
            return AnyType::Null;
        }

        let previous_value = cells[index - 1].borrow().get_value().clone();
        let current_value = cells[index].borrow().get_value().clone();

        (current_value - previous_value).into()
    }

    pub fn get_values_as_vec_with_datetime<T>(&self) -> Vec<(DateTime<Utc>, T)>
    where Option<T>: From<AnyType> {
        let mut values: Vec<(DateTime<Utc>, T)> = vec![];
        for cell in self.cells.borrow().iter() {
            let datetime = cell.borrow().get_row().borrow().get_datetime();
            let value_option: Option<T> = cell.borrow().get_value().clone().into();
            if let Some(value) = value_option {
                values.push((datetime, value));
            }
        }

        values
    }

    pub fn get_values_as_vec_with_unix_datetime<T>(&self) -> Vec<(i64, T)>
    where Option<T>: From<AnyType> {
        let mut values: Vec<(i64, T)> = vec![];
        for cell in self.cells.borrow().iter() {
            let datetime = cell.borrow().get_row().borrow().get_datetime().timestamp();
            let value_option: Option<T> = cell.borrow().get_value().clone().into();
            if let Some(value) = value_option {
                values.push((datetime, value));
            }
        }

        values
    }

    pub fn get_values_as_vec_with_unix_datetime_diff<T>(&self) -> Vec<(i64, T)>
    where Option<T>: From<AnyType>,
    T: Into<f64> {
        let mut values: Vec<(i64, T)> = vec![];
        for (index, cell) in self.cells.borrow().iter().enumerate() {
            let mut datetime = cell.borrow().get_row().borrow().get_datetime().timestamp();
            if index == 0 {
                datetime = 0i64;
            } else  {
                let previous_datetime = values[index - 1].0;
                datetime = datetime - previous_datetime;
            }
            let value_option: Option<T> = cell.borrow().get_value().clone().into();
            if let Some(value) = value_option {
                values.push((datetime, value));
            }
        }

        values
    }

    pub fn get_rolling_means_as_vec_with_unix_datetime_diff<T>(&self) -> Vec<(f64, f64)>
    where Option<T>: From<AnyType>,
    T: Into<f64> {
        let mut values: Vec<(f64, f64)> = vec![];
        let initial_datetime: f64 = self.cells.borrow()[0].borrow().get_row().borrow().get_datetime().timestamp() as f64;
        for (index, cell) in self.cells.borrow().iter().enumerate() {
            let mut datetime: f64 = cell.borrow().get_row().borrow().get_datetime().timestamp() as f64;
            if index == 0 {
                datetime = 0f64;
            } else  {
                datetime = datetime - initial_datetime;
            }
            let rolling_mean_option: Option<AnyType> = cell.borrow().get_rolling_mean().clone();
            if let Some(rolling_mean) = rolling_mean_option {
                let value_option: Option<T> = rolling_mean.into();
                if let Some(value) = value_option {
                    values.push((datetime, value.into()));
                }
            }            
        }

        values
    }

    pub fn rate_of_change_rolling_means_over_x_elements<T>(&self, x: usize) -> (Vec<(f64, f64)>, Vec<(f64, f64)>)
    where Option<T>: From<AnyType>,
    T: Into<f64> {
        let rolling_means: Vec<(f64, f64)> = self.get_rolling_means_as_vec_with_unix_datetime_diff::<T>();
        let mut rate_of_change: Vec<(f64, f64)> = vec![];
        for (index, (datetime, _)) in rolling_means.iter().enumerate()  {
            rate_of_change.push((datetime.clone(), self.least_squares(x, index, &rolling_means)));
        }
        (rolling_means, rate_of_change)
    }

    fn least_squares(&self, slice_size: usize, index: usize, rolling_means: &Vec<(f64, f64)>) -> f64 {
        if index < (slice_size - 1) {
            0f64
        } else {
            let sums = rolling_means[(index - (slice_size - 1))..=index].iter().fold((0f64, 0f64), |mut acc, (datetime, rolling_mean)| {
                acc.0 += datetime;
                acc.1 += rolling_mean;
                acc
            });
            
            let (mean_datetime, mean_rolling_mean) = (sums.0 / slice_size as f64, sums.1 / slice_size as f64);
            let (numerator, denominator) = rolling_means[(index - (slice_size - 1))..=index].iter().fold((0f64, 0f64), |mut acc, (datetime, rolling_mean)| {
                let diff_rolling_mean = rolling_mean - mean_rolling_mean;
                let diff_datetime = datetime - mean_datetime;
                acc.0 += diff_rolling_mean * diff_datetime;
                acc.1 += diff_rolling_mean.powf(2f64);
                acc
            });
            numerator / denominator
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cell::{
        types::datatypes::AnyType,
        Cell,
        RcCell,
    };
    use crate::row::{
        Row,
        RcRow,
    };
    use core::time;
    use std::rc::{ Rc };
    use std::thread;
    #[test]
    fn add_cell() {
        let value: AnyType = 67u16.into();
        let row: RcRow = Row::new(3);
        let cell: RcCell = Cell::new(value, &row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(false, None), Returns::new(false, None));
        column.add_cell(&cell);

        assert!(column.get_grouped_values(value) == Some(&RefCell::new(vec![Rc::clone(&cell)])));
        assert!(column.get_cells()[0] == cell);
    }

    #[test]
    fn add_cell_rolling_mean() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(true, Some(2)), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);

        assert!(column.get_cells()[0].borrow().get_rolling_mean() == None);
        assert!(column.get_cells()[1].borrow().get_rolling_mean() == Some(68u16.into()));
    }

    #[test]
    fn update_rolling_mean() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(false, None), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);

        assert!(column.get_cells()[0].borrow().get_rolling_mean() == None);
        assert!(column.get_cells()[1].borrow().get_rolling_mean() == None);

        column.update_rolling_mean(RollingMean::new(true, Some(2)));

        assert!(column.get_cells()[0].borrow().get_rolling_mean() == None);
        assert!(column.get_cells()[1].borrow().get_rolling_mean() == Some(68u16.into()));
    }

    #[test]
    fn drop_cell() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");
        let third_row: RcRow = Row::new(2);
        let third_cell: RcCell = Cell::new(71u16.into(), &third_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(true, Some(2)), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);
        column.add_cell(&third_cell);

        assert!(column.get_cells()[0].borrow().get_rolling_mean() == None);
        assert!(column.get_cells()[1].borrow().get_rolling_mean() == Some(68u16.into()));
        assert!(column.get_cells()[2].borrow().get_rolling_mean() == Some(70u16.into()));
        // a bit dodgy here, but need to update the index of the third row before dropping the second row
        third_row.borrow_mut().update_index(1);
        column.drop_cell(second_cell);

        assert!(column.get_cells()[0].borrow().get_rolling_mean() == None);
        assert!(column.get_cells()[1].borrow().get_rolling_mean() == Some(69u16.into()));
    }

    #[test]
    fn mean() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");
        let third_row: RcRow = Row::new(2);
        let third_cell: RcCell = Cell::new(71u16.into(), &third_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(true, Some(2)), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);
        column.add_cell(&third_cell);

        assert!(column.mean() == Some(69u16.into()));
    }

    #[test]
    fn get_all_difference_to_last() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");
        let third_row: RcRow = Row::new(2);
        let third_cell: RcCell = Cell::new(71u16.into(), &third_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(true, Some(2)), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);
        column.add_cell(&third_cell);

        assert!(column.get_all_difference_to_last() == vec![AnyType::Null, 2isize.into(), 2isize.into()]);
    }

    #[test]
    fn get_values_as_vec_with_datetime() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");
        let third_row: RcRow = Row::new(2);
        let third_cell: RcCell = Cell::new(71u16.into(), &third_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(false, None), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);
        column.add_cell(&third_cell);

        let values_as_vec = column.get_values_as_vec_with_datetime::<u16>();

        assert_eq!(values_as_vec.len(), 3);
        assert_eq!(values_as_vec[0].1, 67u16);
        assert_eq!(values_as_vec[1].1, 69u16);
        assert_eq!(values_as_vec[2].1, 71u16);
        
    }

    #[test]
    fn get_values_as_vec_with_unix_datetime() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");
        let third_row: RcRow = Row::new(2);
        let third_cell: RcCell = Cell::new(71u16.into(), &third_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(false, None), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);
        column.add_cell(&third_cell);

        let values_as_vec: Vec<(i64, u16)> = column.get_values_as_vec_with_unix_datetime::<u16>();

        assert_eq!(values_as_vec.len(), 3);
        assert_eq!(values_as_vec[0].1, 67u16);
        assert_eq!(values_as_vec[1].1, 69u16);
        assert_eq!(values_as_vec[2].1, 71u16);
        
    }

    #[test]
    fn get_rolling_means_as_vec_with_unix_datetime_diff() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");
        let third_row: RcRow = Row::new(2);
        let third_cell: RcCell = Cell::new(71u16.into(), &third_row, "timmeh");
        let fourth_row: RcRow = Row::new(3);
        let fourth_cell: RcCell = Cell::new(77u16.into(), &fourth_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(true, Some(2)), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);
        column.add_cell(&third_cell);
        column.add_cell(&fourth_cell);

        let values_as_vec: Vec<(f64, f64)> = column.get_rolling_means_as_vec_with_unix_datetime_diff::<u16>();

        assert_eq!(values_as_vec.len(), 3);
        assert_eq!(values_as_vec[0].1, 68f64);
        assert_eq!(values_as_vec[1].1, 70f64);
        assert_eq!(values_as_vec[2].1, 74f64);        
    }

    #[test]
    fn rate_of_change_rolling_means_over_x_elements() {
        let row: RcRow = Row::new(0);
        let cell: RcCell = Cell::new(67u16.into(), &row, "timmeh");
        // sleep to ensure that there is a duration over which an average can be found
        thread::sleep(time::Duration::from_millis(2000));
        let second_row: RcRow = Row::new(1);
        let second_cell: RcCell = Cell::new(69u16.into(), &second_row, "timmeh");
        thread::sleep(time::Duration::from_millis(1500));
        let third_row: RcRow = Row::new(2);
        let third_cell: RcCell = Cell::new(71u16.into(), &third_row, "timmeh");
        thread::sleep(time::Duration::from_millis(2400));
        let fourth_row: RcRow = Row::new(3);
        let fourth_cell: RcCell = Cell::new(77u16.into(), &fourth_row, "timmeh");
        thread::sleep(time::Duration::from_millis(6000));
        let fifth_row: RcRow = Row::new(4);
        let fifth_cell: RcCell = Cell::new(77u16.into(), &fifth_row, "timmeh");

        let mut column = Column::new("timmeh", RollingMean::new(true, Some(2)), Returns::new(false, None));
        column.add_cell(&cell);
        column.add_cell(&second_cell);
        column.add_cell(&third_cell);
        column.add_cell(&fourth_cell);
        column.add_cell(&fifth_cell);

        let (rol, roc): (Vec<(f64, f64)>, Vec<(f64, f64)>) = column.rate_of_change_rolling_means_over_x_elements::<u16>(2);

        assert_eq!(roc.len(), 4);
        assert_eq!(roc[0].1, 0f64);
        // assert_ne!(roc[1].1, 0f64);
        assert_ne!(roc[3].0, 0f64);      
    }

}