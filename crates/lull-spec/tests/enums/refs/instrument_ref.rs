use std::collections::HashSet;

use lull_spec::enums::InstrumentRef;
use lull_spec::types::InstrumentId;

type TestInstrumentRef = InstrumentRef<String>;

#[test]
fn equal_ids_are_equal() {
    let left: TestInstrumentRef = InstrumentRef::Id(InstrumentId::new(String::from("inst-1")));
    let right: TestInstrumentRef = InstrumentRef::Id(InstrumentId::new(String::from("inst-1")));
    assert_eq!(left, right);
}

#[test]
fn distinct_ids_are_not_equal() {
    let left: TestInstrumentRef = InstrumentRef::Id(InstrumentId::new(String::from("inst-1")));
    let right: TestInstrumentRef = InstrumentRef::Id(InstrumentId::new(String::from("inst-2")));
    assert_ne!(left, right);
}

#[test]
fn clone_preserves_equality() {
    let id: TestInstrumentRef = InstrumentRef::Id(InstrumentId::new(String::from("inst-1")));
    assert_eq!(id.clone(), id);
}

#[test]
fn equal_refs_hash_to_the_same_bucket() {
    let mut refs = HashSet::new();
    refs.insert(InstrumentRef::Id(InstrumentId::new(String::from("inst-1"))));
    refs.insert(InstrumentRef::Id(InstrumentId::new(String::from("inst-1"))));
    refs.insert(InstrumentRef::Id(InstrumentId::new(String::from("inst-2"))));
    assert_eq!(refs.len(), 2);
}
