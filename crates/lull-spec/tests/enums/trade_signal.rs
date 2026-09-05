use std::collections::HashSet;

use lull_spec::enums::TradeSignal;

#[test]
fn buy_sell_and_hold_are_distinct() {
    assert_ne!(TradeSignal::Buy, TradeSignal::Sell);
    assert_ne!(TradeSignal::Buy, TradeSignal::Hold);
    assert_ne!(TradeSignal::Sell, TradeSignal::Hold);
}

#[test]
fn clone_preserves_equality() {
    assert_eq!(TradeSignal::Buy.clone(), TradeSignal::Buy);
    assert_eq!(TradeSignal::Sell.clone(), TradeSignal::Sell);
    assert_eq!(TradeSignal::Hold.clone(), TradeSignal::Hold);
}

#[test]
fn equal_signals_hash_to_the_same_bucket() {
    let mut signals = HashSet::new();
    signals.insert(TradeSignal::Buy);
    signals.insert(TradeSignal::Buy);
    signals.insert(TradeSignal::Sell);
    signals.insert(TradeSignal::Hold);
    assert_eq!(signals.len(), 3);
}
