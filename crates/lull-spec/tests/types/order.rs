use std::collections::HashSet;

use lull_spec::enums::Side;
use lull_spec::types::{Order, OrderId};

type TestOrder = Order<String>;

fn order(id: &str, side: Side) -> TestOrder {
    Order::new(OrderId::new(String::from(id)), side)
}

#[test]
fn equal_fields_are_equal_orders() {
    assert_eq!(order("ord-1", Side::Buy), order("ord-1", Side::Buy));
}

#[test]
fn distinct_ids_are_not_equal_orders() {
    assert_ne!(order("ord-1", Side::Buy), order("ord-2", Side::Buy));
}

#[test]
fn distinct_sides_are_not_equal_orders() {
    assert_ne!(order("ord-1", Side::Buy), order("ord-1", Side::Sell));
}

#[test]
fn clone_preserves_equality() {
    let order = order("ord-1", Side::Buy);
    assert_eq!(order.clone(), order);
}

#[test]
fn equal_orders_hash_to_the_same_bucket() {
    let mut orders = HashSet::new();
    orders.insert(order("ord-1", Side::Buy));
    orders.insert(order("ord-1", Side::Buy));
    orders.insert(order("ord-2", Side::Buy));
    orders.insert(order("ord-1", Side::Sell));
    assert_eq!(orders.len(), 3);
}
