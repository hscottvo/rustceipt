use std::{iter::Sum, ops::Add};

use derive_more::{AsRef, Display};

use crate::error::{Error, Result};
#[derive(Debug, Clone, Copy, Display, PartialEq, AsRef)]
pub struct DollarValue(f32);
impl DollarValue {
    pub unsafe fn new_unchecked(value: f32) -> DollarValue {
        DollarValue(value)
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

impl TryFrom<f32> for DollarValue {
    type Error = Error;
    fn try_from(value: f32) -> Result<DollarValue> {
        Ok(DollarValue(value))
    }
}
