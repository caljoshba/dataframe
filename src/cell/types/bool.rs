use super::datatypes::{
    DType,
    DataType,
    AnyType
};


#[derive(Debug, Clone)]
pub struct Boolean {}

impl DType for Boolean {
    fn dtype() -> DataType {
        DataType::Boolean
    }
}

impl From<bool> for AnyType {
    fn from(value: bool) -> Self {
        AnyType::Boolean(value)
    }
}

impl From<AnyType> for Option<bool> {
    fn from(any_type: AnyType) -> Option<bool> {
        match any_type {
            AnyType::Boolean(val) => Some(val),
            _ => None
        }
    }
}