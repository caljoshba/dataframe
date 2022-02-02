use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct Float32 {}

impl DType for Float32 {
    fn dtype() -> DataType {
        DataType::Float32
    }
}

impl From<f32> for DataType {
    fn from(value: f32) -> Self {
        DataType::Float32(value)
    }
}