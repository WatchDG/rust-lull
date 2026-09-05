use std::collections::HashSet;

use lull_spec::enums::OrderSizeRef;
use lull_spec::types::{Lots, Quantity};

type TestOrderSizeRef = OrderSizeRef<i64, i64>;

#[test]
fn equal_quantities_are_equal() {
    let left: TestOrderSizeRef = OrderSizeRef::Quantity(Quantity::new(100_i64));
    let right: TestOrderSizeRef = OrderSizeRef::Quantity(Quantity::new(100_i64));
    assert_eq!(left, right);
}

#[test]
fn distinct_quantities_are_not_equal() {
    let left: TestOrderSizeRef = OrderSizeRef::Quantity(Quantity::new(100_i64));
    let right: TestOrderSizeRef = OrderSizeRef::Quantity(Quantity::new(101_i64));
    assert_ne!(left, right);
}

#[test]
fn equal_lots_are_equal() {
    let left: TestOrderSizeRef = OrderSizeRef::Lots(Lots::new(10_i64));
    let right: TestOrderSizeRef = OrderSizeRef::Lots(Lots::new(10_i64));
    assert_eq!(left, right);
}

#[test]
fn distinct_lots_are_not_equal() {
    let left: TestOrderSizeRef = OrderSizeRef::Lots(Lots::new(10_i64));
    let right: TestOrderSizeRef = OrderSizeRef::Lots(Lots::new(11_i64));
    assert_ne!(left, right);
}

#[test]
fn quantity_is_not_lots() {
    let quantity: TestOrderSizeRef = OrderSizeRef::Quantity(Quantity::new(10_i64));
    let lots: TestOrderSizeRef = OrderSizeRef::Lots(Lots::new(10_i64));
    assert_ne!(quantity, lots);
}

#[test]
fn clone_preserves_equality() {
    let quantity: TestOrderSizeRef = OrderSizeRef::Quantity(Quantity::new(100_i64));
    let lots: TestOrderSizeRef = OrderSizeRef::Lots(Lots::new(10_i64));
    assert_eq!(quantity.clone(), quantity);
    assert_eq!(lots.clone(), lots);
}

#[test]
fn equal_refs_hash_to_the_same_bucket() {
    let mut refs = HashSet::new();
    refs.insert(OrderSizeRef::Quantity(Quantity::new(100_i64)));
    refs.insert(OrderSizeRef::Quantity(Quantity::new(100_i64)));
    refs.insert(OrderSizeRef::Lots(Lots::new(10_i64)));
    refs.insert(OrderSizeRef::Lots(Lots::new(10_i64)));
    refs.insert(OrderSizeRef::Lots(Lots::new(11_i64)));
    assert_eq!(refs.len(), 3);
}
