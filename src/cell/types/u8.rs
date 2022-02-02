use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct UInt8 {}

impl DType for UInt8 {
    fn dtype() -> DataType {
        DataType::UInt8
    }
}

impl From<u8> for DataType {
    fn from(value: u8) -> Self {
        DataType::UInt8(value)
    }
}