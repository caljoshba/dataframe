use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct Float64 {}

impl DType for Float64 {
    fn dtype() -> DataType {
        DataType::Float64
    }
}

impl From<f64> for DataType {
    fn from(value: f64) -> Self {
        DataType::Float64(value)
    }
}