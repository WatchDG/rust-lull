use std::collections::HashSet;

use lull_spec::types::OrderId;

#[test]
fn equal_strings_are_equal_ids() {
    let left = OrderId::new(String::from("ord-1"));
    let right = OrderId::new(String::from("ord-1"));
    assert_eq!(left, right);
}

#[test]
fn distinct_strings_are_not_equal_ids() {
    let left = OrderId::new(String::from("ord-1"));
    let right = OrderId::new(String::from("ord-2"));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let id = OrderId::new(String::from("ord-1"));
    assert_eq!(id.clone(), id);
}

#[test]
fn equal_ids_hash_to_the_same_bucket() {
    let mut ids = HashSet::new();
    ids.insert(OrderId::new(String::from("ord-1")));
    ids.insert(OrderId::new(String::from("ord-1")));
    ids.insert(OrderId::new(String::from("ord-2")));
    assert_eq!(ids.len(), 2);
}
