#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct OrderTypeMarket;

impl OrderTypeMarket {
    pub fn new() -> Self {
        Self
    }
}
