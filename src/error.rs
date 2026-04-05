use crate::receipt::DollarValue;
use thiserror::Error;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("fsdlkfj")]
    SubtotalMismatch {
        got: DollarValue,
        expected: DollarValue,
    },
    #[error("Custom error")]
    Custom(#[from] Box<dyn std::error::Error>),
}
