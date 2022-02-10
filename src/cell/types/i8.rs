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

impl From<AnyType> for Option<i8> {
    fn from(any_type: AnyType) -> Option<i8> {
        match any_type {
            AnyType::Int8(val) => Some(val),
            _ => None
        }
    }
}