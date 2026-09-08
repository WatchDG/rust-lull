mod candlestick_id;

pub use candlestick_id::CandlestickId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Candlestick<CSID> {
    pub id: CandlestickId<CSID>,
}

impl<CSID> Candlestick<CSID> {
    pub fn new(id: CandlestickId<CSID>) -> Self {
        Self { id }
    }
}
