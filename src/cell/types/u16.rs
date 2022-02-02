use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct UInt16 {}

impl DType for UInt16 {
    fn dtype() -> DataType {
        DataType::UInt16
    }
}

impl From<u16> for DataType {
    fn from(value: u16) -> Self {
        DataType::UInt16(value)
    }
}