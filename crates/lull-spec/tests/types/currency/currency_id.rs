use std::collections::HashSet;

use lull_spec::types::CurrencyId;

#[test]
fn equal_strings_are_equal_ids() {
    let left = CurrencyId::new(String::from("cur-1"));
    let right = CurrencyId::new(String::from("cur-1"));
    assert_eq!(left, right);
}

#[test]
fn distinct_strings_are_not_equal_ids() {
    let left = CurrencyId::new(String::from("cur-1"));
    let right = CurrencyId::new(String::from("cur-2"));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let id = CurrencyId::new(String::from("cur-1"));
    assert_eq!(id.clone(), id);
}

#[test]
fn equal_ids_hash_to_the_same_bucket() {
    let mut ids = HashSet::new();
    ids.insert(CurrencyId::new(String::from("cur-1")));
    ids.insert(CurrencyId::new(String::from("cur-1")));
    ids.insert(CurrencyId::new(String::from("cur-2")));
    assert_eq!(ids.len(), 2);
}
