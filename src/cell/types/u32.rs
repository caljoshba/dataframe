use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct UInt32 {}

impl DType for UInt32 {
    fn dtype() -> DataType {
        DataType::UInt32
    }
}

impl From<u32> for AnyType {
    fn from(value: u32) -> Self {
        AnyType::UInt32(value)
    }
}