use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct Int8 {}

impl DType for Int8 {
    fn dtype() -> DataType {
        DataType::Int8
    }
}

impl From<i8> for DataType {
    fn from(value: i8) -> Self {
        DataType::Int8(value)
    }
}