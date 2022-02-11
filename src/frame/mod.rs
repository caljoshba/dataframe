use crate::row::{
    Row,
    RcRow
};
use crate::column::{
    Column,
    RollingMean,
    Returns,
};
use crate::cell::{
    types::datatypes::AnyType,
    Cell,
};
use std::cell::{
    RefCell,
    Ref,
    RefMut,
};
use chrono::{
    DateTime,
    Utc
};

#[derive(Debug)]
pub struct DataFrame {
    rows: RefCell<Vec<RcRow>>,
    columns: Vec<Column>
}

impl DataFrame {
    pub fn new(column_names: Vec<&'static str>) -> Self {
        let mut columns = vec![];
        for column_name in column_names.iter() {
            let column = Column::new(*column_name, RollingMean::new(false, None), Returns::new(false, None));
            columns.push(column);
        }

        Self {
            rows: RefCell::new(vec![]),
            columns
        }
    }

    pub fn get_rows(&self) -> Ref<Vec<RcRow>> {
        self.rows.borrow()
    }

    pub fn get_columns(&self) -> &Vec<Column> {
        &self.columns
    }

    pub fn add_row(&mut self, cell_values: Vec<AnyType>) -> usize {
        let total_rows = self.rows.borrow().len();
        let row = Row::new(total_rows);
        let row_index = row.borrow().index;
        for (index, cell_value) in cell_values.iter().enumerate() {
            let column: &mut Column = &mut self.columns[index];
            let cell = Cell::new(*cell_value, &row, &column.name.clone());
            row.borrow_mut().add_cell(&cell);
            column.add_cell(&cell);
        }
        // this does not update the returns for columns
        self.add_returns_for_cells(row_index, &row);
        self.rows.borrow_mut().push(row);
        row_index
    }

    pub fn add_column_from_values(&mut self, column_name: &'static str, cell_values: Vec<AnyType>, rolling_mean: RollingMean) {
        let mut column = Column::new(column_name, rolling_mean, Returns::new(false, None));
        for (index, cell_value) in cell_values.iter().enumerate() {
            let row = &self.rows.borrow()[index];
            let cell = Cell::new(*cell_value, row, column_name.clone());
            row.borrow_mut().add_cell(&cell);
            column.add_cell(&cell);
        }
        self.columns.push(column);
    }

    pub fn add_column(&mut self, column: Column) {
        self.columns.push(column);
    }

    // create a test that generates a weak pointer
    // run this method and check the weak reference returns None
    // when upgrading weak pointer
    // https://doc.rust-lang.org/std/rc/struct.Weak.html#method.upgrade
    pub fn drop_row(&mut self, row_index: usize) {
        let mut rows = self.rows.borrow_mut();
        let row: Vec<RcRow> = rows.drain(row_index..row_index+1).collect();
        self.update_row_index(&rows);
        for (column_index, cell) in row[0].borrow().get_cells().iter().enumerate() {
            let column = &mut self.columns[column_index];
            column.drop_cell(cell.upgrade().unwrap());
        }
        drop(row);
    }

    fn update_row_index(&self, rows: &RefMut<Vec<RcRow>>) {
        for (index, row) in rows.iter().enumerate() {
            row.borrow_mut().update_index(index);
        }
    }

    pub fn drop_column(&mut self, column_index: usize) {
        let column: Vec<Column> = self.columns.drain(column_index..column_index+1).collect();
        for cell in column[0].get_cells().iter() {
            let row = cell.borrow().clone_row();
            row.borrow_mut().drop_cell(cell);
        }
        drop(column);
    }

    pub fn drop_column_by_name(&mut self, column_name: &str) {
        let index = self.columns.iter().position(|c| c.name == column_name).unwrap();
        self.drop_column(index);
    }

    pub fn update_column_rolling_mean(&mut self, column_name: &'static str, rolling_mean: RollingMean) {
        let column = self.get_mut_column_by_name(column_name);
        column.update_rolling_mean(rolling_mean);
    }

    pub fn create_returns_for_column(&mut self, column_name: &'static str, new_column_name: &'static str, rolling_mean: RollingMean) {
        let column = self.get_mut_column_by_name(column_name);
        let values: Option<Vec<AnyType>> = column.update_returns(Returns::new(true, Some(new_column_name)));
        self.add_column_from_values(new_column_name, values.unwrap(), rolling_mean);
    }

    fn get_mut_column_by_name(&mut self, column_name: &'static str) -> &mut Column {
        self.columns.iter_mut().find(|c| c.name == column_name).unwrap()
    }

    fn get_column_by_name(&self, column_name: &'static str) -> &Column {
        self.columns.iter().find(|c| c.name == column_name).unwrap()
    }

    pub fn add_returns_for_cells(&mut self, row_index: usize, row: &RcRow) {
        let mut iterator =  self.columns.iter_mut();
        for column in iterator.find(|c| c.returns.should_calculate) {
            let value = column.get_difference_to_last(row_index);
            let returns_column = iterator.find(|c| Some(c.name) == column.returns.column_name).unwrap();
            let cell = Cell::new(value, row, returns_column.name.clone());
            returns_column.add_cell(&cell);
            row.borrow_mut().add_cell(&cell);
        }
    }

    pub fn get_column_values_with_datetime<T>(&self, column_name: &'static str) -> Vec<(DateTime<Utc>, T)>
    where Option<T>: From<AnyType> {
        let column = self.get_column_by_name(column_name);
        column.get_values_as_vec_with_datetime()
    }

    pub fn get_column_values_with_unix_datetime<T>(&self, column_name: &'static str) -> Vec<(i64, T)>
    where Option<T>: From<AnyType> {
        let column = self.get_column_by_name(column_name);
        column.get_values_as_vec_with_unix_datetime()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::{ Rc };
    #[test]
    fn drop_row() {
        let columns = vec![
            "rando",
            "second"
        ];
        let mut dataframe = DataFrame::new(columns);
        let cell_values: Vec<AnyType> = vec![
            6.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values);
        let cell_values2: Vec<AnyType> = vec![
            7.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values2);
        let cell_ref = Rc::downgrade(&dataframe.get_columns()[0].get_cells()[0]);
        dataframe.drop_row(0);

        assert!(cell_ref.upgrade().is_none());
        assert_eq!(dataframe.get_rows().len(), 1);
        assert_eq!(dataframe.get_rows()[0].borrow().index, 0);
        assert_eq!(dataframe.get_columns()[0].get_cells().len(), 1);
    }

    #[test]
    fn drop_column() {
        let columns = vec![
            "rando",
            "second"
        ];
        let mut dataframe = DataFrame::new(columns);
        let cell_values: Vec<AnyType> = vec![
            6.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values);
        let cell_values2: Vec<AnyType> = vec![
            7.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values2);
        let cell_ref = Rc::downgrade(&dataframe.get_columns()[1].get_cells()[0]);
        let cell_ref2 = Rc::downgrade(&dataframe.get_columns()[1].get_cells()[1]);
        dataframe.drop_column(1);

        assert!(cell_ref.upgrade().is_none());
        assert!(cell_ref2.upgrade().is_none());
        assert_eq!(dataframe.get_rows().len(), 2);
        assert_eq!(dataframe.get_columns().len(), 1);
    }

    #[test]
    fn create_returns_column() {
        let columns = vec![
            "rando",
            "second"
        ];
        let mut dataframe = DataFrame::new(columns);
        dataframe.create_returns_for_column("rando", "rando_returns", RollingMean::new(true, Some(2)));
        let cell_values: Vec<AnyType> = vec![
            6u8.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values);

        assert_eq!(dataframe.get_columns().len(), 3);
        assert_eq!(dataframe.get_columns()[0].returns.should_calculate, true);
        assert_eq!(dataframe.get_columns()[0].returns.column_name, Some("rando_returns"));
        assert_eq!(dataframe.get_rows()[0].borrow().get_cells().len(), 3);
        assert_eq!(dataframe.get_rows()[0].borrow().get_cells()[2].upgrade().unwrap().borrow().get_value(), &AnyType::Null);
    }

    #[test]
    fn add_returns_column() {
        let columns = vec![
            "rando",
            "second"
        ];
        let mut dataframe = DataFrame::new(columns);
        dataframe.create_returns_for_column("rando", "rando_returns", RollingMean::new(true, Some(2)));
        let cell_values: Vec<AnyType> = vec![
            6.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values);
        let cell_values2: Vec<AnyType> = vec![
            7.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values2);
        let cell_values3: Vec<AnyType> = vec![
            8.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values3);
        let cell_values4: Vec<AnyType> = vec![
            11.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values4);
        let cell_values5: Vec<AnyType> = vec![
            1.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values5);
        assert!(dataframe.get_columns().len() == 3);
        assert!(dataframe.get_rows().len() == 5);
        assert!(dataframe.get_rows()[0].borrow().get_cells().len() == 3);
        assert_eq!(dataframe.get_rows()[2].borrow().get_cells()[2].upgrade().unwrap().borrow().get_value(), &1isize.into());
        assert_eq!(dataframe.get_rows()[3].borrow().get_cells()[2].upgrade().unwrap().borrow().get_value(), &3isize.into());
        assert_eq!(dataframe.get_rows()[4].borrow().get_cells()[2].upgrade().unwrap().borrow().get_value(), &(-10isize).into());
    }

    #[test]
    fn rolling_mean() {
        let columns = vec![
            "rando",
            "second"
        ];
        let mut dataframe = DataFrame::new(columns);
        dataframe.update_column_rolling_mean("rando", RollingMean::new(true, Some(2)));
        let cell_values: Vec<AnyType> = vec![
            6usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values);
        let cell_values2: Vec<AnyType> = vec![
            7usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values2);
        let cell_values3: Vec<AnyType> = vec![
            7usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values3);
        let cell_values4: Vec<AnyType> = vec![
            9usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values4);
        let cell_values5: Vec<AnyType> = vec![
            1usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values5);
        assert!(dataframe.get_columns().len() == 2);
        assert!(dataframe.get_rows().len() == 5);
        assert!(dataframe.get_rows()[0].borrow().get_cells().len() == 2);
        assert_eq!(dataframe.get_rows()[2].borrow().get_cells()[0].upgrade().unwrap().borrow().get_rolling_mean(), Some(7usize.into()));
        assert_eq!(dataframe.get_rows()[3].borrow().get_cells()[0].upgrade().unwrap().borrow().get_rolling_mean(), Some(8usize.into()));
        assert_eq!(dataframe.get_rows()[4].borrow().get_cells()[0].upgrade().unwrap().borrow().get_rolling_mean(), Some(5usize.into()));
    }

    #[test]
    fn get_column_values_with_datetime() {
        let columns = vec![
            "rando",
            "second"
        ];
        let mut dataframe = DataFrame::new(columns);
        let cell_values: Vec<AnyType> = vec![
            6usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values);
        let cell_values2: Vec<AnyType> = vec![
            7usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values2);
        let cell_values3: Vec<AnyType> = vec![
            7usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values3);
        let cell_values4: Vec<AnyType> = vec![
            9usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values4);
        let cell_values5: Vec<AnyType> = vec![
            1usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values5);
        
        let column_values: Vec<(DateTime<Utc>, usize)> = dataframe.get_column_values_with_datetime::<usize>("rando");

        assert_eq!(column_values[0].1, 6usize);
    }

    fn get_column_values_with_unix_datetime() {
        let columns = vec![
            "rando",
            "second"
        ];
        let mut dataframe = DataFrame::new(columns);
        let cell_values: Vec<AnyType> = vec![
            6usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values);
        let cell_values2: Vec<AnyType> = vec![
            7usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values2);
        let cell_values3: Vec<AnyType> = vec![
            7usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values3);
        let cell_values4: Vec<AnyType> = vec![
            9usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values4);
        let cell_values5: Vec<AnyType> = vec![
            1usize.into(),
            "whoop".into()
        ];
        dataframe.add_row(cell_values5);
        
        let column_values: Vec<(i64, usize)> = dataframe.get_column_values_with_unix_datetime::<usize>("rando");

        assert_eq!(column_values[0].1, 6usize);
    }
}