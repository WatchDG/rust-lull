use std::collections::HashSet;

use lull_spec::types::MoneyValue;

#[test]
fn equal_values_are_equal() {
    let left = MoneyValue::new(100_i64);
    let right = MoneyValue::new(100_i64);
    assert_eq!(left, right);
}

#[test]
fn distinct_values_are_not_equal() {
    let left = MoneyValue::new(100_i64);
    let right = MoneyValue::new(101_i64);
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let value = MoneyValue::new(100_i64);
    assert_eq!(value.clone(), value);
}

#[test]
fn equal_values_hash_to_the_same_bucket() {
    let mut values = HashSet::new();
    values.insert(MoneyValue::new(100_i64));
    values.insert(MoneyValue::new(100_i64));
    values.insert(MoneyValue::new(101_i64));
    assert_eq!(values.len(), 2);
}
