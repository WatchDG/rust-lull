mod order_id;
mod order_instrument;
mod order_size;

pub use order_id::OrderId;
pub use order_instrument::OrderInstrument;
pub use order_size::OrderSize;

use crate::enums::{OrderSide, OrderType};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OID, OI, OS> {
    pub id: OrderId<OID>,
    pub side: OrderSide,
    pub r#type: OrderType,
    pub instrument: OrderInstrument<OI>,
    pub size: OrderSize<OS>,
}

impl<OID, OI, OS> Order<OID, OI, OS> {
    pub fn new(
        id: OrderId<OID>,
        side: OrderSide,
        r#type: OrderType,
        instrument: OrderInstrument<OI>,
        size: OrderSize<OS>,
    ) -> Self {
        Self {
            id,
            side,
            r#type,
            instrument,
            size,
        }
    }
}
