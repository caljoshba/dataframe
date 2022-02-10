use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct UInt8 {}

impl DType for UInt8 {
    fn dtype() -> DataType {
        DataType::UInt8
    }
}

impl From<u8> for AnyType {
    fn from(value: u8) -> Self {
        AnyType::UInt8(value)
    }
}

impl From<AnyType> for Option<u8> {
    fn from(any_type: AnyType) -> Option<u8> {
        match any_type {
            AnyType::UInt8(val) => Some(val),
            _ => None
        }
    }
}
