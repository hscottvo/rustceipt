use std::{iter::Sum, ops::Add};

use derive_more::Display;
use thiserror::Error;
type Result<T> = std::result::Result<T, Error>;
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

#[derive(Debug, Clone, Copy, Display, PartialEq)]
pub struct DollarValue(f32);

impl AsRef<f32> for DollarValue {
    fn as_ref(&self) -> &f32 {
        &self.0
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

pub struct Item {
    name: String,
    value: DollarValue,
}

impl Item {
    fn new(name: impl Into<String>, value: DollarValue) -> Item {
        let name = name.into();
        Item { name, value }
    }
}

// TODO: Typestate, based on what's there and what's missing?
pub struct Receipt {
    items: Vec<Item>,
    subtotal: Option<DollarValue>,
    total: Option<DollarValue>,
}

impl Receipt {
    pub fn try_new(items: Vec<Item>, subtotal: Option<DollarValue>) -> Result<Receipt> {
        let total_items_value: DollarValue = items.iter().map(|item| item.value).sum();

        if let Some(subtotal) = subtotal
            && total_items_value != subtotal
        {
            return Err(Error::SubtotalMismatch {
                got: total_items_value,
                expected: subtotal,
            });
        }

        Ok(Receipt {
            items,
            subtotal,
            total: None,
        })
    }

    pub fn names(&self) -> Vec<String> {
        self.items.iter().map(|item| item.name.clone()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_items_add_to_correct_subtotal() -> Result<()> {
        let items = vec![
            Item::new("foo", DollarValue(1.)),
            Item::new("bar", DollarValue(2.)),
        ];
        let subtotal = Some(DollarValue(3.));
        let receipt = Receipt::try_new(items, subtotal)?;
        assert_eq!(receipt.names(), vec!["foo".to_string(), "bar".to_string()]);
        Ok(())
    }
}
