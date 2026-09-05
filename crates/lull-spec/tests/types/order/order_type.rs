use std::collections::HashSet;

use lull_spec::enums::{CurrencyRef, OrderTypeRef};
use lull_spec::types::{
    CurrencyCode, Money, MoneyCurrency, MoneyValue, OrderType, OrderTypeLimit, OrderTypeMarket,
};

type TestCurrency = CurrencyRef<String, [u8; 3]>;
type TestOrderTypeLimit = OrderTypeLimit<i64, TestCurrency>;
type TestOrderTypeRef = OrderTypeRef<i64, TestCurrency>;

fn usd_limit(value: i64) -> TestOrderTypeLimit {
    OrderTypeLimit::new(Money::new(
        MoneyValue::new(value),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"USD"))),
    ))
}

fn usd_limit_ref(value: i64) -> TestOrderTypeRef {
    OrderTypeRef::Limit(usd_limit(value))
}

#[test]
fn wraps_market() {
    let order_type: OrderType<OrderTypeMarket> = OrderType::new(OrderTypeMarket::new());
    assert_eq!(order_type, OrderType::new(OrderTypeMarket::new()));
}

#[test]
fn wraps_limit() {
    let order_type: OrderType<TestOrderTypeLimit> = OrderType::new(usd_limit(100));
    assert_eq!(order_type, OrderType::new(usd_limit(100)));
}

#[test]
fn wraps_ref() {
    let order_type: OrderType<TestOrderTypeRef> = OrderType::new(usd_limit_ref(100));
    assert_eq!(order_type, OrderType::new(usd_limit_ref(100)));
}

#[test]
fn equal_types_are_equal() {
    assert_eq!(
        OrderType::new(usd_limit_ref(100)),
        OrderType::new(usd_limit_ref(100))
    );
}

#[test]
fn distinct_types_are_not_equal() {
    assert_ne!(
        OrderType::new(usd_limit_ref(100)),
        OrderType::new(TestOrderTypeRef::Market)
    );
}

#[test]
fn clone_preserves_equality() {
    let order_type = OrderType::new(usd_limit_ref(100));
    assert_eq!(order_type.clone(), order_type);
}

#[test]
fn equal_types_hash_to_the_same_bucket() {
    let mut types = HashSet::new();
    types.insert(OrderType::new(usd_limit_ref(100)));
    types.insert(OrderType::new(usd_limit_ref(100)));
    types.insert(OrderType::new(TestOrderTypeRef::Market));
    assert_eq!(types.len(), 2);
}
