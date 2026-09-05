use crate::types::{OrderId, PlaceOrder};

pub trait ExecuteEngine<OID, OT, OI, OS> {
    type Error;

    fn place_order(&mut self, order: PlaceOrder<OID, OT, OI, OS>) -> Result<(), Self::Error>;

    fn cancel_order(&mut self, id: OrderId<OID>) -> Result<(), Self::Error>;
}
