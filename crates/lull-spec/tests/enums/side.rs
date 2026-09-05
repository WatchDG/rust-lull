use std::collections::HashSet;

use lull_spec::enums::Side;

#[test]
fn buy_is_not_sell() {
    assert_ne!(Side::Buy, Side::Sell);
}

#[test]
fn clone_preserves_equality() {
    assert_eq!(Side::Buy.clone(), Side::Buy);
    assert_eq!(Side::Sell.clone(), Side::Sell);
}

#[test]
fn equal_sides_hash_to_the_same_bucket() {
    let mut sides = HashSet::new();
    sides.insert(Side::Buy);
    sides.insert(Side::Buy);
    sides.insert(Side::Sell);
    assert_eq!(sides.len(), 2);
}
