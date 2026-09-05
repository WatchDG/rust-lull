use crate::types::Quantity;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderSizeRef<QTY> {
    Quantity(Quantity<QTY>),
}
