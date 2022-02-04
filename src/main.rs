#![allow(dead_code)]
mod cell;
mod column;
mod row;
mod frame;

use cell::{
    types::datatypes::AnyType,
};
use frame::{
    DataFrame
};

fn main() {
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
    for column in dataframe.get_columns().iter() {
        for cell in column.get_cells().iter() {
            println!("{}", cell.borrow().get_value());
        }
    }
}
