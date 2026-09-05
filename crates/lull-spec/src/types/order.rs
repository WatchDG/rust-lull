use super::order_id::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Order<OID> {
    pub id: OrderId<OID>,
}

impl<OID> Order<OID> {
    pub fn new(id: OrderId<OID>) -> Self {
        Self { id }
    }
}
