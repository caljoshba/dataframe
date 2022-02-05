// https://github.com/pola-rs/polars/blob/master/polars/polars-core/src/datatypes.rs

use std::hash::{Hash, Hasher};
use std::fmt::{
    Display,
    Formatter,
    Result
};
use std::ops::{
    Add,
    Div
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
    USize(usize),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    ISize(isize),
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
            ISize(v) => state.write_isize(*v),
            UInt8(v) => state.write_u8(*v),
            UInt16(v) => state.write_u16(*v),
            UInt32(v) => state.write_u32(*v),
            UInt64(v) => state.write_u64(*v),
            USize(v) => state.write_usize(*v),
            Utf8(s) => state.write(s.as_bytes()),
            Boolean(v) => state.write_u8(*v as u8),
            Float32(v) => state.write_i32(v.floor() as i32),
            Float64(v) => state.write_i64(v.floor() as i64),
        }
    }
}

impl Display for AnyType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            AnyType::Null => write!(f, "null"),
            AnyType::Boolean(val) => write!(f, "{}", val),
            AnyType::Utf8(val) => write!(f, "{}", val),
            AnyType::UInt8(val) => write!(f, "{}", val),
            AnyType::UInt16(val) => write!(f, "{}", val),
            AnyType::UInt32(val) => write!(f, "{}", val),
            AnyType::UInt64(val) => write!(f, "{}", val),
            AnyType::USize(val) => write!(f, "{}", val),
            AnyType::Int8(val) => write!(f, "{}", val),
            AnyType::Int16(val) => write!(f, "{}", val),
            AnyType::Int32(val) => write!(f, "{}", val),
            AnyType::Int64(val) => write!(f, "{}", val),
            AnyType::ISize(val) => write!(f, "{}", val),
            AnyType::Float32(val) => write!(f, "{}", val),
            AnyType::Float64(val) => write!(f, "{}", val)
        }
    }
}

impl Add for AnyType {
    type Output = AnyType;
    fn add(self, rhs: Self) -> Self::Output {
        use AnyType::*;
        match (self, rhs) {
            (UInt8(lv), UInt8(rv)) => UInt8(lv + rv),
            (UInt16(lv), UInt16(rv)) => UInt16(lv + rv),
            (UInt32(lv), UInt32(rv)) => UInt32(lv + rv),
            (UInt64(lv), UInt64(rv)) => UInt64(lv + rv),
            (USize(lv), USize(rv)) => USize(lv + rv),
            (Int8(lv), Int8(rv)) => Int8(lv + rv),
            (Int16(lv), Int16(rv)) => Int16(lv + rv),
            (Int32(lv), Int32(rv)) => Int32(lv + rv),
            (Int64(lv), Int64(rv)) => Int64(lv + rv),
            (ISize(lv), ISize(rv)) => ISize(lv + rv),
            (Float32(lv), Float32(rv)) => Float32(lv + rv),
            (Float64(lv), Float64(rv)) => Float64(lv + rv),
            (Null, UInt8(rv)) => UInt8(rv),
            (Null, UInt16(rv)) => UInt16(rv),
            (Null, UInt32(rv)) => UInt32(rv),
            (Null, UInt64(rv)) => UInt64(rv),
            (Null, USize(rv)) => USize(rv),
            (Null, Int8(rv)) => Int8(rv),
            (Null, Int16(rv)) => Int16(rv),
            (Null, Int32(rv)) => Int32(rv),
            (Null, Int64(rv)) => Int64(rv),
            (Null, ISize(rv)) => ISize(rv),
            (Null, Float32(rv)) => Float32(rv),
            (Null, Float64(rv)) => Float64(rv),
            (_, _) => Null 
        }     
    }
}

impl Div for AnyType {
    type Output = AnyType;
    fn div(self, rhs: Self) -> Self::Output {
        use AnyType::*;
        match (self, rhs) {
            (UInt8(lv), UInt8(rv)) => UInt8(lv / rv),
            (UInt16(lv), UInt16(rv)) => UInt16(lv / rv),
            (UInt32(lv), UInt32(rv)) => UInt32(lv / rv),
            (UInt64(lv), UInt64(rv)) => UInt64(lv / rv),
            (USize(lv), USize(rv)) => USize(lv / rv),
            (Int8(lv), Int8(rv)) => Int8(lv / rv),
            (Int16(lv), Int16(rv)) => Int16(lv / rv),
            (Int32(lv), Int32(rv)) => Int32(lv / rv),
            (Int64(lv), Int64(rv)) => Int64(lv / rv),
            (ISize(lv), ISize(rv)) => ISize(lv / rv),
            (Float32(lv), Float32(rv)) => Float32(lv / rv),
            (Float64(lv), Float64(rv)) => Float64(lv / rv),
            (UInt8(lv), USize(rv)) => UInt8(lv / rv as u8),
            (UInt16(lv), USize(rv)) => UInt16(lv / rv as u16),
            (UInt32(lv), USize(rv)) => UInt32(lv / rv as u32),
            (UInt64(lv), USize(rv)) => UInt64(lv / rv as u64),
            (Int8(lv), USize(rv)) => Int8(lv / rv as i8),
            (Int16(lv), USize(rv)) => Int16(lv / rv as i16),
            (Int32(lv), USize(rv)) => Int32(lv / rv as i32),
            (Int64(lv), USize(rv)) => Int64(lv / rv as i64),
            (Float32(lv), USize(rv)) => Float32(lv / rv as f32),
            (Float64(lv), USize(rv)) => Float64(lv / rv as f64),
            (Int8(lv), ISize(rv)) => Int8(lv / rv as i8),
            (Int16(lv), ISize(rv)) => Int16(lv / rv as i16),
            (Int32(lv), ISize(rv)) => Int32(lv / rv as i32),
            (Int64(lv), ISize(rv)) => Int64(lv / rv as i64),
            (Float32(lv), ISize(rv)) => Float32(lv / rv as f32),
            (Float64(lv), ISize(rv)) => Float64(lv / rv as f64),
            (_, _) => Null 
        }     
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
    USize,
    Int8,
    Int16,
    Int32,
    Int64,
    ISize,
    Float32,
    Float64
}

impl PartialEq for AnyType {
    fn eq(&self, other: &Self) -> bool {
        use AnyType::*;
        match (self, other) {
            (Null, Null) => true,
            (Boolean(val), Boolean(rhs)) => val == rhs,
            (Utf8(val), Utf8(rhs)) => val == rhs,
            (UInt8(val), UInt8(rhs)) => val == rhs,
            (UInt16(val), UInt16(rhs)) => val == rhs,
            (UInt32(val), UInt32(rhs)) => val == rhs,
            (UInt64(val), UInt64(rhs)) => val == rhs,
            (Int8(val), Int8(rhs)) => val == rhs,
            (Int16(val), Int16(rhs)) => val == rhs,
            (Int32(val), Int32(rhs)) => val == rhs,
            (Int64(val), Int64(rhs)) => val == rhs,
            (Float32(val), Float32(rhs)) => val == rhs,
            (Float64(val), Float64(rhs)) => val == rhs,
            (_, _) => false
        }
    }
}

impl Eq for AnyType {}

// #[derive(Debug, Clone)]
// pub struct Date {}
// #[derive(Debug, Clone)]
// pub struct Datetime {}
// #[derive(Debug, Clone)]
// pub struct Duration {}
// #[derive(Debug, Clone)]
// pub struct Time {}

impl Display for DataType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let s = match self {
            DataType::Null => "null",
            DataType::Boolean => "bool",
            DataType::UInt8 => "u8",
            DataType::UInt16 => "u16",
            DataType::UInt32 => "u32",
            DataType::UInt64 => "u64",
            DataType::USize => "usize",
            DataType::Int8 => "i8",
            DataType::Int16 => "i16",
            DataType::Int32 => "i32",
            DataType::Int64 => "i64",
            DataType::ISize => "isize",
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