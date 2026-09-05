use lull_core::{Edge, Graph, GraphError, Node, NodeId};

fn node(id: &str) -> Node<&str, &str, &str, ()> {
    Node::new(NodeId::new(id), "StrategyEngine", "identity", ())
}

#[test]
fn rejects_duplicate_node_ids() {
    let error = Graph::new(vec![node("a"), node("a")], Vec::new()).unwrap_err();
    assert_eq!(error, GraphError::DuplicateNode(NodeId::new("a")));
}

#[test]
fn rejects_edge_to_unknown_node() {
    let error = Graph::new(
        vec![node("a")],
        vec![Edge::new(NodeId::new("a"), NodeId::new("missing"))],
    )
    .unwrap_err();
    assert_eq!(error, GraphError::UnknownNode(NodeId::new("missing")));
}

#[test]
fn topological_order_puts_dependencies_first() {
    let graph = Graph::new(
        vec![node("a"), node("b"), node("c")],
        vec![
            Edge::new(NodeId::new("a"), NodeId::new("c")),
            Edge::new(NodeId::new("b"), NodeId::new("c")),
        ],
    )
    .unwrap();
    let order = graph.topological_order().unwrap();
    let rank = |id| order.iter().position(|n| n == &NodeId::new(id)).unwrap();
    assert!(rank("a") < rank("c"));
    assert!(rank("b") < rank("c"));
}

#[test]
fn topological_order_rejects_a_cycle() {
    let graph = Graph::new(
        vec![node("a"), node("b")],
        vec![
            Edge::new(NodeId::new("a"), NodeId::new("b")),
            Edge::new(NodeId::new("b"), NodeId::new("a")),
        ],
    )
    .unwrap();
    assert_eq!(graph.topological_order().unwrap_err(), GraphError::Cycle);
}
