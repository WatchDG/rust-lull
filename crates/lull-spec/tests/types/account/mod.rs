mod account_id;
mod account_name;

use std::collections::HashSet;

use lull_spec::types::{Account, AccountId, AccountName};

type TestAccount = Account<String, String>;

fn account(id: &str, name: Option<&str>) -> TestAccount {
    Account::new(
        AccountId::new(String::from(id)),
        name.map(|name| AccountName::new(String::from(name))),
    )
}

#[test]
fn equal_fields_are_equal_accounts() {
    assert_eq!(account("acc-1", None), account("acc-1", None));
    assert_eq!(account("acc-1", Some("main")), account("acc-1", Some("main")));
}

#[test]
fn distinct_ids_are_not_equal_accounts() {
    assert_ne!(account("acc-1", None), account("acc-2", None));
}

#[test]
fn present_name_is_not_absent_name() {
    assert_ne!(account("acc-1", Some("main")), account("acc-1", None));
}

#[test]
fn distinct_names_are_not_equal_accounts() {
    assert_ne!(
        account("acc-1", Some("main")),
        account("acc-1", Some("margin"))
    );
}

#[test]
fn clone_preserves_equality() {
    let account = account("acc-1", Some("main"));
    assert_eq!(account.clone(), account);
}

#[test]
fn equal_accounts_hash_to_the_same_bucket() {
    let mut accounts = HashSet::new();
    accounts.insert(account("acc-1", None));
    accounts.insert(account("acc-1", None));
    accounts.insert(account("acc-2", None));
    accounts.insert(account("acc-1", Some("main")));
    accounts.insert(account("acc-1", Some("margin")));
    assert_eq!(accounts.len(), 4);
}
