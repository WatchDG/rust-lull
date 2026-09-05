use super::currency_code::CurrencyCode;
use super::currency_id::CurrencyId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Currency<CID, CC> {
    Id(CurrencyId<CID>),
    Code(CurrencyCode<CC>),
}
