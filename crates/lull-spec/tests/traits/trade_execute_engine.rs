use lull_spec::enums::{InstrumentRef, OrderSide, OrderSizeRef, OrderType};
use lull_spec::traits::TradeExecuteEngine;
use lull_spec::types::{InstrumentId, Order, OrderId, OrderInstrument, OrderSize, Quantity};

type TestOrder = Order<String, InstrumentRef<String>, OrderSizeRef<i64, i64>>;

struct RecordingEngine {
    placed: Vec<TestOrder>,
    cancelled: Vec<OrderId<String>>,
}

impl TradeExecuteEngine<String, InstrumentRef<String>, OrderSizeRef<i64, i64>> for RecordingEngine {
    type Error = ();

    fn place_order(&mut self, order: TestOrder) -> Result<(), Self::Error> {
        self.placed.push(order);
        Ok(())
    }

    fn cancel_order(&mut self, id: OrderId<String>) -> Result<(), Self::Error> {
        self.cancelled.push(id);
        Ok(())
    }
}

fn sample_order(id: &str) -> TestOrder {
    Order::new(
        OrderId::new(String::from(id)),
        OrderSide::Buy,
        OrderType::Limit,
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from("inst-1")))),
        OrderSize::new(OrderSizeRef::Quantity(Quantity::new(100_i64))),
    )
}

#[test]
fn place_order_accepts_an_order() {
    let mut engine = RecordingEngine {
        placed: Vec::new(),
        cancelled: Vec::new(),
    };
    engine.place_order(sample_order("ord-1")).unwrap();
    assert_eq!(engine.placed, vec![sample_order("ord-1")]);
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
