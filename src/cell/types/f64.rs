use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct Float64 {}

impl DType for Float64 {
    fn dtype() -> DataType {
        DataType::Float64
    }
}

impl From<f64> for AnyType {
    fn from(value: f64) -> Self {
        AnyType::Float64(value)
    }
}

impl From<AnyType> for Option<f64> {
    fn from(any_type: AnyType) -> Option<f64> {
        match any_type {
            AnyType::Float64(val) => Some(val),
            _ => None
        }
    }
}