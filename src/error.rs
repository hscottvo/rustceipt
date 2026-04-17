use crate::receipt::{dollar_value::DollarValue, ratio::Ratio};
use thiserror::Error;
pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug, Error)]
pub enum Error {
    #[error("fsdlkfj")]
    SubtotalMismatch {
        got: DollarValue,
        expected: DollarValue,
    },
    #[error("Ratios must add up to 1, got {0}")]
    SplitRatioMismatch(Ratio),
    #[error("Ratios must be from range 0 to 1, got {0}")]
    RatioRange(f32),
    #[error("Custom error")]
    Custom(#[from] Box<dyn std::error::Error>),
}
