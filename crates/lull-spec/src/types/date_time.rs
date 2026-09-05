#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DateTime<DT>(DT);

impl<DT> DateTime<DT> {
    pub fn new(inner: DT) -> Self {
        Self(inner)
    }
}
