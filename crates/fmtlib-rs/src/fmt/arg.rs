use std::ffi::CString;

use super::{errors, Value};

#[derive(Clone, Debug)]
pub enum Arg<'s> {
    Pos(Value<'s>),
    Named(CString, Value<'s>),
}

impl<'arg> Arg<'arg> {
    pub fn positional<'s: 'arg, T: Into<Value<'s>>>(arg: T) -> Self {
        Self::Pos(arg.into())
    }

    pub fn named<'s: 'arg, S: Into<Vec<u8>>, T: Into<Value<'s>>>(name: S, arg: T) -> Self {
        let n = CString::new(name).expect("invalid argument name");
        Self::Named(n, arg.into())
    }
}

impl Arg<'_> {
    pub fn is_named(&self) -> bool {
        matches!(self, Arg::Named(..))
    }

    pub fn is_positional(&self) -> bool {
        matches!(self, Arg::Pos(..))
    }

    pub fn get_name(&self) -> Result<&std::ffi::CStr, errors::Error> {
        match self {
            Arg::Named(name, _) => Ok(name),
            _ => Err(errors::Error::InvalidOperation(
                "positional argument has no name",
            )),
        }
    }

    pub fn get_name_ptr(&self) -> Result<*const std::ffi::c_char, errors::Error> {
        match self {
            Arg::Named(name, _) => Ok(name.as_ptr()),
            _ => Err(errors::Error::InvalidOperation(
                "positional argument has no name",
            )),
        }
    }

    pub fn get_value(&self) -> &Value {
        match self {
            Arg::Named(_, value) => value,
            Arg::Pos(value) => value,
        }
    }
}
