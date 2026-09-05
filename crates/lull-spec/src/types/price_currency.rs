#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PriceCurrency<PC>(PC);

impl<PC> PriceCurrency<PC> {
    pub fn new(inner: PC) -> Self {
        Self(inner)
    }
}
