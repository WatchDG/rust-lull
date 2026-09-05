use std::collections::HashSet;

use lull_spec::enums::InstrumentRef;
use lull_spec::types::{InstrumentId, OrderInstrument};

type TestOrderInstrument = OrderInstrument<InstrumentRef<String>>;

#[test]
fn equal_instruments_are_equal() {
    let left: TestOrderInstrument =
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from("inst-1"))));
    let right: TestOrderInstrument =
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from("inst-1"))));
    assert_eq!(left, right);
}

#[test]
fn distinct_instruments_are_not_equal() {
    let left: TestOrderInstrument =
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from("inst-1"))));
    let right: TestOrderInstrument =
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from("inst-2"))));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let instrument: TestOrderInstrument =
        OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(String::from("inst-1"))));
    assert_eq!(instrument.clone(), instrument);
}

#[test]
fn equal_instruments_hash_to_the_same_bucket() {
    let mut instruments = HashSet::new();
    instruments.insert(OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(
        String::from("inst-1"),
    ))));
    instruments.insert(OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(
        String::from("inst-1"),
    ))));
    instruments.insert(OrderInstrument::new(InstrumentRef::Id(InstrumentId::new(
        String::from("inst-2"),
    ))));
    assert_eq!(instruments.len(), 2);
}
