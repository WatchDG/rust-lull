use crate::types::money::Money;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderTypeLimit<MV, MC> {
    pub price: Money<MV, MC>,
}

impl<MV, MC> OrderTypeLimit<MV, MC> {
    pub fn new(price: Money<MV, MC>) -> Self {
        Self { price }
    }
}
