#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CandlestickInstrument<CI>(CI);

impl<CI> CandlestickInstrument<CI> {
    pub fn new(inner: CI) -> Self {
        Self(inner)
    }
}
