use std::collections::HashSet;

use lull_spec::enums::OrderType;

#[test]
fn market_is_not_limit() {
    assert_ne!(OrderType::Market, OrderType::Limit);
}

#[test]
fn clone_preserves_equality() {
    assert_eq!(OrderType::Market.clone(), OrderType::Market);
    assert_eq!(OrderType::Limit.clone(), OrderType::Limit);
}

#[test]
fn equal_order_types_hash_to_the_same_bucket() {
    let mut types = HashSet::new();
    types.insert(OrderType::Market);
    types.insert(OrderType::Market);
    types.insert(OrderType::Limit);
    assert_eq!(types.len(), 2);
}
