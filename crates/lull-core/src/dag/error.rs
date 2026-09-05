use super::node::NodeId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GraphError<NID> {
    DuplicateNode(NodeId<NID>),
    UnknownNode(NodeId<NID>),
    Cycle,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecuteError<NID, ROLE, IMPL, E> {
    Graph(GraphError<NID>),
    UnknownFactory { r#type: ROLE, implementation: IMPL },
    Node { id: NodeId<NID>, source: E },
}

impl<NID, ROLE, IMPL, E> From<GraphError<NID>> for ExecuteError<NID, ROLE, IMPL, E> {
    fn from(error: GraphError<NID>) -> Self {
        Self::Graph(error)
    }
}
