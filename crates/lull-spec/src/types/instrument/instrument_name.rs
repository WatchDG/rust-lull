#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstrumentName<IN>(IN);

impl<IN> InstrumentName<IN> {
    pub fn new(inner: IN) -> Self {
        Self(inner)
    }
}
