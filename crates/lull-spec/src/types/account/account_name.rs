#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct AccountName<AN>(AN);

impl<AN> AccountName<AN> {
    pub fn new(inner: AN) -> Self {
        Self(inner)
    }
}
