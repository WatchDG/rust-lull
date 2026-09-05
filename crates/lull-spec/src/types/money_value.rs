#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoneyValue<MV>(MV);

impl<MV> MoneyValue<MV> {
    pub fn new(inner: MV) -> Self {
        Self(inner)
    }
}
