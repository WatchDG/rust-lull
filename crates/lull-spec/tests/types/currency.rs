use std::collections::HashSet;

use lull_spec::types::{Currency, CurrencyCode, CurrencyId};

type TestCurrency = Currency<String, [u8; 3]>;

fn currency(id: &str, code: [u8; 3]) -> TestCurrency {
    Currency::new(CurrencyId::new(String::from(id)), CurrencyCode::new(code))
}

#[test]
fn equal_fields_are_equal() {
    assert_eq!(currency("cur-1", *b"USD"), currency("cur-1", *b"USD"));
}

#[test]
fn distinct_ids_are_not_equal() {
    assert_ne!(currency("cur-1", *b"USD"), currency("cur-2", *b"USD"));
}

#[test]
fn distinct_codes_are_not_equal() {
    assert_ne!(currency("cur-1", *b"USD"), currency("cur-1", *b"EUR"));
}

#[test]
fn clone_preserves_equality() {
    let currency = currency("cur-1", *b"USD");
    assert_eq!(currency.clone(), currency);
}

#[test]
fn equal_currencies_hash_to_the_same_bucket() {
    let mut currencies = HashSet::new();
    currencies.insert(currency("cur-1", *b"USD"));
    currencies.insert(currency("cur-1", *b"USD"));
    currencies.insert(currency("cur-2", *b"USD"));
    currencies.insert(currency("cur-1", *b"EUR"));
    assert_eq!(currencies.len(), 3);
}
