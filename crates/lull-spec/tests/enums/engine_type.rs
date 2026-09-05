use std::collections::HashSet;

use lull_spec::enums::EngineType;

#[test]
fn strategy_is_not_execution() {
    assert_ne!(EngineType::Strategy, EngineType::Execution);
}

#[test]
fn clone_preserves_equality() {
    assert_eq!(EngineType::Strategy.clone(), EngineType::Strategy);
    assert_eq!(EngineType::Risk.clone(), EngineType::Risk);
}

#[test]
fn equal_engine_types_hash_to_the_same_bucket() {
    let mut types = HashSet::new();
    types.insert(EngineType::Strategy);
    types.insert(EngineType::Strategy);
    types.insert(EngineType::Execution);
    types.insert(EngineType::Risk);
    types.insert(EngineType::Accounting);
    types.insert(EngineType::Compliance);
    assert_eq!(types.len(), 5);
}
