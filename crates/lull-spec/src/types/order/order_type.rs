#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderType<OT>(OT);

impl<OT> OrderType<OT> {
    pub fn new(inner: OT) -> Self {
        Self(inner)
    }
}
