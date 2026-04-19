use crate::receipt::{dollar_value::DollarValue, ratio::Ratio};
use rust_decimal::Decimal;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("total mismatch: expected {expected}, got {got}")]
    TotalMismatch {
        got: DollarValue,
        expected: DollarValue,
    },
    #[error("ratios must add up to 1, got {0:?}")]
    RatioSumNotOne(Vec<Ratio>),
    #[error("ratio value out of range (0..=1), got {0}")]
    RatioOutOfRange(Decimal),
    #[error("invalid ratio")]
    RatioInvalid,
    #[error("invalid dollar value")]
    DollarValueInvalid,
}
