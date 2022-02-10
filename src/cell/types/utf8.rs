use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct Utf8 {}

impl DType for Utf8 {
    fn dtype() -> DataType {
        DataType::Utf8
    }
}

impl From<&'static str> for AnyType {
    fn from(value: &'static str) -> Self {
        AnyType::Utf8(value)
    }
}

impl From<AnyType> for Option<&'static str> {
    fn from(any_type: AnyType) -> Option<&'static str> {
        match any_type {
            AnyType::Utf8(val) => Some(val),
            _ => None
        }
    }
}