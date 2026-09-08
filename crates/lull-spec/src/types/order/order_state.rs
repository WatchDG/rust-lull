#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderState<OST>(OST);

impl<OST> OrderState<OST> {
    pub fn new(inner: OST) -> Self {
        Self(inner)
    }
}
