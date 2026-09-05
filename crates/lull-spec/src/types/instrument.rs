use super::instrument_id::InstrumentId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Instrument<IID> {
    pub id: InstrumentId<IID>,
}

impl<IID> Instrument<IID> {
    pub fn new(id: InstrumentId<IID>) -> Self {
        Self { id }
    }
}
