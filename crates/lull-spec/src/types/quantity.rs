#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Quantity<QTY>(pub QTY);

impl<QTY> Quantity<QTY> {
    pub fn new(inner: QTY) -> Self {
        Self(inner)
    }
}
