use std::collections::HashMap;

use lull_core::{CoreDAGBuilder, Edge, ExecuteError, FnFactory, Node, NodeHandler, NodeId};
use lull_spec::enums::{EngineType, TradeSignal};

struct PassThrough;

impl NodeHandler<TradeSignal, ()> for PassThrough {
    fn process(&mut self, inputs: &[TradeSignal]) -> Result<Vec<TradeSignal>, ()> {
        Ok(inputs.to_vec())
    }
}

struct Emit(TradeSignal);

impl NodeHandler<TradeSignal, ()> for Emit {
    fn process(&mut self, _inputs: &[TradeSignal]) -> Result<Vec<TradeSignal>, ()> {
        Ok(vec![self.0.clone()])
    }
}

fn node(
    id: &'static str,
    r#type: EngineType,
    implementation: &'static str,
) -> Node<&'static str, &'static str, ()> {
    Node::new(NodeId::new(id), r#type, implementation, ())
}

fn emit_buy(_: &()) -> Result<Emit, ()> {
    Ok(Emit(TradeSignal::Buy))
}

fn emit_sell(_: &()) -> Result<Emit, ()> {
    Ok(Emit(TradeSignal::Sell))
}

fn pass_through(_: &()) -> Result<PassThrough, ()> {
    Ok(PassThrough)
}

fn builder() -> CoreDAGBuilder<&'static str, &'static str, (), TradeSignal, ()> {
    CoreDAGBuilder::new()
        .register(EngineType::Strategy, "momentum", Box::new(FnFactory(emit_buy)))
        .register(
            EngineType::Strategy,
            "mean_reversion",
            Box::new(FnFactory(emit_sell)),
        )
        .register(EngineType::Risk, "var_limit", Box::new(FnFactory(pass_through)))
        .register(
            EngineType::Execution,
            "binance_adapter",
            Box::new(FnFactory(pass_through)),
        )
}

#[test]
fn executes_strategy_risk_execution_dag() {
    let dag = builder()
        .node(node("strat_momentum", EngineType::Strategy, "momentum"))
        .node(node(
            "strat_meanrev",
            EngineType::Strategy,
            "mean_reversion",
        ))
        .node(node("risk", EngineType::Risk, "var_limit"))
        .node(node(
            "exec_binance",
            EngineType::Execution,
            "binance_adapter",
        ))
        .edge(Edge::new(
            NodeId::new("strat_momentum"),
            NodeId::new("risk"),
        ))
        .edge(Edge::new(NodeId::new("strat_meanrev"), NodeId::new("risk")))
        .edge(Edge::new(NodeId::new("risk"), NodeId::new("exec_binance")))
        .build()
        .unwrap();

    let outputs = dag.execute(&HashMap::new()).unwrap();
    let exec = &outputs[&NodeId::new("exec_binance")];
    assert!(exec.contains(&TradeSignal::Buy));
    assert!(exec.contains(&TradeSignal::Sell));
    assert_eq!(exec.len(), 2);
}

#[test]
fn unknown_implementation_is_a_registry_miss() {
    let dag = builder()
        .node(node("only", EngineType::Strategy, "missing_impl"))
        .build()
        .unwrap();
    let error = dag.execute(&HashMap::new()).unwrap_err();
    assert_eq!(
        error,
        ExecuteError::UnknownFactory {
            r#type: EngineType::Strategy,
            implementation: "missing_impl",
        }
    );
}

#[test]
fn source_node_receives_seed_inputs() {
    let dag = builder()
        .node(node("risk", EngineType::Risk, "var_limit"))
        .build()
        .unwrap();
    let mut seeds = HashMap::new();
    seeds.insert(NodeId::new("risk"), vec![TradeSignal::Hold]);
    let outputs = dag.execute(&seeds).unwrap();
    assert_eq!(outputs[&NodeId::new("risk")], vec![TradeSignal::Hold]);
}
