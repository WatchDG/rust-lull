use lull_spec::enums::{CurrencyRef, InstrumentRef, OrderSide, OrderSizeRef, OrderTypeRef};
use lull_spec::traits::ExecuteEngine;
use lull_spec::types::{
    CurrencyCode, InstrumentId, Money, MoneyCurrency, MoneyValue, OrderId, OrderInstrument,
    OrderSize, OrderType, OrderTypeLimit, PlaceOrder, Quantity,
};

type TestOrderTypeRef = OrderTypeRef<i64, CurrencyRef<String, [u8; 3]>>;
type TestPlaceOrder =
    PlaceOrder<String, TestOrderTypeRef, InstrumentRef<String>, OrderSizeRef<i64, i64>>;

struct RecordingEngine {
    placed: Vec<TestPlaceOrder>,
    cancelled: Vec<OrderId<String>>,
}

impl ExecuteEngine<String, TestOrderTypeRef, InstrumentRef<String>, OrderSizeRef<i64, i64>>
    for RecordingEngine
{
    type Error = ();

    fn place_order(&mut self, order: TestPlaceOrder) -> Result<(), Self::Error> {
        self.placed.push(order);
        Ok(())
    }

    fn cancel_order(&mut self, id: OrderId<String>) -> Result<(), Self::Error> {
        self.cancelled.push(id);
        Ok(())
    }
}

fn sample_place_order(id: Option<&str>) -> TestPlaceOrder {
    PlaceOrder::new(
        id.map(|id| OrderId::new(String::from(id))),
        OrderSide::Buy,
        OrderType::new(OrderTypeRef::Limit(OrderTypeLimit::new(Money::new(
            MoneyValue::new(100_i64),
            MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"USD"))),
        )))),
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from("inst-1")))),
        OrderSize::new(OrderSizeRef::Quantity(Quantity::new(100_i64))),
    )
}

#[test]
fn place_order_accepts_a_place_order() {
    let mut engine = RecordingEngine {
        placed: Vec::new(),
        cancelled: Vec::new(),
    };
    engine.place_order(sample_place_order(Some("ord-1"))).unwrap();
    assert_eq!(engine.placed, vec![sample_place_order(Some("ord-1"))]);
}

#[test]
fn cancel_order_accepts_an_order_id() {
    let mut engine = RecordingEngine {
        placed: Vec::new(),
        cancelled: Vec::new(),
    };
    engine
        .cancel_order(OrderId::new(String::from("ord-1")))
        .unwrap();
    assert_eq!(engine.cancelled, vec![OrderId::new(String::from("ord-1"))]);
}
