use std::collections::HashSet;

use lull_spec::enums::OrderRef;
use lull_spec::types::OrderId;

type TestOrderRef = OrderRef<String>;

#[test]
fn equal_ids_are_equal() {
    let left: TestOrderRef = OrderRef::Id(OrderId::new(String::from("ord-1")));
    let right: TestOrderRef = OrderRef::Id(OrderId::new(String::from("ord-1")));
    assert_eq!(left, right);
}

#[test]
fn distinct_ids_are_not_equal() {
    let left: TestOrderRef = OrderRef::Id(OrderId::new(String::from("ord-1")));
    let right: TestOrderRef = OrderRef::Id(OrderId::new(String::from("ord-2")));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let id: TestOrderRef = OrderRef::Id(OrderId::new(String::from("ord-1")));
    assert_eq!(id.clone(), id);
}

#[test]
fn equal_refs_hash_to_the_same_bucket() {
    let mut refs = HashSet::new();
    refs.insert(OrderRef::Id(OrderId::new(String::from("ord-1"))));
    refs.insert(OrderRef::Id(OrderId::new(String::from("ord-1"))));
    refs.insert(OrderRef::Id(OrderId::new(String::from("ord-2"))));
    assert_eq!(refs.len(), 2);
}
