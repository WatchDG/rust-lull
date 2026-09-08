mod candlestick_datetime;
mod candlestick_id;
mod candlestick_instrument;
mod candlestick_interval;

pub use candlestick_datetime::CandlestickDateTime;
pub use candlestick_id::CandlestickId;
pub use candlestick_instrument::CandlestickInstrument;
pub use candlestick_interval::CandlestickInterval;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Candlestick<CSID, CI, CSI, CDT> {
    pub id: CandlestickId<CSID>,
    pub instrument: CandlestickInstrument<CI>,
    pub interval: CandlestickInterval<CSI>,
    pub datetime: CandlestickDateTime<CDT>,
}

impl<CSID, CI, CSI, CDT> Candlestick<CSID, CI, CSI, CDT> {
    pub fn new(
        id: CandlestickId<CSID>,
        instrument: CandlestickInstrument<CI>,
        interval: CandlestickInterval<CSI>,
        datetime: CandlestickDateTime<CDT>,
    ) -> Self {
        Self {
            id,
            instrument,
            interval,
            datetime,
        }
    }
}
