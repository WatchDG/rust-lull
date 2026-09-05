#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct NodeId<NID>(NID);

impl<NID> NodeId<NID> {
    pub fn new(inner: NID) -> Self {
        Self(inner)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Node<NID, ROLE, IMPL, NP> {
    pub id: NodeId<NID>,
    pub r#type: ROLE,
    pub implementation: IMPL,
    pub params: NP,
}

impl<NID, ROLE, IMPL, NP> Node<NID, ROLE, IMPL, NP> {
    pub fn new(id: NodeId<NID>, r#type: ROLE, implementation: IMPL, params: NP) -> Self {
        Self {
            id,
            r#type,
            implementation,
            params,
        }
    }
}
