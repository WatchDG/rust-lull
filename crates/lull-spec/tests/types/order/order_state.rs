use std::collections::HashSet;

use lull_spec::types::OrderState;

#[test]
fn equal_payloads_are_equal_states() {
    let left = OrderState::new(String::from("new"));
    let right = OrderState::new(String::from("new"));
    assert_eq!(left, right);
}

#[test]
fn distinct_payloads_are_not_equal_states() {
    let left = OrderState::new(String::from("new"));
    let right = OrderState::new(String::from("pending"));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let state = OrderState::new(String::from("new"));
    assert_eq!(state.clone(), state);
}

#[test]
fn equal_states_hash_to_the_same_bucket() {
    let mut states = HashSet::new();
    states.insert(OrderState::new(String::from("new")));
    states.insert(OrderState::new(String::from("new")));
    states.insert(OrderState::new(String::from("pending")));
    assert_eq!(states.len(), 2);
}
