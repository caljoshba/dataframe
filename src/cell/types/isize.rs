use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct ISize {}

impl DType for ISize {
    fn dtype() -> DataType {
        DataType::ISize
    }
}

impl From<isize> for AnyType {
    fn from(value: isize) -> Self {
        AnyType::ISize(value)
    }
}

impl From<AnyType> for Option<isize> {
    fn from(any_type: AnyType) -> Option<isize> {
        match any_type {
            AnyType::ISize(val) => Some(val),
            _ => None
        }
    }
}