use lull_spec::traits::StrategyEngine;

struct StubStrategy;

impl StrategyEngine for StubStrategy {}

#[test]
fn stub_implements_strategy_engine() {
    fn assert_engine<E: StrategyEngine>(_: &E) {}
    assert_engine(&StubStrategy);
}
