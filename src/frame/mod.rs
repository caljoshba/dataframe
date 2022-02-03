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
use std::cell::RefCell;
use std::rc::{ Rc };

pub struct DataFrame {
    rows: RefCell<Vec<RcRow>>,
    pub columns: Vec<Column>
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

    pub fn add_row(&mut self, cell_values: Vec<AnyType>) {
        let row = Row::new();
        for (index, cell_value) in cell_values.iter().enumerate() {
            let cell = Cell::new(*cell_value, &row);
            row.borrow_mut().add_cell(&cell);
            self.columns[index].add_cell(Rc::clone(&cell));
        }
        self.rows.borrow_mut().push(row);
    }
}