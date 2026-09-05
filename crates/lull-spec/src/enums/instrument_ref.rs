use crate::types::InstrumentId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InstrumentRef<IID> {
    Id(InstrumentId<IID>),
}
