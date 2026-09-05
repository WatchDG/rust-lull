//! Cores for Lull: DAG graph execution and linear pipelines.
//!
//! Engine contracts, `EngineType`, and domain messages come from `lull-spec`.
//! A node or stage only sees its port input; it does not know its neighbors.
//! Concrete engines are resolved from a registry by `EngineType` and implementation.

pub mod dag;
pub mod pipeline;

pub use lull_spec::enums::EngineType;
pub use dag::{
    BoxedFactory, BoxedHandler, CoreDAG, CoreDAGBuilder, Edge, ExecuteError, FnFactory, Graph,
    GraphError, Node, NodeFactory, NodeHandler, NodeId, NodeOutputs, Registry,
};
pub use pipeline::{
    BoxedStage, BoxedStageFactory, CorePipeline, CorePipelineBuilder, FnStageFactory, PipelineError,
    PipelineStage, Stage, StageFactory, StageId, StageRegistry,
};
