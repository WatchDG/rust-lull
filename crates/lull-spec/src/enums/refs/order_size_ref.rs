use crate::types::{Lots, Quantity};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum OrderSizeRef<QTY, LTS> {
    Quantity(Quantity<QTY>),
    Lots(Lots<LTS>),
}
