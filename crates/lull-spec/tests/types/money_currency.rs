use std::collections::HashSet;

use lull_spec::types::{Currency, CurrencyCode, CurrencyId, MoneyCurrency};

type TestMoneyCurrency = MoneyCurrency<Currency<String, [u8; 3]>>;

#[test]
fn equal_currencies_are_equal() {
    let left: TestMoneyCurrency =
        MoneyCurrency::new(Currency::Id(CurrencyId::new(String::from("cur-1"))));
    let right: TestMoneyCurrency =
        MoneyCurrency::new(Currency::Id(CurrencyId::new(String::from("cur-1"))));
    assert_eq!(left, right);
}

#[test]
fn distinct_currencies_are_not_equal() {
    let left: TestMoneyCurrency =
        MoneyCurrency::new(Currency::Id(CurrencyId::new(String::from("cur-1"))));
    let right: TestMoneyCurrency =
        MoneyCurrency::new(Currency::Id(CurrencyId::new(String::from("cur-2"))));
    assert_ne!(left, right);
}

#[test]
fn id_is_not_code() {
    let id: TestMoneyCurrency =
        MoneyCurrency::new(Currency::Id(CurrencyId::new(String::from("USD"))));
    let code: TestMoneyCurrency = MoneyCurrency::new(Currency::Code(CurrencyCode::new(*b"USD")));
    assert_ne!(id, code);
}

#[test]
fn clone_preserves_equality() {
    let currency: TestMoneyCurrency =
        MoneyCurrency::new(Currency::Code(CurrencyCode::new(*b"USD")));
    assert_eq!(currency.clone(), currency);
}

#[test]
fn equal_currencies_hash_to_the_same_bucket() {
    let mut currencies = HashSet::new();
    currencies.insert(MoneyCurrency::new(Currency::Id(CurrencyId::new(
        String::from("cur-1"),
    ))));
    currencies.insert(MoneyCurrency::new(Currency::Id(CurrencyId::new(
        String::from("cur-1"),
    ))));
    currencies.insert(MoneyCurrency::new(Currency::Code(CurrencyCode::new(
        *b"USD",
    ))));
    currencies.insert(MoneyCurrency::new(Currency::Code(CurrencyCode::new(
        *b"USD",
    ))));
    currencies.insert(MoneyCurrency::new(Currency::Code(CurrencyCode::new(
        *b"EUR",
    ))));
    assert_eq!(currencies.len(), 3);
}
