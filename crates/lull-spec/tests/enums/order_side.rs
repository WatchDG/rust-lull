use std::collections::HashSet;

use lull_spec::enums::OrderSide;

#[test]
fn buy_is_not_sell() {
    assert_ne!(OrderSide::Buy, OrderSide::Sell);
}

#[test]
fn clone_preserves_equality() {
    assert_eq!(OrderSide::Buy.clone(), OrderSide::Buy);
    assert_eq!(OrderSide::Sell.clone(), OrderSide::Sell);
}

#[test]
fn equal_sides_hash_to_the_same_bucket() {
    let mut sides = HashSet::new();
    sides.insert(OrderSide::Buy);
    sides.insert(OrderSide::Buy);
    sides.insert(OrderSide::Sell);
    assert_eq!(sides.len(), 2);
}
