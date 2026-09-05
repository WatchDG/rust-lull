use std::collections::HashSet;

use lull_spec::enums::{CurrencyRef, OrderTypeRef};
use lull_spec::types::{CurrencyCode, Money, MoneyCurrency, MoneyValue, OrderTypeLimit};

type TestOrderTypeRef = OrderTypeRef<i64, CurrencyRef<String, [u8; 3]>>;

fn usd_limit(value: i64) -> TestOrderTypeRef {
    OrderTypeRef::Limit(OrderTypeLimit::new(Money::new(
        MoneyValue::new(value),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"USD"))),
    )))
}

#[test]
fn market_is_not_limit() {
    assert_ne!(TestOrderTypeRef::Market, usd_limit(100));
}

#[test]
fn equal_limit_prices_are_equal() {
    assert_eq!(usd_limit(100), usd_limit(100));
}

#[test]
fn distinct_limit_prices_are_not_equal() {
    assert_ne!(usd_limit(100), usd_limit(101));
}

#[test]
fn clone_preserves_equality() {
    assert_eq!(TestOrderTypeRef::Market.clone(), TestOrderTypeRef::Market);
    assert_eq!(usd_limit(100).clone(), usd_limit(100));
}

#[test]
fn equal_order_types_hash_to_the_same_bucket() {
    let mut types = HashSet::new();
    types.insert(TestOrderTypeRef::Market);
    types.insert(TestOrderTypeRef::Market);
    types.insert(usd_limit(100));
    types.insert(usd_limit(100));
    types.insert(usd_limit(101));
    assert_eq!(types.len(), 3);
}
