#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandlestickId<CSID>(CSID);

impl<CSID> CandlestickId<CSID> {
    pub fn new(inner: CSID) -> Self {
        Self(inner)
    }
}
