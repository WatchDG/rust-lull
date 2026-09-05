use crate::enums::Side;

use super::order_id::OrderId;
use super::order_instrument::OrderInstrument;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OID, OI> {
    pub id: OrderId<OID>,
    pub side: Side,
    pub instrument: OrderInstrument<OI>,
}

impl<OID, OI> Order<OID, OI> {
    pub fn new(id: OrderId<OID>, side: Side, instrument: OrderInstrument<OI>) -> Self {
        Self {
            id,
            side,
            instrument,
        }
    }
}
