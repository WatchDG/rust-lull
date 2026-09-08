mod candlestick_id;
mod candlestick_interval;

pub use candlestick_id::CandlestickId;
pub use candlestick_interval::CandlestickInterval;

use crate::types::date_time::DateTime;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Candlestick<CSID, CSI, DT> {
    pub id: CandlestickId<CSID>,
    pub interval: CandlestickInterval<CSI>,
    pub datetime: DateTime<DT>,
}

impl<CSID, CSI, DT> Candlestick<CSID, CSI, DT> {
    pub fn new(
        id: CandlestickId<CSID>,
        interval: CandlestickInterval<CSI>,
        datetime: DateTime<DT>,
    ) -> Self {
        Self {
            id,
            interval,
            datetime,
        }
    }
}
