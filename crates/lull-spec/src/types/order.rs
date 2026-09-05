use crate::enums::Side;

use super::order_id::OrderId;
use super::order_instrument::OrderInstrument;
use super::order_size::OrderSize;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OID, OI, OS> {
    pub id: OrderId<OID>,
    pub side: Side,
    pub instrument: OrderInstrument<OI>,
    pub size: OrderSize<OS>,
}

impl<OID, OI, OS> Order<OID, OI, OS> {
    pub fn new(
        id: OrderId<OID>,
        side: Side,
        instrument: OrderInstrument<OI>,
        size: OrderSize<OS>,
    ) -> Self {
        Self {
            id,
            side,
            instrument,
            size,
        }
    }
}
