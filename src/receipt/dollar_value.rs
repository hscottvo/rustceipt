use std::{iter::Sum, ops::Add};

use derive_more::{AsRef, Display};

#[derive(Debug, Clone, Copy, Display, PartialEq, AsRef)]
pub struct DollarValue(f32);
impl DollarValue {
    pub unsafe fn new_unchecked(value: f32) -> DollarValue {
        DollarValue(value)
    }
    #[cfg(test)]
    fn inner(self) -> f32 {
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
        iter.fold(DollarValue(0.), |x, a| x + a)
    }
}

impl<T> From<T> for DollarValue
where
    T: Into<f32>,
{
    fn from(value: T) -> DollarValue {
        let truncated_value = (value.into() * 100.0).round() / 100.0;
        DollarValue(truncated_value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assert_close;
    #[test]
    fn test_truncate_to_cents() {
        let value = DollarValue::from(1.2345);
        assert_eq!(DollarValue(1.23), value);
    }

    #[test]
    fn test_negative_float() {
        let value = DollarValue::from(-12.432);
        assert_close(value.inner(), -12.43);
    }

    #[test]
    fn test_parse_int() {
        let value = DollarValue::from(5i16);
        assert_close(value.inner(), 5.);
    }

    #[test]
    fn test_as_ref() {
        let value = DollarValue::from(12i16);
        assert_close(*value.as_ref(), 12f32);
    }
}
