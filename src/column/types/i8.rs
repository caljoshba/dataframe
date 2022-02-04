use crate::cell::{
    types::datatypes::AnyType,
};
use crate::column::{
    Column
};

pub fn mean(column: &Column) -> Option<AnyType> {
    let mut total: i8 = 0;
    let mut number: i8 = 0;
    for cell in column.get_cells().iter() {
        if let AnyType::Int8(cell_value) = cell.borrow().get_value().clone() {
            total += cell_value;
            number += 1;
        }
    }

    Some((total / number).into())
}