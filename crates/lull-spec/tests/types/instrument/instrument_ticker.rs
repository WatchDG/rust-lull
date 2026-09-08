use std::collections::HashSet;

use lull_spec::types::InstrumentTicker;

#[test]
fn equal_strings_are_equal_tickers() {
    let left = InstrumentTicker::new(String::from("SBER"));
    let right = InstrumentTicker::new(String::from("SBER"));
    assert_eq!(left, right);
}

#[test]
fn distinct_strings_are_not_equal_tickers() {
    let left = InstrumentTicker::new(String::from("SBER"));
    let right = InstrumentTicker::new(String::from("GAZP"));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let ticker = InstrumentTicker::new(String::from("SBER"));
    assert_eq!(ticker.clone(), ticker);
}

#[test]
fn equal_tickers_hash_to_the_same_bucket() {
    let mut tickers = HashSet::new();
    tickers.insert(InstrumentTicker::new(String::from("SBER")));
    tickers.insert(InstrumentTicker::new(String::from("SBER")));
    tickers.insert(InstrumentTicker::new(String::from("GAZP")));
    assert_eq!(tickers.len(), 2);
}
