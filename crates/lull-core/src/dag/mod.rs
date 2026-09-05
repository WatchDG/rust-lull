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

use lull_spec::enums::EngineType;

use executor::execute;

pub struct CoreDAGBuilder<NID, IMPL, NP, M, E> {
    nodes: Vec<Node<NID, IMPL, NP>>,
    edges: Vec<Edge<NID>>,
    registry: Registry<IMPL, M, NP, E>,
}

impl<NID, IMPL, NP, M, E> CoreDAGBuilder<NID, IMPL, NP, M, E>
where
    NID: Clone + Eq + Hash,
    IMPL: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            registry: Registry::new(),
        }
    }

    pub fn node(mut self, node: Node<NID, IMPL, NP>) -> Self {
        self.nodes.push(node);
        self
    }

    pub fn edge(mut self, edge: Edge<NID>) -> Self {
        self.edges.push(edge);
        self
    }

    pub fn register(
        mut self,
        r#type: EngineType,
        implementation: IMPL,
        factory: BoxedFactory<M, NP, E>,
    ) -> Self {
        self.registry.register(r#type, implementation, factory);
        self
    }

    pub fn build(self) -> Result<CoreDAG<NID, IMPL, NP, M, E>, GraphError<NID>> {
        let graph = Graph::new(self.nodes, self.edges)?;
        let order = graph.topological_order()?;
        Ok(CoreDAG {
            graph,
            registry: self.registry,
            order,
        })
    }
}

impl<NID, IMPL, NP, M, E> Default for CoreDAGBuilder<NID, IMPL, NP, M, E>
where
    NID: Clone + Eq + Hash,
    IMPL: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct CoreDAG<NID, IMPL, NP, M, E> {
    graph: Graph<NID, IMPL, NP>,
    registry: Registry<IMPL, M, NP, E>,
    order: Vec<NodeId<NID>>,
}

impl<NID, IMPL, NP, M, E> CoreDAG<NID, IMPL, NP, M, E>
where
    NID: Clone + Eq + Hash,
    IMPL: Clone + Eq + Hash,
    M: Clone,
{
    pub fn execute(
        &self,
        seeds: &NodeOutputs<NID, M>,
    ) -> Result<NodeOutputs<NID, M>, ExecuteError<NID, IMPL, E>> {
        execute(&self.graph, &self.registry, seeds, &self.order)
    }

    pub fn graph(&self) -> &Graph<NID, IMPL, NP> {
        &self.graph
    }
}
