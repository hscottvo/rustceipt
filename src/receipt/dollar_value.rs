use std::{
    iter::Sum,
    ops::{Add, Mul},
};

use derive_more::{AsRef, Display};
use rust_decimal::{Decimal, dec};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, Display, PartialEq, AsRef)]
pub struct DollarValue(Decimal);

impl DollarValue {
    pub fn new(value: Decimal) -> DollarValue {
        DollarValue(value)
    }
    pub fn inner(self) -> Decimal {
        self.0
    }
}

impl Add for DollarValue {
    type Output = DollarValue;
    fn add(self, rhs: Self) -> Self::Output {
        DollarValue(self.0 + rhs.0)
    }
}

impl Sum for DollarValue {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(DollarValue(dec!(0)), |x, a| x + a)
    }
}

impl Mul<Decimal> for DollarValue {
    type Output = DollarValue;
    fn mul(self, rhs: Decimal) -> Self::Output {
        DollarValue(self.0 * rhs)
    }
}

impl From<Decimal> for DollarValue {
    fn from(value: Decimal) -> Self {
        Self::new(value)
    }
}

impl From<i32> for DollarValue {
    fn from(value: i32) -> Self {
        Self::new(value.into())
    }
}

impl TryFrom<f32> for DollarValue {
    type Error = Error;
    fn try_from(value: f32) -> Result<DollarValue> {
        let decimal = Decimal::try_from(value).map_err(|_| Error::DollarValueInvalid)?;
        Ok(DollarValue::new(decimal))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_negative_float() {
        let value = DollarValue::try_from(-12.432).unwrap();
        assert_eq!(value.inner(), dec!(-12.432));
    }

    #[test]
    fn test_as_ref() {
        let value = DollarValue(dec!(12));
        assert_eq!(value.as_ref(), &dec!(12));
    }

    #[test]
    fn test_mul() {
        let value = DollarValue::try_from(12.5).unwrap();
        assert_eq!(*(value * dec!(2)).as_ref(), dec!(25));
    }
}
