#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyId<CID>(CID);

impl<CID> CurrencyId<CID> {
    pub fn new(inner: CID) -> Self {
        Self(inner)
    }
}
