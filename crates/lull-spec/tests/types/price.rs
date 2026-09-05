use std::collections::HashSet;

use lull_spec::types::{Currency, CurrencyCode, CurrencyId, Price, PriceCurrency, PriceValue};

type TestPrice = Price<i64, Currency<String, [u8; 3]>>;

fn usd(value: i64) -> TestPrice {
    Price::new(
        PriceValue::new(value),
        PriceCurrency::new(Currency::Code(CurrencyCode::new(*b"USD"))),
    )
}

#[test]
fn equal_prices_are_equal() {
    assert_eq!(usd(100), usd(100));
}

#[test]
fn distinct_values_are_not_equal() {
    assert_ne!(usd(100), usd(101));
}

#[test]
fn distinct_currencies_are_not_equal() {
    let usd_price = usd(100);
    let eur_price = Price::new(
        PriceValue::new(100_i64),
        PriceCurrency::new(Currency::Code(CurrencyCode::new(*b"EUR"))),
    );
    assert_ne!(usd_price, eur_price);
}

#[test]
fn id_currency_is_not_code_currency() {
    let by_id = Price::new(
        PriceValue::new(100_i64),
        PriceCurrency::new(Currency::Id(CurrencyId::new(String::from("USD")))),
    );
    assert_ne!(by_id, usd(100));
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
    prices.insert(Price::new(
        PriceValue::new(100_i64),
        PriceCurrency::new(Currency::Code(CurrencyCode::new(*b"EUR"))),
    ));
    assert_eq!(prices.len(), 3);
}
