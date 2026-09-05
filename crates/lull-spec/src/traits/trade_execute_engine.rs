use crate::types::{Order, OrderId};

pub trait TradeExecuteEngine<OID, OI, OS> {
    type Error;

    fn place_order(&mut self, order: Order<OID, OI, OS>) -> Result<(), Self::Error>;

    fn cancel_order(&mut self, id: OrderId<OID>) -> Result<(), Self::Error>;
}
