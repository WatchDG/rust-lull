#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandlestickPrice<CSP>(CSP);

impl<CSP> CandlestickPrice<CSP> {
    pub fn new(inner: CSP) -> Self {
        Self(inner)
    }
}
