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

impl From<AnyType> for Option<u64> {
    fn from(any_type: AnyType) -> Option<u64> {
        match any_type {
            AnyType::UInt64(val) => Some(val),
            _ => None
        }
    }
}