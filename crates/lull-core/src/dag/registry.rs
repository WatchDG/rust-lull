use std::collections::HashMap;
use std::hash::Hash;

use lull_spec::enums::EngineType;

use super::handler::{BoxedFactory, NodeFactory};

pub struct Registry<IMPL, M, NP, E> {
    factories: HashMap<(EngineType, IMPL), BoxedFactory<M, NP, E>>,
}

impl<IMPL, M, NP, E> Registry<IMPL, M, NP, E>
where
    IMPL: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        r#type: EngineType,
        implementation: IMPL,
        factory: BoxedFactory<M, NP, E>,
    ) {
        self.factories.insert((r#type, implementation), factory);
    }

    pub fn get(
        &self,
        r#type: EngineType,
        implementation: &IMPL,
    ) -> Option<&dyn NodeFactory<M, NP, E>>
    where
        IMPL: Clone,
    {
        self.factories
            .get(&(r#type, implementation.clone()))
            .map(|factory| factory.as_ref())
    }
}

impl<IMPL, M, NP, E> Default for Registry<IMPL, M, NP, E>
where
    IMPL: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
