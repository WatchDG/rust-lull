use std::collections::HashSet;

use lull_spec::types::AccountName;

#[test]
fn equal_strings_are_equal_names() {
    let left = AccountName::new(String::from("main"));
    let right = AccountName::new(String::from("main"));
    assert_eq!(left, right);
}

#[test]
fn distinct_strings_are_not_equal_names() {
    let left = AccountName::new(String::from("main"));
    let right = AccountName::new(String::from("margin"));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let name = AccountName::new(String::from("main"));
    assert_eq!(name.clone(), name);
}

#[test]
fn equal_names_hash_to_the_same_bucket() {
    let mut names = HashSet::new();
    names.insert(AccountName::new(String::from("main")));
    names.insert(AccountName::new(String::from("main")));
    names.insert(AccountName::new(String::from("margin")));
    assert_eq!(names.len(), 2);
}
