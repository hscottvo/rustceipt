use derive_more::{AsRef, Display};

use crate::error::{Error, Result};

#[derive(Debug, Clone, Copy, Display, PartialEq, AsRef)]
pub struct Ratio(f32);

impl Ratio {
    pub unsafe fn new_unchecked(value: f32) -> Ratio {
        Ratio(value)
    }

    pub fn new(value: f32) -> Result<Ratio> {
        if value.is_nan() || !((-1.0 - f32::EPSILON)..=(1.0 + f32::EPSILON)).contains(&value) {
            Err(Error::RatioRange(value))
        } else {
            Ok(Self(value))
        }
    }

    pub fn inner(self) -> f32 {
        self.0
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
        assert_eq!(0.234, value.inner());
    }

    #[test]
    fn test_lower_bound_ratio() {
        let value = Ratio::new(-1.).unwrap();
        assert_eq!(-1., value.inner());
    }

    #[test]
    fn test_upper_bound_ratio() {
        let value = Ratio::new(1.).unwrap();
        assert_eq!(1., value.inner());
    }

    #[test]
    fn test_too_small_ratio() {
        let value = Ratio::new(-1.0001);
        assert!(matches!(value, Err(Error::RatioRange(_))));
    }

    #[test]
    fn test_too_large_ratio() {
        let value = Ratio::new(1.0001);
        assert!(matches!(value, Err(Error::RatioRange(_))));
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
}
