use lull_spec::traits::AccountingEngine;

struct StubAccounting;

impl AccountingEngine for StubAccounting {}

#[test]
fn stub_implements_accounting_engine() {
    fn assert_engine<E: AccountingEngine>(_: &E) {}
    assert_engine(&StubAccounting);
}
