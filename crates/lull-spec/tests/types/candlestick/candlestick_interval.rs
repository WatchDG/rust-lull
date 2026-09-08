use std::collections::HashSet;

use lull_spec::types::CandlestickInterval;

#[test]
fn equal_intervals_are_equal() {
    let left = CandlestickInterval::new(300_i32);
    let right = CandlestickInterval::new(300_i32);
    assert_eq!(left, right);
}

#[test]
fn distinct_intervals_are_not_equal() {
    let left = CandlestickInterval::new(300_i32);
    let right = CandlestickInterval::new(60_i32);
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let interval = CandlestickInterval::new(300_i32);
    assert_eq!(interval.clone(), interval);
}

#[test]
fn equal_intervals_hash_to_the_same_bucket() {
    let mut intervals = HashSet::new();
    intervals.insert(CandlestickInterval::new(300_i32));
    intervals.insert(CandlestickInterval::new(300_i32));
    intervals.insert(CandlestickInterval::new(60_i32));
    assert_eq!(intervals.len(), 2);
}
