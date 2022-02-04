use crate::row::{
    Row,
    RcRow
};
use crate::column::{
    Column
};
use crate::cell::{
    types::datatypes::AnyType,
    Cell,
};
use std::cell::{
    RefCell,
    Ref
};

pub struct DataFrame {
    rows: RefCell<Vec<RcRow>>,
    columns: Vec<Column>
}

impl DataFrame {
    pub fn new(column_names: Vec<&'static str>) -> Self {
        let mut columns = vec![];
        for column_name in column_names.iter() {
            let column = Column::new(*column_name);
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

    pub fn add_row(&mut self, cell_values: Vec<AnyType>) {
        let total_rows = self.rows.borrow().len();
        let row = Row::new(total_rows);
        for (index, cell_value) in cell_values.iter().enumerate() {
            let column: &mut Column = &mut self.columns[index];
            let cell = Cell::new(*cell_value, &row, &column.name.clone());
            row.borrow_mut().add_cell(&cell);
            column.add_cell(cell);
        }
        self.rows.borrow_mut().push(row);
    }

    pub fn add_column(&mut self, column_name: &'static str, cell_values: Vec<AnyType>) {
        let mut column = Column::new(column_name);
        for (index, cell_value) in cell_values.iter().enumerate() {
            let row = &self.rows.borrow()[index];
            let cell = Cell::new(*cell_value, row, column_name.clone());
            row.borrow_mut().add_cell(&cell);
            column.add_cell(cell);
        }
        self.columns.push(column);
    }

    // create a test that generates a weak pointer
    // run this method and check the weak reference returns None
    // when upgrading weak pointer
    // https://doc.rust-lang.org/std/rc/struct.Weak.html#method.upgrade
    pub fn drop_row(&mut self, row_index: usize) {
        let mut rows = self.rows.borrow_mut();
        let row: Vec<RcRow> = rows.drain(row_index..row_index+1).collect();
        for (column_index, cell) in row[0].borrow().get_cells().iter().enumerate() {
            let column = &mut self.columns[column_index];
            column.drop_cell(cell.upgrade().unwrap());
        }
        drop(row);
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
}