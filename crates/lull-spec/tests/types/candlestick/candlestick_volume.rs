use std::collections::HashSet;

use lull_spec::types::{CandlestickVolume, Quantity};

type TestCandlestickVolume = CandlestickVolume<Quantity<i64>>;

#[test]
fn equal_volumes_are_equal() {
    let left: TestCandlestickVolume = CandlestickVolume::new(Quantity::new(1_000_i64));
    let right: TestCandlestickVolume = CandlestickVolume::new(Quantity::new(1_000_i64));
    assert_eq!(left, right);
}

#[test]
fn distinct_volumes_are_not_equal() {
    let left: TestCandlestickVolume = CandlestickVolume::new(Quantity::new(1_000_i64));
    let right: TestCandlestickVolume = CandlestickVolume::new(Quantity::new(2_000_i64));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let volume: TestCandlestickVolume = CandlestickVolume::new(Quantity::new(1_000_i64));
    assert_eq!(volume.clone(), volume);
}

#[test]
fn equal_volumes_hash_to_the_same_bucket() {
    let mut volumes = HashSet::new();
    volumes.insert(CandlestickVolume::new(Quantity::new(1_000_i64)));
    volumes.insert(CandlestickVolume::new(Quantity::new(1_000_i64)));
    volumes.insert(CandlestickVolume::new(Quantity::new(2_000_i64)));
    assert_eq!(volumes.len(), 2);
}
