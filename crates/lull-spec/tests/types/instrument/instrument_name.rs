use std::collections::HashSet;

use lull_spec::types::InstrumentName;

#[test]
fn equal_strings_are_equal_names() {
    let left = InstrumentName::new(String::from("SBER"));
    let right = InstrumentName::new(String::from("SBER"));
    assert_eq!(left, right);
}

#[test]
fn distinct_strings_are_not_equal_names() {
    let left = InstrumentName::new(String::from("SBER"));
    let right = InstrumentName::new(String::from("GAZP"));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let name = InstrumentName::new(String::from("SBER"));
    assert_eq!(name.clone(), name);
}

#[test]
fn equal_names_hash_to_the_same_bucket() {
    let mut names = HashSet::new();
    names.insert(InstrumentName::new(String::from("SBER")));
    names.insert(InstrumentName::new(String::from("SBER")));
    names.insert(InstrumentName::new(String::from("GAZP")));
    assert_eq!(names.len(), 2);
}
