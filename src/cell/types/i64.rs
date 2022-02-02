use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct Int64 {}

impl DType for Int64 {
    fn dtype() -> DataType {
        DataType::Int64
    }
}

impl From<i64> for DataType {
    fn from(value: i64) -> Self {
        DataType::Int64(value)
    }
}