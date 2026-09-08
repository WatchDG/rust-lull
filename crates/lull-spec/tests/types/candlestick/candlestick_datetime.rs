use std::collections::HashSet;

use lull_spec::types::{CandlestickDateTime, DateTime};

type TestCandlestickDateTime = CandlestickDateTime<DateTime<i64>>;

#[test]
fn equal_datetimes_are_equal() {
    let left: TestCandlestickDateTime =
        CandlestickDateTime::new(DateTime::new(1_700_000_000_i64));
    let right: TestCandlestickDateTime =
        CandlestickDateTime::new(DateTime::new(1_700_000_000_i64));
    assert_eq!(left, right);
}

#[test]
fn distinct_datetimes_are_not_equal() {
    let left: TestCandlestickDateTime =
        CandlestickDateTime::new(DateTime::new(1_700_000_000_i64));
    let right: TestCandlestickDateTime =
        CandlestickDateTime::new(DateTime::new(1_700_000_300_i64));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let datetime: TestCandlestickDateTime =
        CandlestickDateTime::new(DateTime::new(1_700_000_000_i64));
    assert_eq!(datetime.clone(), datetime);
}

#[test]
fn equal_datetimes_hash_to_the_same_bucket() {
    let mut datetimes = HashSet::new();
    datetimes.insert(CandlestickDateTime::new(DateTime::new(1_700_000_000_i64)));
    datetimes.insert(CandlestickDateTime::new(DateTime::new(1_700_000_000_i64)));
    datetimes.insert(CandlestickDateTime::new(DateTime::new(1_700_000_300_i64)));
    assert_eq!(datetimes.len(), 2);
}
