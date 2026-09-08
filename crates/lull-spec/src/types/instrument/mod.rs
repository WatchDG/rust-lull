mod instrument_id;
mod instrument_name;
mod instrument_ticker;

pub use instrument_id::InstrumentId;
pub use instrument_name::InstrumentName;
pub use instrument_ticker::InstrumentTicker;

use crate::types::quantity::Quantity;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instrument<IID, IN, IT, QTY> {
    pub id: InstrumentId<IID>,
    pub name: InstrumentName<IN>,
    pub ticker: InstrumentTicker<IT>,
    pub lot_size: Quantity<QTY>,
}

impl<IID, IN, IT, QTY> Instrument<IID, IN, IT, QTY> {
    pub fn new(
        id: InstrumentId<IID>,
        name: InstrumentName<IN>,
        ticker: InstrumentTicker<IT>,
        lot_size: Quantity<QTY>,
    ) -> Self {
        Self {
            id,
            name,
            ticker,
            lot_size,
        }
    }
}
