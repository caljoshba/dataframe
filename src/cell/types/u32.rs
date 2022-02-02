use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct UInt32 {}

impl DType for UInt32 {
    fn dtype() -> DataType {
        DataType::UInt32
    }
}

impl From<u32> for DataType {
    fn from(value: u32) -> Self {
        DataType::UInt32(value)
    }
}