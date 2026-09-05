use crate::types::order::OrderTypeLimit;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderTypeRef<MV, MC> {
    Market,
    Limit(OrderTypeLimit<MV, MC>),
}
