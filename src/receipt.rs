pub(crate) mod dollar_value;
pub(crate) mod ratio;

use crate::{
    error::{Error, Result},
    receipt::ratio::Ratio,
};
use dollar_value::DollarValue;
use rust_decimal::{Decimal, RoundingStrategy, dec, prelude::ToPrimitive};

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
    pub fn try_new<I>(items: I, total: DollarValue) -> Result<Receipt>
    where
        I: IntoIterator<Item = Item>,
    {
        let items: Vec<Item> = items.into_iter().collect();
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
    pub fn split<I>(&self, splits: I) -> Result<Vec<UserSplitResult>>
    where
        I: IntoIterator<Item = UserSplit>,
    {
        let splits: Vec<UserSplit> = splits.into_iter().collect();
        let ratios: Vec<Ratio> = splits.iter().map(|split| split.ratio).collect();
        Self::validate_full_ratio(ratios)?;

        let results: Vec<UserSplitResult> = splits
            .iter()
            .map(|split| UserSplitResult {
                username: split.username.clone(),
                amount: (split.ratio.inner() * self.total.inner()).into(),
            })
            .collect();

        let extra_value = self.get_extra_value(&results).inner();

        let results = Self::distribute_extra_value(results, extra_value);

        Ok(results)
    }

    fn distribute_extra_value<I>(results: I, mut extra_value: Decimal) -> Vec<UserSplitResult>
    where
        I: IntoIterator<Item = UserSplitResult>,
    {
        let mut results: Vec<UserSplitResult> = results.into_iter().collect();
        let each_add = (extra_value / Decimal::from(results.len()))
            .round_dp_with_strategy(2, RoundingStrategy::ToZero);
        let cent = Decimal::try_from(0.01).expect("Failed to parse 0.01 into Decimal somehow");
        extra_value -= each_add;

        let dist_num = (extra_value / cent).trunc().to_usize().unwrap_or(0);

        for i in 0..usize::min(results.len(), dist_num) {
            results[i].amount = results[i].amount + DollarValue::new(cent);
        }

        results
    }

    fn get_extra_value(&self, results: &[UserSplitResult]) -> DollarValue {
        let total_split_value = results.iter().map(|result| result.amount).sum();
        println!("total_...{total_split_value}");
        self.total - total_split_value
    }

    fn validate_full_ratio<I>(ratios: I) -> Result<()>
    where
        I: IntoIterator<Item = Ratio>,
    {
        let ratios: Vec<Ratio> = ratios.into_iter().collect();
        let total_ratio = Ratio::sum(ratios.iter().copied())
            .map_err(|_| Error::RatioSumNotOne(ratios.clone()))?;

        let one = Ratio::try_new(dec!(1))?;
        let epsilon = dec!(0.000001);

        if (total_ratio.inner() - one.inner()).abs() > epsilon {
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
    fn receipt_items_as_array() -> Result<()> {
        let items = [
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

        let user_splits = [
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
                .map(|result| result.value())
                .collect::<Vec<Decimal>>(),
            [dec!(0.34), dec!(0.33), dec!(0.33)]
        );

        Ok(())
    }

    #[test]
    fn get_extra_cents() -> Result<()> {
        let receipt = Receipt {
            items: vec![],
            total: DollarValue::from(1),
        };

        let split_results = [
            UserSplitResult {
                username: "A".to_string(),
                amount: DollarValue::try_from(0.33)?,
            },
            UserSplitResult {
                username: "B".to_string(),
                amount: DollarValue::try_from(0.33)?,
            },
            UserSplitResult {
                username: "C".to_string(),
                amount: DollarValue::try_from(0.33)?,
            },
        ];
        let extra_value = receipt.get_extra_value(&split_results);
        assert_eq!(extra_value, DollarValue::try_from(0.01)?);

        let split_results = [
            UserSplitResult {
                username: "A".to_string(),
                amount: DollarValue::try_from(0.5)?,
            },
            UserSplitResult {
                username: "B".to_string(),
                amount: DollarValue::try_from(0.5)?,
            },
        ];
        let extra_value = receipt.get_extra_value(&split_results);
        assert_eq!(extra_value, DollarValue::from(0));

        Ok(())
    }

    #[test]
    fn validate_full_ratio() -> Result<()> {
        let ratios = unsafe {
            [
                Ratio::new_unchecked(dec!(0.1)),
                Ratio::new_unchecked(dec!(0.2)),
            ]
        };
        let result = Receipt::validate_full_ratio(ratios);
        assert!(matches!(result, Err(Error::RatioSumNotOne(_))));

        let ratios = unsafe {
            [
                Ratio::new_unchecked(dec!(0.8)),
                Ratio::new_unchecked(dec!(0.2)),
            ]
        };
        Receipt::validate_full_ratio(ratios)?;
        Ok(())
    }
}
