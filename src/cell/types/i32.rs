use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct Int32 {}

impl DType for Int32 {
    fn dtype() -> DataType {
        DataType::Int32
    }
}

impl From<i32> for AnyType {
    fn from(value: i32) -> Self {
        AnyType::Int32(value)
    }
}

impl From<AnyType> for Option<i32> {
    fn from(any_type: AnyType) -> Option<i32> {
        match any_type {
            AnyType::Int32(val) => Some(val),
            _ => None
        }
    }
}