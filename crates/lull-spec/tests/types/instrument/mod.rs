mod instrument_id;
mod instrument_name;

use std::collections::HashSet;

use lull_spec::types::{Instrument, InstrumentId, InstrumentName, Quantity};

type TestInstrument = Instrument<String, String, i32>;

fn instrument(id: &str, name: &str, lot_size: i32) -> TestInstrument {
    Instrument::new(
        InstrumentId::new(String::from(id)),
        InstrumentName::new(String::from(name)),
        Quantity::new(lot_size),
    )
}

#[test]
fn equal_fields_are_equal_instruments() {
    assert_eq!(
        instrument("inst-1", "SBER", 1),
        instrument("inst-1", "SBER", 1)
    );
}

#[test]
fn distinct_ids_are_not_equal_instruments() {
    assert_ne!(
        instrument("inst-1", "SBER", 1),
        instrument("inst-2", "SBER", 1)
    );
}

#[test]
fn distinct_names_are_not_equal_instruments() {
    assert_ne!(
        instrument("inst-1", "SBER", 1),
        instrument("inst-1", "GAZP", 1)
    );
}

#[test]
fn distinct_lot_sizes_are_not_equal_instruments() {
    assert_ne!(
        instrument("inst-1", "SBER", 1),
        instrument("inst-1", "SBER", 10)
    );
}

#[test]
fn clone_preserves_equality() {
    let instrument = instrument("inst-1", "SBER", 1);
    assert_eq!(instrument.clone(), instrument);
}

#[test]
fn equal_instruments_hash_to_the_same_bucket() {
    let mut instruments = HashSet::new();
    instruments.insert(instrument("inst-1", "SBER", 1));
    instruments.insert(instrument("inst-1", "SBER", 1));
    instruments.insert(instrument("inst-2", "SBER", 1));
    instruments.insert(instrument("inst-1", "GAZP", 1));
    instruments.insert(instrument("inst-1", "SBER", 10));
    assert_eq!(instruments.len(), 4);
}
