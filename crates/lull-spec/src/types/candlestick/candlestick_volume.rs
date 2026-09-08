#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandlestickVolume<CSV>(CSV);

impl<CSV> CandlestickVolume<CSV> {
    pub fn new(inner: CSV) -> Self {
        Self(inner)
    }
}
