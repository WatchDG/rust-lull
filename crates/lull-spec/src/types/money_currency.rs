#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MoneyCurrency<MC>(MC);

impl<MC> MoneyCurrency<MC> {
    pub fn new(inner: MC) -> Self {
        Self(inner)
    }
}
