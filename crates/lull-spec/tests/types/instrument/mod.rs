mod instrument_id;

use std::collections::HashSet;

use lull_spec::types::{Instrument, InstrumentId};

type TestInstrument = Instrument<String>;

fn instrument(id: &str) -> TestInstrument {
    Instrument::new(InstrumentId::new(String::from(id)))
}

#[test]
fn equal_ids_are_equal_instruments() {
    assert_eq!(instrument("inst-1"), instrument("inst-1"));
}

#[test]
fn distinct_ids_are_not_equal_instruments() {
    assert_ne!(instrument("inst-1"), instrument("inst-2"));
}

#[test]
fn clone_preserves_equality() {
    let instrument = instrument("inst-1");
    assert_eq!(instrument.clone(), instrument);
}

#[test]
fn equal_instruments_hash_to_the_same_bucket() {
    let mut instruments = HashSet::new();
    instruments.insert(instrument("inst-1"));
    instruments.insert(instrument("inst-1"));
    instruments.insert(instrument("inst-2"));
    assert_eq!(instruments.len(), 2);
}
