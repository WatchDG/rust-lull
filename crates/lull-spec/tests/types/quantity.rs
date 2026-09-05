use std::collections::HashSet;

use lull_spec::types::Quantity;

#[test]
fn equal_values_are_equal() {
    let left = Quantity::new(100_i64);
    let right = Quantity::new(100_i64);
    assert_eq!(left, right);
}

#[test]
fn distinct_values_are_not_equal() {
    let left = Quantity::new(100_i64);
    let right = Quantity::new(101_i64);
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let value = Quantity::new(100_i64);
    assert_eq!(value.clone(), value);
}

#[test]
fn equal_values_hash_to_the_same_bucket() {
    let mut values = HashSet::new();
    values.insert(Quantity::new(100_i64));
    values.insert(Quantity::new(100_i64));
    values.insert(Quantity::new(101_i64));
    assert_eq!(values.len(), 2);
}
