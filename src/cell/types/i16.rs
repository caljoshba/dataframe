use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct Int16 {}

impl DType for Int16 {
    fn dtype() -> DataType {
        DataType::Int16
    }
}

impl From<i16> for AnyType {
    fn from(value: i16) -> Self {
        AnyType::Int16(value)
    }
}