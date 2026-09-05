use lull_spec::traits::TradeEngine;

struct StubEngine;

impl TradeEngine for StubEngine {}

#[test]
fn stub_implements_trade_engine() {
    fn assert_engine<E: TradeEngine>(_: &E) {}
    assert_engine(&StubEngine);
}
