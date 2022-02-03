use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct UInt64 {}

impl DType for UInt64 {
    fn dtype() -> DataType {
        DataType::UInt64
    }
}

impl From<u64> for AnyType {
    fn from(value: u64) -> Self {
        AnyType::UInt64(value)
    }
}