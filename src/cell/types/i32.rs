use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct Int32 {}

impl DType for Int32 {
    fn dtype() -> DataType {
        DataType::Int32
    }
}

impl From<i32> for DataType {
    fn from(value: i32) -> Self {
        DataType::Int32(value)
    }
}