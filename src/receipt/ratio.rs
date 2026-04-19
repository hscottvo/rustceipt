use std::fmt::Display;

use derive_more::AsRef;
use thiserror::Error;

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("value is not finite: {0}")]
    NotFinite(f32),
    #[error("value less than minimum of 0: {0}")]
    TooSmall(f32),
    #[error("value greater than maximum of 1: {0}")]
    TooLarge(f32),
}

#[derive(Debug, Clone, Copy, PartialEq, AsRef)]
pub struct Ratio(f32);

impl Ratio {
    pub unsafe fn new_unchecked(value: f32) -> Ratio {
        Ratio(value)
    }
    pub fn new(value: f32) -> Result<Ratio> {
        if !value.is_finite() {
            Err(Error::NotFinite(value))
        } else if value < 0. {
            Err(Error::TooSmall(value))
        } else if value > 1. {
            Err(Error::TooLarge(value))
        } else {
            Ok(Self(value))
        }
    }

    pub fn sum<I>(items: I) -> Result<Ratio>
    where
        I: IntoIterator<Item = Ratio>,
    {
        let sum: f32 = items.into_iter().map(|item| item.inner()).sum();
        Ratio::new(sum)
    }

    pub fn inner(self) -> f32 {
        self.0
    }
}

impl Display for Ratio {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.3}", self.0)
    }
}

impl From<Ratio> for f32 {
    fn from(r: Ratio) -> f32 {
        r.0
    }
}

impl TryFrom<f32> for Ratio {
    type Error = Error;
    fn try_from(value: f32) -> Result<Ratio> {
        Ratio::new(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::close;

    #[test]
    fn test_valid_ratio() {
        let value = Ratio::new(0.234).unwrap();
        assert!(close(0.234, value.inner()));
    }

    #[test]
    fn test_lower_bound_ratio() {
        let value = Ratio::new(0.).unwrap();
        assert!(close(0., value.inner()));
    }

    #[test]
    fn test_upper_bound_ratio() {
        let value = Ratio::new(1.).unwrap();
        assert!(close(1., value.inner()));
    }

    #[test]
    fn test_too_small_ratio() {
        let value = Ratio::new(-0.0001);
        assert!(matches!(value, Err(Error::TooSmall(_))));
    }

    #[test]
    fn test_too_large_ratio() {
        let value = Ratio::new(1.0001);
        assert!(matches!(value, Err(Error::TooLarge(_))));
    }

    #[test]
    fn test_as_ref() {
        let value = Ratio::new(0.53).unwrap();
        assert!(close(0.53, value.into()));
    }

    #[test]
    fn test_into_f32() {
        let value = Ratio::new(0.5).unwrap();
        assert!(close(0.5, value.into()));
    }

    #[test]
    fn test_invalid_value() {
        assert!(matches!(Ratio::new(f32::NAN), Err(Error::NotFinite(_))));
        assert!(matches!(
            Ratio::new(f32::INFINITY),
            Err(Error::NotFinite(_))
        ));
        assert!(matches!(
            Ratio::new(f32::NEG_INFINITY),
            Err(Error::NotFinite(_))
        ));
    }

    #[test]
    fn test_display() {
        let r = Ratio::new(0.123456).unwrap();
        let s = format!("{}", r);
        assert_eq!(s, "0.123")
    }

    #[test]
    fn test_try_from() {
        assert!(Ratio::try_from(0.5f32).is_ok());
        assert!(Ratio::try_from(2.0f32).is_err());
    }
}
