use super::money_currency::MoneyCurrency;
use super::money_value::MoneyValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Money<MV, MC> {
    pub amount: MoneyValue<MV>,
    pub currency: MoneyCurrency<MC>,
}

impl<MV, MC> Money<MV, MC> {
    pub fn new(amount: MoneyValue<MV>, currency: MoneyCurrency<MC>) -> Self {
        Self { amount, currency }
    }
}
