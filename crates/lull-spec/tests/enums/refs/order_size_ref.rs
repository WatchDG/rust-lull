use std::collections::HashSet;

use lull_spec::enums::OrderSizeRef;
use lull_spec::types::Quantity;

type TestOrderSizeRef = OrderSizeRef<i64>;

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
fn clone_preserves_equality() {
    let size: TestOrderSizeRef = OrderSizeRef::Quantity(Quantity::new(100_i64));
    assert_eq!(size.clone(), size);
}

#[test]
fn equal_refs_hash_to_the_same_bucket() {
    let mut refs = HashSet::new();
    refs.insert(OrderSizeRef::Quantity(Quantity::new(100_i64)));
    refs.insert(OrderSizeRef::Quantity(Quantity::new(100_i64)));
    refs.insert(OrderSizeRef::Quantity(Quantity::new(101_i64)));
    assert_eq!(refs.len(), 2);
}
