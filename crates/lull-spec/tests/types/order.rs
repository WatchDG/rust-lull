use std::collections::HashSet;

use lull_spec::enums::{InstrumentRef, OrderSizeRef, Side};
use lull_spec::types::{InstrumentId, Order, OrderId, OrderInstrument, OrderSize, Quantity};

type TestOrder = Order<String, InstrumentRef<String>, OrderSizeRef<i64>>;

fn order(id: &str, side: Side, instrument_id: &str, qty: i64) -> TestOrder {
    Order::new(
        OrderId::new(String::from(id)),
        side,
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from(
            instrument_id,
        )))),
        OrderSize::new(OrderSizeRef::Quantity(Quantity::new(qty))),
    )
}

#[test]
fn equal_fields_are_equal_orders() {
    assert_eq!(
        order("ord-1", Side::Buy, "inst-1", 100),
        order("ord-1", Side::Buy, "inst-1", 100)
    );
}

#[test]
fn distinct_ids_are_not_equal_orders() {
    assert_ne!(
        order("ord-1", Side::Buy, "inst-1", 100),
        order("ord-2", Side::Buy, "inst-1", 100)
    );
}

#[test]
fn distinct_sides_are_not_equal_orders() {
    assert_ne!(
        order("ord-1", Side::Buy, "inst-1", 100),
        order("ord-1", Side::Sell, "inst-1", 100)
    );
}

#[test]
fn distinct_instruments_are_not_equal_orders() {
    assert_ne!(
        order("ord-1", Side::Buy, "inst-1", 100),
        order("ord-1", Side::Buy, "inst-2", 100)
    );
}

#[test]
fn distinct_sizes_are_not_equal_orders() {
    assert_ne!(
        order("ord-1", Side::Buy, "inst-1", 100),
        order("ord-1", Side::Buy, "inst-1", 101)
    );
}

#[test]
fn clone_preserves_equality() {
    let order = order("ord-1", Side::Buy, "inst-1", 100);
    assert_eq!(order.clone(), order);
}

#[test]
fn equal_orders_hash_to_the_same_bucket() {
    let mut orders = HashSet::new();
    orders.insert(order("ord-1", Side::Buy, "inst-1", 100));
    orders.insert(order("ord-1", Side::Buy, "inst-1", 100));
    orders.insert(order("ord-2", Side::Buy, "inst-1", 100));
    orders.insert(order("ord-1", Side::Sell, "inst-1", 100));
    orders.insert(order("ord-1", Side::Buy, "inst-2", 100));
    orders.insert(order("ord-1", Side::Buy, "inst-1", 101));
    assert_eq!(orders.len(), 5);
}
