#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PriceValue<PV>(PV);

impl<PV> PriceValue<PV> {
    pub fn new(inner: PV) -> Self {
        Self(inner)
    }
}
