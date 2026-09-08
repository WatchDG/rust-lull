use std::collections::HashSet;

use lull_spec::types::OrderStatus;

#[test]
fn equal_payloads_are_equal_statuses() {
    let left = OrderStatus::new(String::from("new"));
    let right = OrderStatus::new(String::from("new"));
    assert_eq!(left, right);
}

#[test]
fn distinct_payloads_are_not_equal_statuses() {
    let left = OrderStatus::new(String::from("new"));
    let right = OrderStatus::new(String::from("fill"));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let status = OrderStatus::new(String::from("new"));
    assert_eq!(status.clone(), status);
}

#[test]
fn equal_statuses_hash_to_the_same_bucket() {
    let mut statuses = HashSet::new();
    statuses.insert(OrderStatus::new(String::from("new")));
    statuses.insert(OrderStatus::new(String::from("new")));
    statuses.insert(OrderStatus::new(String::from("fill")));
    assert_eq!(statuses.len(), 2);
}
