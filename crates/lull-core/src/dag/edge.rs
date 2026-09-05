use super::node::NodeId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Edge<NID> {
    pub from: NodeId<NID>,
    pub to: NodeId<NID>,
}

impl<NID> Edge<NID> {
    pub fn new(from: NodeId<NID>, to: NodeId<NID>) -> Self {
        Self { from, to }
    }
}
