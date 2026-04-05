pub(crate) mod dollar_value;

use crate::error::{Error, Result};
use dollar_value::DollarValue;

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
            Item::new("foo", unsafe { DollarValue::new_unchecked(1.) }),
            Item::new("bar", unsafe { DollarValue::new_unchecked(2.) }),
        ];
        let subtotal = Some(unsafe { DollarValue::new_unchecked(3.) });
        let receipt = Receipt::try_new(items, subtotal)?;
        assert_eq!(receipt.names(), vec!["foo".to_string(), "bar".to_string()]);
        Ok(())
    }
}
