use std::collections::HashSet;

use lull_spec::enums::CurrencyRef;
use lull_spec::types::{CurrencyCode, CurrencyId};

type TestCurrencyRef = CurrencyRef<String, [u8; 3]>;

#[test]
fn equal_ids_are_equal() {
    let left: TestCurrencyRef = CurrencyRef::Id(CurrencyId::new(String::from("cur-1")));
    let right: TestCurrencyRef = CurrencyRef::Id(CurrencyId::new(String::from("cur-1")));
    assert_eq!(left, right);
}

#[test]
fn distinct_ids_are_not_equal() {
    let left: TestCurrencyRef = CurrencyRef::Id(CurrencyId::new(String::from("cur-1")));
    let right: TestCurrencyRef = CurrencyRef::Id(CurrencyId::new(String::from("cur-2")));
    assert_ne!(left, right);
}

#[test]
fn equal_codes_are_equal() {
    let left: TestCurrencyRef = CurrencyRef::Code(CurrencyCode::new(*b"USD"));
    let right: TestCurrencyRef = CurrencyRef::Code(CurrencyCode::new(*b"USD"));
    assert_eq!(left, right);
}

#[test]
fn distinct_codes_are_not_equal() {
    let left: TestCurrencyRef = CurrencyRef::Code(CurrencyCode::new(*b"USD"));
    let right: TestCurrencyRef = CurrencyRef::Code(CurrencyCode::new(*b"EUR"));
    assert_ne!(left, right);
}

#[test]
fn id_is_not_code() {
    let id: TestCurrencyRef = CurrencyRef::Id(CurrencyId::new(String::from("USD")));
    let code: TestCurrencyRef = CurrencyRef::Code(CurrencyCode::new(*b"USD"));
    assert_ne!(id, code);
}

#[test]
fn clone_preserves_equality() {
    let id: TestCurrencyRef = CurrencyRef::Id(CurrencyId::new(String::from("cur-1")));
    let code: TestCurrencyRef = CurrencyRef::Code(CurrencyCode::new(*b"USD"));
    assert_eq!(id.clone(), id);
    assert_eq!(code.clone(), code);
}

#[test]
fn equal_refs_hash_to_the_same_bucket() {
    let mut refs = HashSet::new();
    refs.insert(CurrencyRef::Id(CurrencyId::new(String::from("cur-1"))));
    refs.insert(CurrencyRef::Id(CurrencyId::new(String::from("cur-1"))));
    refs.insert(CurrencyRef::Code(CurrencyCode::new(*b"USD")));
    refs.insert(CurrencyRef::Code(CurrencyCode::new(*b"USD")));
    refs.insert(CurrencyRef::Code(CurrencyCode::new(*b"EUR")));
    assert_eq!(refs.len(), 3);
}
