use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct UInt64 {}

impl DType for UInt64 {
    fn dtype() -> DataType {
        DataType::UInt64
    }
}

impl From<u64> for DataType {
    fn from(value: u64) -> Self {
        DataType::UInt64(value)
    }
}