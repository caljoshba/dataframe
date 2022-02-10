use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct UInt16 {}

impl DType for UInt16 {
    fn dtype() -> DataType {
        DataType::UInt16
    }
}

impl From<u16> for AnyType {
    fn from(value: u16) -> Self {
        AnyType::UInt16(value)
    }
}

impl From<AnyType> for Option<u16> {
    fn from(any_type: AnyType) -> Option<u16> {
        match any_type {
            AnyType::UInt16(val) => Some(val),
            _ => None
        }
    }
}