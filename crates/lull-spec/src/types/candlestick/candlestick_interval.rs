#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandlestickInterval<CSI>(CSI);

impl<CSI> CandlestickInterval<CSI> {
    pub fn new(inner: CSI) -> Self {
        Self(inner)
    }
}
