use crate::types::{CurrencyCode, CurrencyId};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CurrencyRef<CID, CC> {
    Id(CurrencyId<CID>),
    Code(CurrencyCode<CC>),
}
