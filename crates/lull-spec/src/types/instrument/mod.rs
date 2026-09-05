mod instrument_id;
mod instrument_name;

pub use instrument_id::InstrumentId;
pub use instrument_name::InstrumentName;

use crate::types::quantity::Quantity;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instrument<IID, IN, QTY> {
    pub id: InstrumentId<IID>,
    pub name: InstrumentName<IN>,
    pub lot_size: Quantity<QTY>,
}

impl<IID, IN, QTY> Instrument<IID, IN, QTY> {
    pub fn new(
        id: InstrumentId<IID>,
        name: InstrumentName<IN>,
        lot_size: Quantity<QTY>,
    ) -> Self {
        Self {
            id,
            name,
            lot_size,
        }
    }
}
