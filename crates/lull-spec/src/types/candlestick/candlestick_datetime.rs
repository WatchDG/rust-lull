#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandlestickDateTime<CDT>(CDT);

impl<CDT> CandlestickDateTime<CDT> {
    pub fn new(inner: CDT) -> Self {
        Self(inner)
    }
}
