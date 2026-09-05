use std::collections::HashSet;

use lull_spec::types::{Order, OrderId};

type TestOrder = Order<String>;

fn order(id: &str) -> TestOrder {
    Order::new(OrderId::new(String::from(id)))
}

#[test]
fn equal_ids_are_equal_orders() {
    assert_eq!(order("ord-1"), order("ord-1"));
}

#[test]
fn distinct_ids_are_not_equal_orders() {
    assert_ne!(order("ord-1"), order("ord-2"));
}

#[test]
fn clone_preserves_equality() {
    let order = order("ord-1");
    assert_eq!(order.clone(), order);
}

#[test]
fn equal_orders_hash_to_the_same_bucket() {
    let mut orders = HashSet::new();
    orders.insert(order("ord-1"));
    orders.insert(order("ord-1"));
    orders.insert(order("ord-2"));
    assert_eq!(orders.len(), 2);
}
