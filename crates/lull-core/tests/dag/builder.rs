use lull_core::{CoreDAGBuilder, Edge, GraphError, Node, NodeId};
use lull_spec::enums::EngineType;

fn node(id: &'static str) -> Node<&'static str, &'static str, ()> {
    Node::new(NodeId::new(id), EngineType::Risk, "var_limit", ())
}

#[test]
fn builder_returns_core_dag() {
    let dag = CoreDAGBuilder::<_, _, _, (), ()>::new()
        .node(node("a"))
        .node(node("b"))
        .edge(Edge::new(NodeId::new("a"), NodeId::new("b")))
        .build()
        .unwrap();
    assert_eq!(dag.graph().nodes().len(), 2);
}

#[test]
fn builder_rejects_a_cycle() {
    let result = CoreDAGBuilder::<_, _, _, (), ()>::new()
        .node(node("a"))
        .node(node("b"))
        .edge(Edge::new(NodeId::new("a"), NodeId::new("b")))
        .edge(Edge::new(NodeId::new("b"), NodeId::new("a")))
        .build();
    assert!(matches!(result, Err(GraphError::Cycle)));
}
