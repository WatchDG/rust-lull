use std::collections::HashMap;

use super::handler::{BoxedStageFactory, StageFactory};
use super::stage::StageId;

pub struct StageRegistry<M, SP, E> {
    factories: HashMap<StageId, BoxedStageFactory<M, SP, E>>,
}

impl<M, SP, E> StageRegistry<M, SP, E> {
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: StageId, factory: BoxedStageFactory<M, SP, E>) {
        self.factories.insert(id, factory);
    }

    pub fn get(&self, id: &StageId) -> Option<&dyn StageFactory<M, SP, E>> {
        self.factories.get(id).map(|factory| factory.as_ref())
    }
}

impl<M, SP, E> Default for StageRegistry<M, SP, E> {
    fn default() -> Self {
        Self::new()
    }
}
