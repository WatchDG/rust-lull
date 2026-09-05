#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountId<AID>(AID);

impl<AID> AccountId<AID> {
    pub fn new(inner: AID) -> Self {
        Self(inner)
    }
}
