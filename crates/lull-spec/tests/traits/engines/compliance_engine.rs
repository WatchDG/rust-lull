use lull_spec::traits::ComplianceEngine;

struct StubCompliance;

impl ComplianceEngine for StubCompliance {}

#[test]
fn stub_implements_compliance_engine() {
    fn assert_engine<E: ComplianceEngine>(_: &E) {}
    assert_engine(&StubCompliance);
}
