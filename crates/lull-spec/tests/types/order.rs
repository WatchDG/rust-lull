use std::collections::HashSet;

use lull_spec::enums::{InstrumentRef, Side};
use lull_spec::types::{InstrumentId, Order, OrderId, OrderInstrument};

type TestOrder = Order<String, InstrumentRef<String>>;

fn order(id: &str, side: Side, instrument_id: &str) -> TestOrder {
    Order::new(
        OrderId::new(String::from(id)),
        side,
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from(
            instrument_id,
        )))),
    )
}

#[test]
fn equal_fields_are_equal_orders() {
    assert_eq!(
        order("ord-1", Side::Buy, "inst-1"),
        order("ord-1", Side::Buy, "inst-1")
    );
}

#[test]
fn distinct_ids_are_not_equal_orders() {
    assert_ne!(
        order("ord-1", Side::Buy, "inst-1"),
        order("ord-2", Side::Buy, "inst-1")
    );
}

#[test]
fn distinct_sides_are_not_equal_orders() {
    assert_ne!(
        order("ord-1", Side::Buy, "inst-1"),
        order("ord-1", Side::Sell, "inst-1")
    );
}

#[test]
fn distinct_instruments_are_not_equal_orders() {
    assert_ne!(
        order("ord-1", Side::Buy, "inst-1"),
        order("ord-1", Side::Buy, "inst-2")
    );
}

#[test]
fn clone_preserves_equality() {
    let order = order("ord-1", Side::Buy, "inst-1");
    assert_eq!(order.clone(), order);
}

#[test]
fn equal_orders_hash_to_the_same_bucket() {
    let mut orders = HashSet::new();
    orders.insert(order("ord-1", Side::Buy, "inst-1"));
    orders.insert(order("ord-1", Side::Buy, "inst-1"));
    orders.insert(order("ord-2", Side::Buy, "inst-1"));
    orders.insert(order("ord-1", Side::Sell, "inst-1"));
    orders.insert(order("ord-1", Side::Buy, "inst-2"));
    assert_eq!(orders.len(), 4);
}
