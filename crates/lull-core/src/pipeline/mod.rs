//! Linear stage chain (chain of responsibility).
//!
//! The flow is a list of stage names. A registry maps each name to a factory.
//! The executor walks the list and calls `process` on each handler in order.

mod error;
mod executor;
mod handler;
mod registry;
mod stage;

pub use error::PipelineError;
pub use handler::{BoxedStage, BoxedStageFactory, FnStageFactory, Stage, StageFactory};
pub use registry::StageRegistry;
pub use stage::{PipelineStage, StageId};

use std::hash::Hash;

use executor::{execute, PreparedStage};

pub struct CorePipelineBuilder<SID, M, SP, E> {
    stages: Vec<PipelineStage<SID, SP>>,
    registry: StageRegistry<SID, M, SP, E>,
}

impl<SID, M, SP, E> CorePipelineBuilder<SID, M, SP, E>
where
    SID: Clone + Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            registry: StageRegistry::new(),
        }
    }

    pub fn stage(mut self, stage: PipelineStage<SID, SP>) -> Self {
        self.stages.push(stage);
        self
    }

    pub fn register(mut self, id: StageId<SID>, factory: BoxedStageFactory<M, SP, E>) -> Self {
        self.registry.register(id, factory);
        self
    }

    pub fn build(self) -> Result<CorePipeline<SID, M, SP, E>, PipelineError<SID, E>> {
        for stage in &self.stages {
            if self.registry.get(&stage.id).is_none() {
                return Err(PipelineError::UnknownStage(stage.id.clone()));
            }
        }
        Ok(CorePipeline {
            stages: self.stages,
            registry: self.registry,
        })
    }
}

impl<SID, M, SP, E> Default for CorePipelineBuilder<SID, M, SP, E>
where
    SID: Clone + Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}

pub struct CorePipeline<SID, M, SP, E> {
    stages: Vec<PipelineStage<SID, SP>>,
    registry: StageRegistry<SID, M, SP, E>,
}

impl<SID, M, SP, E> CorePipeline<SID, M, SP, E>
where
    SID: Clone + Eq + Hash,
{
    pub fn execute(&self, input: M) -> Result<M, PipelineError<SID, E>> {
        let mut prepared = Vec::with_capacity(self.stages.len());
        for stage in &self.stages {
            let factory = self
                .registry
                .get(&stage.id)
                .ok_or_else(|| PipelineError::UnknownStage(stage.id.clone()))?;
            let handler = factory
                .create(&stage.params)
                .map_err(|source| PipelineError::Stage {
                    id: stage.id.clone(),
                    source,
                })?;
            prepared.push(PreparedStage {
                id: stage.id.clone(),
                handler,
            });
        }
        execute(&mut prepared, input)
    }

    pub fn len(&self) -> usize {
        self.stages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.stages.is_empty()
    }
}
