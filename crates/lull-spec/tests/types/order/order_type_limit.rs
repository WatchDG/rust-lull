use std::collections::HashSet;

use lull_spec::enums::CurrencyRef;
use lull_spec::types::{CurrencyCode, Money, MoneyCurrency, MoneyValue, OrderTypeLimit};

type TestOrderTypeLimit = OrderTypeLimit<i64, CurrencyRef<String, [u8; 3]>>;

fn usd_limit(value: i64) -> TestOrderTypeLimit {
    OrderTypeLimit::new(Money::new(
        MoneyValue::new(value),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"USD"))),
    ))
}

#[test]
fn equal_prices_are_equal_limits() {
    assert_eq!(usd_limit(100), usd_limit(100));
}

#[test]
fn distinct_prices_are_not_equal_limits() {
    assert_ne!(usd_limit(100), usd_limit(101));
}

#[test]
fn clone_preserves_equality() {
    let limit = usd_limit(100);
    assert_eq!(limit.clone(), limit);
}

#[test]
fn equal_limits_hash_to_the_same_bucket() {
    let mut limits = HashSet::new();
    limits.insert(usd_limit(100));
    limits.insert(usd_limit(100));
    limits.insert(usd_limit(101));
    assert_eq!(limits.len(), 2);
}
