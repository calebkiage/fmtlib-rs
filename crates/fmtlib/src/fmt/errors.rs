use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid operation: {0}")]
    InvalidOperation(&'static str),
    #[error("formatting failed: {0}")]
    FormatFailed(String),
    #[error("incorrect value type access: value does not hold the type {0}")]
    ValueAccess(&'static str),
}
