use std::collections::HashMap;
use std::hash::Hash;

use super::handler::{BoxedStageFactory, StageFactory};
use super::stage::StageId;

pub struct StageRegistry<SID, M, SP, E> {
    factories: HashMap<StageId<SID>, BoxedStageFactory<M, SP, E>>,
}

impl<SID, M, SP, E> StageRegistry<SID, M, SP, E>
where
    SID: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(&mut self, id: StageId<SID>, factory: BoxedStageFactory<M, SP, E>) {
        self.factories.insert(id, factory);
    }

    pub fn get(&self, id: &StageId<SID>) -> Option<&dyn StageFactory<M, SP, E>> {
        self.factories.get(id).map(|factory| factory.as_ref())
    }
}

impl<SID, M, SP, E> Default for StageRegistry<SID, M, SP, E>
where
    SID: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
