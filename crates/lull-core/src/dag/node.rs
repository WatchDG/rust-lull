use lull_spec::enums::EngineType;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId<NID>(NID);

impl<NID> NodeId<NID> {
    pub fn new(inner: NID) -> Self {
        Self(inner)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<NID, IMPL, NP> {
    pub id: NodeId<NID>,
    pub r#type: EngineType,
    pub implementation: IMPL,
    pub params: NP,
}

impl<NID, IMPL, NP> Node<NID, IMPL, NP> {
    pub fn new(id: NodeId<NID>, r#type: EngineType, implementation: IMPL, params: NP) -> Self {
        Self {
            id,
            r#type,
            implementation,
            params,
        }
    }
}
