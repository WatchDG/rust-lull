use lull_spec::traits::RiskEngine;

struct StubRisk;

impl RiskEngine for StubRisk {}

#[test]
fn stub_implements_risk_engine() {
    fn assert_engine<E: RiskEngine>(_: &E) {}
    assert_engine(&StubRisk);
}
