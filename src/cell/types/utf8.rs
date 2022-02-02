use super::datatypes::{
    DType,
    DataType
};

#[derive(Debug, Clone)]
pub struct Utf8 {}

impl DType for Utf8 {
    fn dtype() -> DataType {
        DataType::Utf8
    }
}

impl From<String> for DataType {
    fn from(value: String) -> Self {
        DataType::Utf8(value)
    }
}