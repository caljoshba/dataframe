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