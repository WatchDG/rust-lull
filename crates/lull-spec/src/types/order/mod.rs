mod order_id;
mod order_instrument;
mod order_size;
mod order_type;
mod order_type_limit;
mod order_type_market;

pub use order_id::OrderId;
pub use order_instrument::OrderInstrument;
pub use order_size::OrderSize;
pub use order_type::OrderType;
pub use order_type_limit::OrderTypeLimit;
pub use order_type_market::OrderTypeMarket;

use crate::enums::OrderSide;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OID, OT, OI, OS> {
    pub id: OrderId<OID>,
    pub side: OrderSide,
    pub r#type: OrderType<OT>,
    pub instrument: OrderInstrument<OI>,
    pub size: OrderSize<OS>,
}

impl<OID, OT, OI, OS> Order<OID, OT, OI, OS> {
    pub fn new(
        id: OrderId<OID>,
        side: OrderSide,
        r#type: OrderType<OT>,
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
