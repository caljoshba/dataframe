use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct Int64 {}

impl DType for Int64 {
    fn dtype() -> DataType {
        DataType::Int64
    }
}

impl From<i64> for AnyType {
    fn from(value: i64) -> Self {
        AnyType::Int64(value)
    }
}

impl From<AnyType> for Option<i64> {
    fn from(any_type: AnyType) -> Option<i64> {
        match any_type {
            AnyType::Int64(val) => Some(val),
            _ => None
        }
    }
}