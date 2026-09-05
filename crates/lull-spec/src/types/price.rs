use super::price_currency::PriceCurrency;
use super::price_value::PriceValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Price<PV, PC> {
    pub price: PriceValue<PV>,
    pub currency: PriceCurrency<PC>,
}

impl<PV, PC> Price<PV, PC> {
    pub fn new(price: PriceValue<PV>, currency: PriceCurrency<PC>) -> Self {
        Self { price, currency }
    }
}
