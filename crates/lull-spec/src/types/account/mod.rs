mod account_id;
mod account_name;

pub use account_id::AccountId;
pub use account_name::AccountName;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Account<AID, AN> {
    pub id: AccountId<AID>,
    pub name: Option<AccountName<AN>>,
}

impl<AID, AN> Account<AID, AN> {
    pub fn new(id: AccountId<AID>, name: Option<AccountName<AN>>) -> Self {
        Self { id, name }
    }
}
