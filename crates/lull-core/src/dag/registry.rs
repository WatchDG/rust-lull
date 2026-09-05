use std::collections::HashMap;
use std::hash::Hash;

use super::handler::{BoxedFactory, NodeFactory};

pub struct Registry<ROLE, IMPL, M, NP, E> {
    factories: HashMap<(ROLE, IMPL), BoxedFactory<M, NP, E>>,
}

impl<ROLE, IMPL, M, NP, E> Registry<ROLE, IMPL, M, NP, E>
where
    ROLE: Eq + Hash,
    IMPL: Eq + Hash,
{
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    pub fn register(
        &mut self,
        r#type: ROLE,
        implementation: IMPL,
        factory: BoxedFactory<M, NP, E>,
    ) {
        self.factories.insert((r#type, implementation), factory);
    }

    pub fn get(&self, r#type: &ROLE, implementation: &IMPL) -> Option<&dyn NodeFactory<M, NP, E>>
    where
        ROLE: Clone,
        IMPL: Clone,
    {
        self.factories
            .get(&(r#type.clone(), implementation.clone()))
            .map(|factory| factory.as_ref())
    }
}

impl<ROLE, IMPL, M, NP, E> Default for Registry<ROLE, IMPL, M, NP, E>
where
    ROLE: Eq + Hash,
    IMPL: Eq + Hash,
{
    fn default() -> Self {
        Self::new()
    }
}
