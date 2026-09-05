#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyCode<CC>(CC);

impl<CC> CurrencyCode<CC> {
    pub fn new(inner: CC) -> Self {
        Self(inner)
    }
}
