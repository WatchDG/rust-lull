#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderId<OID>(OID);

impl<OID> OrderId<OID> {
    pub fn new(inner: OID) -> Self {
        Self(inner)
    }
}
