#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstrumentId<IID>(IID);

impl<IID> InstrumentId<IID> {
    pub fn new(inner: IID) -> Self {
        Self(inner)
    }
}
