pub(crate) mod dollar_value;
pub(crate) mod ratio;

use crate::{
    error::{Error, Result},
    receipt::ratio::Ratio,
};
use dollar_value::DollarValue;
use rust_decimal::{Decimal, dec};

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

#[derive(Debug, Clone)]
pub struct UserSplit {
    username: String,
    ratio: Ratio,
}

#[derive(Debug, Clone)]
pub struct UserSplitResult {
    username: String,
    amount: DollarValue,
}

impl UserSplitResult {
    fn value(&self) -> Decimal {
        self.amount.inner()
    }
}
// TODO: Typestate, based on what's there and what's missing?
pub struct Receipt {
    items: Vec<Item>,
    total: DollarValue,
}

impl Receipt {
    pub fn try_new(items: Vec<Item>, total: DollarValue) -> Result<Receipt> {
        let total_items_value: DollarValue = items.iter().map(|item| item.value).sum();

        if total_items_value != total {
            return Err(Error::TotalMismatch {
                got: total_items_value,
                expected: total,
            });
        }

        Ok(Receipt { items, total })
    }

    pub fn names(&self) -> Vec<String> {
        self.items.iter().map(|item| item.name.clone()).collect()
    }
    pub fn split(&self, splits: Vec<UserSplit>) -> Result<Vec<UserSplitResult>> {
        let ratios: Vec<Ratio> = splits.iter().map(|split| split.ratio).collect();
        Self::validate_full_ratio(ratios)?;

        let results: Vec<UserSplitResult> = splits
            .iter()
            .map(|split| UserSplitResult {
                username: split.username.clone(),
                amount: (split.ratio.inner() * self.total.inner()).into(),
            })
            .collect();
        Ok(results)
    }

    fn validate_full_ratio(ratios: Vec<Ratio>) -> Result<()> {
        let total_ratio =
            Ratio::sum(ratios.clone()).map_err(|_| Error::RatioSumNotOne(ratios.clone()))?;
        if total_ratio != Ratio::try_new(dec!(1))? {
            return Err(Error::RatioSumNotOne(ratios));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn receipt_items_add_to_correct_subtotal() -> Result<()> {
        let items = vec![
            Item::new("foo", DollarValue::from(1)),
            Item::new("bar", DollarValue::new(dec!(2))),
        ];
        let total = DollarValue::from(3);
        let receipt = Receipt::try_new(items, total)?;
        assert_eq!(receipt.names(), vec!["foo".to_string(), "bar".to_string()]);
        Ok(())
    }

    #[test]
    fn receipt_split_flow() -> Result<()> {
        let items = vec![Item::new("nothing", 20f32.try_into()?)];
        let total = DollarValue::from(20);
        let receipt = Receipt::try_new(items, total)?;

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
        assert_eq!(splits[0].value(), dec!(12));
        assert_eq!(splits[1].username, "B");
        assert_eq!(splits[1].value(), dec!(8));
        Ok(())
    }

    #[test]
    fn split_sum_too_small() -> Result<()> {
        let items = vec![];
        let total = DollarValue::from(0);
        let receipt = Receipt::try_new(items, total)?;

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

        assert!(matches!(
            receipt.split(user_splits),
            Err(Error::RatioSumNotOne(_))
        ));

        Ok(())
    }

    #[test]
    fn split_sum_too_large() -> Result<()> {
        let items = vec![];
        let total = DollarValue::try_from(0.)?;
        let receipt = Receipt::try_new(items, total)?;

        let user_splits = vec![
            UserSplit {
                username: "A".to_string(),
                ratio: Ratio::try_from(0.5)?,
            },
            UserSplit {
                username: "B".to_string(),
                ratio: Ratio::try_from(0.6)?,
            },
        ];

        assert!(matches!(
            receipt.split(user_splits),
            Err(Error::RatioSumNotOne(_))
        ));

        Ok(())
    }

    #[test]
    fn split_has_extra_cents() -> Result<()> {
        let items = vec![Item::new("test", 1.into())];
        let total = DollarValue::from(1);
        let receipt = Receipt::try_new(items, total)?;

        let user_splits = vec![
            UserSplit {
                username: "A".to_string(),
                ratio: Ratio::try_from(1. / 3.)?,
            },
            UserSplit {
                username: "B".to_string(),
                ratio: Ratio::try_from(1. / 3.)?,
            },
            UserSplit {
                username: "C".to_string(),
                ratio: Ratio::try_from(1. / 3.)?,
            },
        ];

        assert_eq!(
            receipt
                .split(user_splits)?
                .iter()
                .map(|result| result.amount.inner())
                .collect::<Vec<Decimal>>(),
            [dec!(0.34), dec!(0.33), dec!(0.33)]
        );

        Ok(())
    }
}
