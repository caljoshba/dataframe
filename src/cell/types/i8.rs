use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct Int8 {}

impl DType for Int8 {
    fn dtype() -> DataType {
        DataType::Int8
    }
}

impl From<i8> for AnyType {
    fn from(value: i8) -> Self {
        AnyType::Int8(value)
    }
}