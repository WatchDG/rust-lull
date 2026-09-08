mod candlestick_id;
mod candlestick_interval;

use std::collections::HashSet;

use lull_spec::types::{Candlestick, CandlestickId, CandlestickInterval, DateTime};

type TestCandlestick = Candlestick<String, i32, i64>;

fn candlestick(id: &str, interval: i32, datetime: i64) -> TestCandlestick {
    Candlestick::new(
        CandlestickId::new(String::from(id)),
        CandlestickInterval::new(interval),
        DateTime::new(datetime),
    )
}

#[test]
fn equal_fields_are_equal_candlesticks() {
    assert_eq!(
        candlestick("cs-1", 300, 1_700_000_000),
        candlestick("cs-1", 300, 1_700_000_000)
    );
}

#[test]
fn distinct_ids_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick("cs-1", 300, 1_700_000_000),
        candlestick("cs-2", 300, 1_700_000_000)
    );
}

#[test]
fn distinct_intervals_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick("cs-1", 300, 1_700_000_000),
        candlestick("cs-1", 60, 1_700_000_000)
    );
}

#[test]
fn distinct_datetimes_are_not_equal_candlesticks() {
    assert_ne!(
        candlestick("cs-1", 300, 1_700_000_000),
        candlestick("cs-1", 300, 1_700_000_300)
    );
}

#[test]
fn clone_preserves_equality() {
    let candlestick = candlestick("cs-1", 300, 1_700_000_000);
    assert_eq!(candlestick.clone(), candlestick);
}

#[test]
fn equal_candlesticks_hash_to_the_same_bucket() {
    let mut candlesticks = HashSet::new();
    candlesticks.insert(candlestick("cs-1", 300, 1_700_000_000));
    candlesticks.insert(candlestick("cs-1", 300, 1_700_000_000));
    candlesticks.insert(candlestick("cs-2", 300, 1_700_000_000));
    candlesticks.insert(candlestick("cs-1", 60, 1_700_000_000));
    candlesticks.insert(candlestick("cs-1", 300, 1_700_000_300));
    assert_eq!(candlesticks.len(), 4);
}
