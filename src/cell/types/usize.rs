use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct USize {}

impl DType for USize {
    fn dtype() -> DataType {
        DataType::USize
    }
}

impl From<usize> for AnyType {
    fn from(value: usize) -> Self {
        AnyType::USize(value)
    }
}

impl From<AnyType> for Option<usize> {
    fn from(any_type: AnyType) -> Option<usize> {
        match any_type {
            AnyType::USize(val) => Some(val),
            _ => None
        }
    }
}