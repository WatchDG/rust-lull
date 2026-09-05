//! Linear stage chain (chain of responsibility).
//!
//! The flow is a list of `EngineType` stages. A registry maps each type to a
//! factory. The executor walks the list and calls `process` on each handler.

mod error;
mod executor;
mod handler;
mod registry;
mod stage;

pub use error::PipelineError;
pub use handler::{BoxedStage, BoxedStageFactory, FnStageFactory, Stage, StageFactory};
pub use registry::StageRegistry;
pub use stage::{PipelineStage, StageId};

use executor::{execute, PreparedStage};

pub struct CorePipelineBuilder<M, SP, E> {
    stages: Vec<PipelineStage<SP>>,
    registry: StageRegistry<M, SP, E>,
}

impl<M, SP, E> CorePipelineBuilder<M, SP, E> {
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
            registry: StageRegistry::new(),
        }
    }

    pub fn stage(mut self, stage: PipelineStage<SP>) -> Self {
        self.stages.push(stage);
        self
    }

    pub fn register(mut self, id: StageId, factory: BoxedStageFactory<M, SP, E>) -> Self {
        self.registry.register(id, factory);
        self
    }

    pub fn build(self) -> Result<CorePipeline<M, SP, E>, PipelineError<E>> {
        for stage in &self.stages {
            if self.registry.get(&stage.id).is_none() {
                return Err(PipelineError::UnknownStage(stage.id));
            }
        }
        Ok(CorePipeline {
            stages: self.stages,
            registry: self.registry,
        })
    }
}

impl<M, SP, E> Default for CorePipelineBuilder<M, SP, E> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct CorePipeline<M, SP, E> {
    stages: Vec<PipelineStage<SP>>,
    registry: StageRegistry<M, SP, E>,
}

impl<M, SP, E> CorePipeline<M, SP, E> {
    pub fn execute(&self, input: M) -> Result<M, PipelineError<E>> {
        let mut prepared = Vec::with_capacity(self.stages.len());
        for stage in &self.stages {
            let factory = self
                .registry
                .get(&stage.id)
                .ok_or(PipelineError::UnknownStage(stage.id))?;
            let handler = factory
                .create(&stage.params)
                .map_err(|source| PipelineError::Stage {
                    id: stage.id,
                    source,
                })?;
            prepared.push(PreparedStage {
                id: stage.id,
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
