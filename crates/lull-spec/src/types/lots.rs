#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lots<LTS>(pub LTS);

impl<LTS> Lots<LTS> {
    pub fn new(inner: LTS) -> Self {
        Self(inner)
    }
}
