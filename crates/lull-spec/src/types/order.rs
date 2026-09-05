use crate::enums::Side;

use super::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OID> {
    pub id: OrderId<OID>,
    pub side: Side,
}

impl<OID> Order<OID> {
    pub fn new(id: OrderId<OID>, side: Side) -> Self {
        Self { id, side }
    }
}
