use std::collections::HashSet;

use lull_spec::types::OrderTypeMarket;

#[test]
fn empty_markets_are_equal() {
    assert_eq!(OrderTypeMarket::new(), OrderTypeMarket::new());
}

#[test]
fn clone_preserves_equality() {
    let market = OrderTypeMarket::new();
    assert_eq!(market.clone(), market);
}

#[test]
fn equal_markets_hash_to_the_same_bucket() {
    let mut markets = HashSet::new();
    markets.insert(OrderTypeMarket::new());
    markets.insert(OrderTypeMarket::new());
    assert_eq!(markets.len(), 1);
}
