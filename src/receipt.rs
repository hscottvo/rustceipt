pub(crate) mod dollar_value;
pub(crate) mod ratio;

use crate::{
    error::{Error, Result},
    receipt::ratio::Ratio,
};
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

pub struct UserSplit {
    username: String,
    ratio: Ratio,
}

pub struct UserSplitResult {
    username: String,
    amount: DollarValue,
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
    pub fn split(&self, splits: Vec<UserSplit>) -> Result<Vec<UserSplitResult>> {
        let split_total: f32 = splits.iter().map(|split| *split.ratio.as_ref()).sum();
        let total_ratio = Ratio::try_from(split_total)?;

        if (*total_ratio.as_ref() - 1.).abs() > f32::EPSILON {
            return Err(Error::SplitRatioMismatch(total_ratio));
        }

        let amounts: Vec<UserSplitResult> = splits
            .iter()
            .map(|split| UserSplitResult {
                username: split.username.clone(),
                amount: (*split.ratio.as_ref() * *self.subtotal.unwrap().as_ref()).into(),
            })
            .collect();
        Ok(amounts)
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_close;

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

    #[test]
    fn receipt_split_flow() -> Result<()> {
        let items = vec![Item::new("nothing", 20f32.into())];
        let subtotal = Some(unsafe { DollarValue::new_unchecked(20.) });
        let receipt = Receipt::try_new(items, subtotal)?;

        let user_splits = vec![
            UserSplit {
                username: "A".to_string(),
                ratio: Ratio::try_from(0.6)?,
            },
            UserSplit {
                username: "B".to_string(),
                ratio: Ratio::try_from(0.4)?,
            },
        ];

        let splits = receipt.split(user_splits)?;
        assert_eq!(splits[0].username, "A");
        assert_close(*splits[0].amount.as_ref(), 12.);
        assert_eq!(splits[1].username, "B");
        assert_close(*splits[1].amount.as_ref(), 8.);
        Ok(())
    }

    #[test]
    fn receipt_split_requires_full_split() -> Result<()> {
        let items = vec![];
        let subtotal = DollarValue::from(0.);
        let receipt = Receipt::try_new(items, Some(subtotal))?;

        let user_splits = vec![
            UserSplit {
                username: "A".to_string(),
                ratio: Ratio::try_from(0.5)?,
            },
            UserSplit {
                username: "B".to_string(),
                ratio: Ratio::try_from(0.4)?,
            },
        ];

        assert!(receipt.split(user_splits).is_err());

        Ok(())
    }
}
