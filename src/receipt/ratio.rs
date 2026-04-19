use std::fmt::Display;

use derive_more::AsRef;
use rust_decimal::{Decimal, dec};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, PartialEq, AsRef)]
pub struct Ratio(Decimal);

impl Ratio {
    pub unsafe fn new_unchecked(value: Decimal) -> Ratio {
        Ratio(value)
    }
    pub fn try_new(value: Decimal) -> Result<Ratio> {
        if value.is_sign_negative() || value > dec!(1) {
            Err(Error::RatioOutOfRange(value))
        } else {
            Ok(Self(value))
        }
    }

    pub fn sum<I>(items: I) -> Result<Ratio>
    where
        I: IntoIterator<Item = Ratio>,
    {
        let sum: Decimal = items.into_iter().map(|item| item.inner()).sum();
        Ratio::try_new(sum)
    }

    pub fn inner(self) -> Decimal {
        self.0
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}

impl From<Ratio> for Decimal {
    fn from(r: Ratio) -> Decimal {
        r.0
    }
}

impl TryFrom<Decimal> for Ratio {
    type Error = Error;
    fn try_from(value: Decimal) -> Result<Ratio> {
        Ratio::try_new(value)
    }
}

impl TryFrom<f32> for Ratio {
    type Error = Error;
    fn try_from(value: f32) -> Result<Ratio> {
        let dec = Decimal::try_from(value).map_err(|_| Error::RatioInvalid)?;
        Ratio::try_from(dec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_ratio() {
        let value = Ratio::try_from(0.234).unwrap();
        assert_eq!(dec!(0.234), value.inner());
    }

    #[test]
    fn test_lower_bound_ratio() {
        let value = Ratio::try_from(0.).unwrap();
        assert_eq!(dec!(0), value.inner());
    }

    #[test]
    fn test_upper_bound_ratio() {
        let value = Ratio::try_from(1.).unwrap();
        assert_eq!(dec!(1), value.inner());
    }

    #[test]
    fn test_too_small_ratio() {
        let value = Ratio::try_from(-0.0001);
        assert!(matches!(value, Err(Error::RatioOutOfRange(_))));
    }

    #[test]
    fn test_too_large_ratio() {
        let value = Ratio::try_from(1.0001);
        assert!(matches!(value, Err(Error::RatioOutOfRange(_))));
    }

    #[test]
    fn test_as_ref() {
        let value = Ratio::try_from(0.53).unwrap();
        assert_eq!(dec!(0.53), value.into());
    }

    #[test]
    fn test_into_f32() {
        let value = Ratio::try_from(0.5).unwrap();
        assert_eq!(dec!(0.5), value.into());
    }

    #[test]
    fn test_invalid_value() {
        assert!(matches!(
            Ratio::try_from(f32::NAN),
            Err(Error::RatioInvalid)
        ));
        assert!(matches!(
            Ratio::try_from(f32::INFINITY),
            Err(Error::RatioInvalid)
        ));
        assert!(matches!(
            Ratio::try_from(f32::NEG_INFINITY),
            Err(Error::RatioInvalid)
        ));
    }

    #[test]
    fn test_display() {
        let r = Ratio::try_from(0.123456).unwrap();
        let s = format!("{}", r);
        assert_eq!(s, "0.123")
    }

    #[test]
    fn test_try_from() {
        assert!(Ratio::try_from(0.5f32).is_ok());
        assert!(Ratio::try_from(2.0f32).is_err());
    }
}
