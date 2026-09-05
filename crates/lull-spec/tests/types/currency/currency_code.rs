use std::collections::HashSet;

use lull_spec::types::CurrencyCode;

#[test]
fn equal_codes_are_equal() {
    let left = CurrencyCode::new(*b"USD");
    let right = CurrencyCode::new(*b"USD");
    assert_eq!(left, right);
}

#[test]
fn distinct_codes_are_not_equal() {
    let left = CurrencyCode::new(*b"USD");
    let right = CurrencyCode::new(*b"EUR");
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let code = CurrencyCode::new(*b"USD");
    assert_eq!(code.clone(), code);
}

#[test]
fn equal_codes_hash_to_the_same_bucket() {
    let mut codes = HashSet::new();
    codes.insert(CurrencyCode::new(*b"USD"));
    codes.insert(CurrencyCode::new(*b"USD"));
    codes.insert(CurrencyCode::new(*b"EUR"));
    assert_eq!(codes.len(), 2);
}
