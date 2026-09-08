use std::collections::HashSet;

use lull_spec::enums::CurrencyRef;
use lull_spec::types::{CandlestickPrice, CurrencyCode, Money, MoneyCurrency, MoneyValue};

type TestCandlestickPrice = CandlestickPrice<Money<i64, CurrencyRef<String, [u8; 3]>>>;

fn usd(value: i64) -> TestCandlestickPrice {
    CandlestickPrice::new(Money::new(
        MoneyValue::new(value),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"USD"))),
    ))
}

#[test]
fn equal_prices_are_equal() {
    assert_eq!(usd(100), usd(100));
}

#[test]
fn distinct_prices_are_not_equal() {
    assert_ne!(usd(100), usd(101));
}

#[test]
fn clone_preserves_equality() {
    let price = usd(100);
    assert_eq!(price.clone(), price);
}

#[test]
fn equal_prices_hash_to_the_same_bucket() {
    let mut prices = HashSet::new();
    prices.insert(usd(100));
    prices.insert(usd(100));
    prices.insert(usd(101));
    assert_eq!(prices.len(), 2);
}
