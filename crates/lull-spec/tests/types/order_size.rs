use std::collections::HashSet;

use lull_spec::enums::OrderSizeRef;
use lull_spec::types::{Lots, OrderSize, Quantity};

type TestOrderSize = OrderSize<OrderSizeRef<i64, i64>>;

#[test]
fn equal_sizes_are_equal() {
    let left: TestOrderSize = OrderSize::new(OrderSizeRef::Quantity(Quantity::new(100_i64)));
    let right: TestOrderSize = OrderSize::new(OrderSizeRef::Quantity(Quantity::new(100_i64)));
    assert_eq!(left, right);
}

#[test]
fn distinct_sizes_are_not_equal() {
    let left: TestOrderSize = OrderSize::new(OrderSizeRef::Quantity(Quantity::new(100_i64)));
    let right: TestOrderSize = OrderSize::new(OrderSizeRef::Quantity(Quantity::new(101_i64)));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let size: TestOrderSize = OrderSize::new(OrderSizeRef::Quantity(Quantity::new(100_i64)));
    assert_eq!(size.clone(), size);
}

#[test]
fn equal_sizes_hash_to_the_same_bucket() {
    let mut sizes = HashSet::new();
    sizes.insert(OrderSize::new(OrderSizeRef::Quantity(Quantity::new(
        100_i64,
    ))));
    sizes.insert(OrderSize::new(OrderSizeRef::Quantity(Quantity::new(
        100_i64,
    ))));
    sizes.insert(OrderSize::new(OrderSizeRef::Quantity(Quantity::new(
        101_i64,
    ))));
    sizes.insert(OrderSize::new(OrderSizeRef::Lots(Lots::new(10_i64))));
    assert_eq!(sizes.len(), 3);
}
