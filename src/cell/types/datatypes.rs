// https://github.com/pola-rs/polars/blob/master/polars/polars-core/src/datatypes.rs

use std::hash::{Hash, Hasher};
use std::fmt::{
    Display,
    Formatter,
    Result
};

#[derive(Debug, Copy, Clone)]
pub enum AnyType {
    Null,
    Boolean(bool),
    Utf8(&'static str),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    // Date,
    // Datetime,
    // Duration,
    // Time,
}

impl<T> From<Option<T>> for AnyType
where
    T: Into<AnyType>,
{
    fn from(a: Option<T>) -> Self {
        match a {
            None => AnyType::Null,
            Some(v) => v.into(),
        }
    }
}

impl Hash for AnyType {
    fn hash<H: Hasher>(&self, state: &mut H) {
        use AnyType::*;
        match self {
            Null => state.write_u64(u64::MAX / 2 + 135123),
            Int8(v) => state.write_i8(*v),
            Int16(v) => state.write_i16(*v),
            Int32(v) => state.write_i32(*v),
            Int64(v) => state.write_i64(*v),
            UInt8(v) => state.write_u8(*v),
            UInt16(v) => state.write_u16(*v),
            UInt32(v) => state.write_u32(*v),
            UInt64(v) => state.write_u64(*v),
            Utf8(s) => state.write(s.as_bytes()),
            Boolean(v) => state.write_u8(*v as u8),
            _ => unimplemented!(),
        }
    }
}

impl Display for AnyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let value = match self {
            AnyType::Null => "null".to_string(),
            AnyType::Boolean(val) => val.to_string(),
            AnyType::Utf8(val) => val.to_string(),
            AnyType::UInt8(val) => val.to_string(),
            AnyType::UInt16(val) => val.to_string(),
            AnyType::UInt32(val) => val.to_string(),
            AnyType::UInt64(val) => val.to_string(),
            AnyType::Int8(val) => val.to_string(),
            AnyType::Int16(val) => val.to_string(),
            AnyType::Int32(val) => val.to_string(),
            AnyType::Int64(val) => val.to_string(),
            AnyType::Float32(val) => val.to_string(),
            AnyType::Float64(val) => val.to_string()
        };
        write!(f, "{}", value)
    }
}

// #[derive(Clone, PartialEq, Eq, Debug, Hash)]
pub enum DataType {
    Null,
    Boolean,
    Utf8,
    UInt8,
    UInt16,
    UInt32,
    UInt64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float32,
    Float64
}

// impl Eq for AnyType {}

// #[derive(Debug, Clone)]
// pub struct Date {}
// #[derive(Debug, Clone)]
// pub struct Datetime {}
// #[derive(Debug, Clone)]
// pub struct Duration {}
// #[derive(Debug, Clone)]
// pub struct Time {}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DataType::Null => "null",
            DataType::Boolean => "bool",
            DataType::UInt8 => "u8",
            DataType::UInt16 => "u16",
            DataType::UInt32 => "u32",
            DataType::UInt64 => "u64",
            DataType::Int8 => "i8",
            DataType::Int16 => "i16",
            DataType::Int32 => "i32",
            DataType::Int64 => "i64",
            DataType::Float32 => "f32",
            DataType::Float64 => "f64",
            DataType::Utf8 => "String",
        };
        f.write_str(s)
    }
}

pub trait DType {
    fn dtype() -> DataType where Self: Sized;
}