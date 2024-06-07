use std::borrow::Cow;

use super::errors;

// Only implement types supported by Lua for now
#[derive(Clone, Debug, PartialEq)]
pub enum Value<'s> {
    Bool(bool),
    Float64(f64),
    Int64(i64),
    String(Cow<'s, str>),
}

impl<'val> Value<'val> {
    pub fn new<T: Into<Value<'val>>>(val: T) -> Self {
        val.into()
    }
}

impl Value<'_> {
    pub fn is_bool(&self) -> bool {
        matches!(self, Self::Bool(..))
    }

    pub fn is_int64(&self) -> bool {
        matches!(self, Self::Int64(..))
    }

    pub fn is_float64(&self) -> bool {
        matches!(self, Self::Float64(..))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(..))
    }

    pub fn get_bool(&self) -> Result<bool, errors::Error> {
        if let Self::Bool(i) = self {
            Ok(*i)
        } else {
            Err(errors::Error::ValueAccess("bool"))
        }
    }

    pub fn get_int64(&self) -> Result<i64, errors::Error> {
        if let Self::Int64(i) = self {
            Ok(*i)
        } else {
            Err(errors::Error::ValueAccess("int64"))
        }
    }

    pub fn get_float64(&self) -> Result<f64, errors::Error> {
        if let Self::Float64(i) = self {
            Ok(*i)
        } else {
            Err(errors::Error::ValueAccess("float64"))
        }
    }

    pub fn get_string(&self) -> Result<&str, errors::Error> {
        if let Self::String(i) = self {
            Ok(i)
        } else {
            Err(errors::Error::ValueAccess("string"))
        }
    }
}

macro_rules! impl_from_direct {
    ($ty:ty, $variant:ident) => {
        impl<'s> From<$ty> for Value<'s> {
            fn from(value: $ty) -> Self {
                Self::$variant(value)
            }
        }
    };
}

impl_from_direct!(bool, Bool);
impl_from_direct!(i64, Int64);
impl_from_direct!(f64, Float64);
impl_from_direct!(Cow<'s, str>, String);

impl<'s> From<&'s str> for Value<'s> {
    fn from(value: &'s str) -> Self {
        Value::String(Cow::Borrowed(value))
    }
}

impl<'s> From<String> for Value<'s> {
    fn from(value: String) -> Self {
        Value::String(Cow::Owned(value))
    }
}
