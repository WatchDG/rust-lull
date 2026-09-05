use std::collections::HashSet;

use lull_spec::enums::AccountRef;
use lull_spec::types::AccountId;

type TestAccountRef = AccountRef<String>;

#[test]
fn equal_ids_are_equal() {
    let left: TestAccountRef = AccountRef::Id(AccountId::new(String::from("acc-1")));
    let right: TestAccountRef = AccountRef::Id(AccountId::new(String::from("acc-1")));
    assert_eq!(left, right);
}

#[test]
fn distinct_ids_are_not_equal() {
    let left: TestAccountRef = AccountRef::Id(AccountId::new(String::from("acc-1")));
    let right: TestAccountRef = AccountRef::Id(AccountId::new(String::from("acc-2")));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let id: TestAccountRef = AccountRef::Id(AccountId::new(String::from("acc-1")));
    assert_eq!(id.clone(), id);
}

#[test]
fn equal_refs_hash_to_the_same_bucket() {
    let mut refs = HashSet::new();
    refs.insert(AccountRef::Id(AccountId::new(String::from("acc-1"))));
    refs.insert(AccountRef::Id(AccountId::new(String::from("acc-1"))));
    refs.insert(AccountRef::Id(AccountId::new(String::from("acc-2"))));
    assert_eq!(refs.len(), 2);
}
