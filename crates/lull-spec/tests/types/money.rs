use std::collections::HashSet;

use lull_spec::enums::CurrencyRef;
use lull_spec::types::{CurrencyCode, CurrencyId, Money, MoneyCurrency, MoneyValue};

type TestMoney = Money<i64, CurrencyRef<String, [u8; 3]>>;

fn usd(value: i64) -> TestMoney {
    Money::new(
        MoneyValue::new(value),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"USD"))),
    )
}

#[test]
fn equal_amounts_are_equal() {
    assert_eq!(usd(100), usd(100));
}

#[test]
fn distinct_values_are_not_equal() {
    assert_ne!(usd(100), usd(101));
}

#[test]
fn distinct_currencies_are_not_equal() {
    let usd_money = usd(100);
    let eur_money = Money::new(
        MoneyValue::new(100_i64),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"EUR"))),
    );
    assert_ne!(usd_money, eur_money);
}

#[test]
fn id_currency_is_not_code_currency() {
    let by_id = Money::new(
        MoneyValue::new(100_i64),
        MoneyCurrency::new(CurrencyRef::Id(CurrencyId::new(String::from("USD")))),
    );
    assert_ne!(by_id, usd(100));
}

#[test]
fn clone_preserves_equality() {
    let money = usd(100);
    assert_eq!(money.clone(), money);
}

#[test]
fn equal_amounts_hash_to_the_same_bucket() {
    let mut amounts = HashSet::new();
    amounts.insert(usd(100));
    amounts.insert(usd(100));
    amounts.insert(usd(101));
    amounts.insert(Money::new(
        MoneyValue::new(100_i64),
        MoneyCurrency::new(CurrencyRef::Code(CurrencyCode::new(*b"EUR"))),
    ));
    assert_eq!(amounts.len(), 3);
}
