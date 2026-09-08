mod candlestick_id;
mod candlestick_interval;

pub use candlestick_id::CandlestickId;
pub use candlestick_interval::CandlestickInterval;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Candlestick<CSID, CSI> {
    pub id: CandlestickId<CSID>,
    pub interval: CandlestickInterval<CSI>,
}

impl<CSID, CSI> Candlestick<CSID, CSI> {
    pub fn new(id: CandlestickId<CSID>, interval: CandlestickInterval<CSI>) -> Self {
        Self { id, interval }
    }
}
