use super::datatypes::{
    DType,
    DataType,
    AnyType
};

#[derive(Debug, Clone)]
pub struct Float32 {}

impl DType for Float32 {
    fn dtype() -> DataType {
        DataType::Float32
    }
}

impl From<f32> for AnyType {
    fn from(value: f32) -> Self {
        AnyType::Float32(value)
    }
}