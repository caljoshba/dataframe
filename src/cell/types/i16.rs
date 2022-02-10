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

impl From<AnyType> for Option<i16> {
    fn from(any_type: AnyType) -> Option<i16> {
        match any_type {
            AnyType::Int16(val) => Some(val),
            _ => None
        }
    }
}