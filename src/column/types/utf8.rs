use crate::column::{
    Column
};
use crate::cell::{
    types::datatypes::AnyType,
};

pub fn mean(_: &Column) -> Option<AnyType> {
    None
}