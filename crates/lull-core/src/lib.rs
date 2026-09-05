//! DAG executor, engine registry, and other Lull cores.
//!
//! The DAG core owns nodes and edges. Engine contracts and domain messages
//! come from `lull-spec`. A node only sees inputs on its port; it does not know
//! its neighbors. Concrete engines are resolved by `(type, implementation)` keys.

pub mod dag;

pub use dag::{
    BoxedFactory, BoxedHandler, CoreDAG, CoreDAGBuilder, Edge, ExecuteError, FnFactory, Graph,
    GraphError, Node, NodeFactory, NodeHandler, NodeId, NodeOutputs, Registry,
};
