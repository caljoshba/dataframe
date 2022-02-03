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