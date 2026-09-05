use super::currency_code::CurrencyCode;
use super::currency_id::CurrencyId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Currency<CID, CC> {
    pub id: CurrencyId<CID>,
    pub code: CurrencyCode<CC>,
}

impl<CID, CC> Currency<CID, CC> {
    pub fn new(id: CurrencyId<CID>, code: CurrencyCode<CC>) -> Self {
        Self { id, code }
    }
}
