#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderSize<OS>(OS);

impl<OS> OrderSize<OS> {
    pub fn new(inner: OS) -> Self {
        Self(inner)
    }
}
