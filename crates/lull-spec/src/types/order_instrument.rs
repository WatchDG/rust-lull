#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderInstrument<OI>(OI);

impl<OI> OrderInstrument<OI> {
    pub fn new(inner: OI) -> Self {
        Self(inner)
    }
}
