use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct Int16 {}

impl DType for Int16 {
    fn dtype() -> DataType {
        DataType::Int16
    }
}

impl From<i16> for DataType {
    fn from(value: i16) -> Self {
        DataType::Int16(value)
    }
}