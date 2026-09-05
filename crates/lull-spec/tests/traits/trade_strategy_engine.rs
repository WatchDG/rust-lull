use lull_spec::traits::TradeStrategyEngine;

struct StubStrategy;

impl TradeStrategyEngine for StubStrategy {}

#[test]
fn stub_implements_trade_strategy_engine() {
    fn assert_engine<E: TradeStrategyEngine>(_: &E) {}
    assert_engine(&StubStrategy);
}
