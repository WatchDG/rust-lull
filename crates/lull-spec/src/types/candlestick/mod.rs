mod candlestick_datetime;
mod candlestick_id;
mod candlestick_instrument;
mod candlestick_interval;
mod candlestick_price;
mod candlestick_volume;

pub use candlestick_datetime::CandlestickDateTime;
pub use candlestick_id::CandlestickId;
pub use candlestick_instrument::CandlestickInstrument;
pub use candlestick_interval::CandlestickInterval;
pub use candlestick_price::CandlestickPrice;
pub use candlestick_volume::CandlestickVolume;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Candlestick<CSID, CI, CSI, CDT, CSP, CSV> {
    pub id: CandlestickId<CSID>,
    pub instrument: CandlestickInstrument<CI>,
    pub interval: CandlestickInterval<CSI>,
    pub datetime: CandlestickDateTime<CDT>,
    pub open_price: CandlestickPrice<CSP>,
    pub high_price: CandlestickPrice<CSP>,
    pub low_price: CandlestickPrice<CSP>,
    pub close_price: Option<CandlestickPrice<CSP>>,
    pub volume: Option<CandlestickVolume<CSV>>,
}

impl<CSID, CI, CSI, CDT, CSP, CSV> Candlestick<CSID, CI, CSI, CDT, CSP, CSV> {
    pub fn new(
        id: CandlestickId<CSID>,
        instrument: CandlestickInstrument<CI>,
        interval: CandlestickInterval<CSI>,
        datetime: CandlestickDateTime<CDT>,
        open_price: CandlestickPrice<CSP>,
        high_price: CandlestickPrice<CSP>,
        low_price: CandlestickPrice<CSP>,
        close_price: Option<CandlestickPrice<CSP>>,
        volume: Option<CandlestickVolume<CSV>>,
    ) -> Self {
        Self {
            id,
            instrument,
            interval,
            datetime,
            open_price,
            high_price,
            low_price,
            close_price,
            volume,
        }
    }
}
