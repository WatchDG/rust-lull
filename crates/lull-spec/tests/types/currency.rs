use std::collections::HashSet;

use lull_spec::types::{Currency, CurrencyCode, CurrencyId};

type TestCurrency = Currency<String, [u8; 3]>;

#[test]
fn equal_ids_are_equal() {
    let left: TestCurrency = Currency::Id(CurrencyId::new(String::from("cur-1")));
    let right: TestCurrency = Currency::Id(CurrencyId::new(String::from("cur-1")));
    assert_eq!(left, right);
}

#[test]
fn distinct_ids_are_not_equal() {
    let left: TestCurrency = Currency::Id(CurrencyId::new(String::from("cur-1")));
    let right: TestCurrency = Currency::Id(CurrencyId::new(String::from("cur-2")));
    assert_ne!(left, right);
}

#[test]
fn equal_codes_are_equal() {
    let left: TestCurrency = Currency::Code(CurrencyCode::new(*b"USD"));
    let right: TestCurrency = Currency::Code(CurrencyCode::new(*b"USD"));
    assert_eq!(left, right);
}

#[test]
fn distinct_codes_are_not_equal() {
    let left: TestCurrency = Currency::Code(CurrencyCode::new(*b"USD"));
    let right: TestCurrency = Currency::Code(CurrencyCode::new(*b"EUR"));
    assert_ne!(left, right);
}

#[test]
fn id_is_not_code() {
    let id: TestCurrency = Currency::Id(CurrencyId::new(String::from("USD")));
    let code: TestCurrency = Currency::Code(CurrencyCode::new(*b"USD"));
    assert_ne!(id, code);
}

#[test]
fn clone_preserves_equality() {
    let id: TestCurrency = Currency::Id(CurrencyId::new(String::from("cur-1")));
    let code: TestCurrency = Currency::Code(CurrencyCode::new(*b"USD"));
    assert_eq!(id.clone(), id);
    assert_eq!(code.clone(), code);
}

#[test]
fn equal_currencies_hash_to_the_same_bucket() {
    let mut currencies = HashSet::new();
    currencies.insert(Currency::Id(CurrencyId::new(String::from("cur-1"))));
    currencies.insert(Currency::Id(CurrencyId::new(String::from("cur-1"))));
    currencies.insert(Currency::Code(CurrencyCode::new(*b"USD")));
    currencies.insert(Currency::Code(CurrencyCode::new(*b"USD")));
    currencies.insert(Currency::Code(CurrencyCode::new(*b"EUR")));
    assert_eq!(currencies.len(), 3);
}
