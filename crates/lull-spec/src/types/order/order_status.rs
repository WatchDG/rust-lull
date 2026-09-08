#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OrderStatus<OSS>(OSS);

impl<OSS> OrderStatus<OSS> {
    pub fn new(inner: OSS) -> Self {
        Self(inner)
    }
}
