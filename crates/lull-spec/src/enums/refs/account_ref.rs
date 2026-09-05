use crate::types::AccountId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AccountRef<AID> {
    Id(AccountId<AID>),
}
