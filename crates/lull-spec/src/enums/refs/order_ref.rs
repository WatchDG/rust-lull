use crate::types::OrderId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderRef<OID> {
    Id(OrderId<OID>),
}
