mod edge;
mod error;
mod executor;
mod graph;
mod handler;
mod node;
mod registry;

pub use edge::Edge;
pub use error::{ExecuteError, GraphError};
pub use executor::NodeOutputs;
pub use graph::Graph;
pub use handler::{BoxedFactory, BoxedHandler, FnFactory, NodeFactory, NodeHandler};
pub use node::{Node, NodeId};
pub use registry::Registry;

use std::hash::Hash;

use executor::execute;

pub struct CoreDAGBuilder<NID, ROLE, IMPL, NP, M, E> {
    nodes: Vec<Node<NID, ROLE, IMPL, NP>>,
    edges: Vec<Edge<NID>>,
    registry: Registry<ROLE, IMPL, M, NP, E>,
}

impl<NID, ROLE, IMPL, NP, M, E> CoreDAGBuilder<NID, ROLE, IMPL, NP, M, E>
where
    NID: Clone + Eq + Hash,
    ROLE: Clone + Eq + Hash,
    IMPL: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            registry: Registry::new(),
        }
    }

    pub fn node(mut self, node: Node<NID, ROLE, IMPL, NP>) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn edge(mut self, edge: Edge<NID>) -> Self {
        self.edges.push(edge);
        self
    }

    pub fn register(
        mut self,
        r#type: ROLE,
        implementation: IMPL,
        factory: BoxedFactory<M, NP, E>,
    ) -> Self {
        self.registry.register(r#type, implementation, factory);
        self
    }

    #[allow(clippy::type_complexity)]
    pub fn build(self) -> Result<CoreDAG<NID, ROLE, IMPL, NP, M, E>, GraphError<NID>> {
        let graph = Graph::new(self.nodes, self.edges)?;
        let order = graph.topological_order()?;
        Ok(CoreDAG {
            graph,
            registry: self.registry,
            order,
        })
    }
}

impl<NID, ROLE, IMPL, NP, M, E> Default for CoreDAGBuilder<NID, ROLE, IMPL, NP, M, E>
where
    NID: Clone + Eq + Hash,
    ROLE: Clone + Eq + Hash,
    IMPL: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct CoreDAG<NID, ROLE, IMPL, NP, M, E> {
    graph: Graph<NID, ROLE, IMPL, NP>,
    registry: Registry<ROLE, IMPL, M, NP, E>,
    order: Vec<NodeId<NID>>,
}

impl<NID, ROLE, IMPL, NP, M, E> CoreDAG<NID, ROLE, IMPL, NP, M, E>
where
    NID: Clone + Eq + Hash,
    ROLE: Clone + Eq + Hash,
    IMPL: Clone + Eq + Hash,
    M: Clone,
{
    pub fn execute(
        &self,
        seeds: &NodeOutputs<NID, M>,
    ) -> Result<NodeOutputs<NID, M>, ExecuteError<NID, ROLE, IMPL, E>> {
        execute(&self.graph, &self.registry, seeds, &self.order)
    }

    pub fn graph(&self) -> &Graph<NID, ROLE, IMPL, NP> {
        &self.graph
    }
}
