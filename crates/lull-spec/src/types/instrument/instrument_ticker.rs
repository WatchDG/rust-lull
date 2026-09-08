#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InstrumentTicker<IT>(IT);

impl<IT> InstrumentTicker<IT> {
    pub fn new(inner: IT) -> Self {
        Self(inner)
    }
}
