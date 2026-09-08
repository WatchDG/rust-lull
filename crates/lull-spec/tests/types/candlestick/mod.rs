mod candlestick_id;

use std::collections::HashSet;

use lull_spec::types::{Candlestick, CandlestickId};

type TestCandlestick = Candlestick<String>;

fn candlestick(id: &str) -> TestCandlestick {
    Candlestick::new(CandlestickId::new(String::from(id)))
}

#[test]
fn equal_ids_are_equal_candlesticks() {
    assert_eq!(candlestick("cs-1"), candlestick("cs-1"));
}

#[test]
fn distinct_ids_are_not_equal_candlesticks() {
    assert_ne!(candlestick("cs-1"), candlestick("cs-2"));
}

#[test]
fn clone_preserves_equality() {
    let candlestick = candlestick("cs-1");
    assert_eq!(candlestick.clone(), candlestick);
}

#[test]
fn equal_candlesticks_hash_to_the_same_bucket() {
    let mut candlesticks = HashSet::new();
    candlesticks.insert(candlestick("cs-1"));
    candlesticks.insert(candlestick("cs-1"));
    candlesticks.insert(candlestick("cs-2"));
    assert_eq!(candlesticks.len(), 2);
}
