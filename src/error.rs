use crate::receipt::{
    dollar_value::DollarValue,
    ratio::{self, Ratio},
};
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("subtotal mismatch: got {got}, expected {expected}")]
    SubtotalMismatch {
        got: DollarValue,
        expected: DollarValue,
    },
    #[error("ratios must add up to 1; got {ratios:?}")]
    SumMismatch {
        ratios: Vec<Ratio>,
        #[source]
        source: ratio::Error,
    },
    #[error("fldsjfsl")]
    Ratio(#[from] ratio::Error),
    #[error("Custom error")]
    Custom(#[from] Box<dyn std::error::Error>),
}
