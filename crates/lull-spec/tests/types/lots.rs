use std::collections::HashSet;

use lull_spec::types::Lots;

#[test]
fn equal_values_are_equal() {
    let left = Lots::new(10_i64);
    let right = Lots::new(10_i64);
    assert_eq!(left, right);
}

#[test]
fn distinct_values_are_not_equal() {
    let left = Lots::new(10_i64);
    let right = Lots::new(11_i64);
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let value = Lots::new(10_i64);
    assert_eq!(value.clone(), value);
}

#[test]
fn equal_values_hash_to_the_same_bucket() {
    let mut values = HashSet::new();
    values.insert(Lots::new(10_i64));
    values.insert(Lots::new(10_i64));
    values.insert(Lots::new(11_i64));
    assert_eq!(values.len(), 2);
}
