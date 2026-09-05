#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StageId<SID>(SID);

impl<SID> StageId<SID> {
    pub fn new(inner: SID) -> Self {
        Self(inner)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineStage<SID, SP> {
    pub id: StageId<SID>,
    pub params: SP,
}

impl<SID, SP> PipelineStage<SID, SP> {
    pub fn new(id: StageId<SID>, params: SP) -> Self {
        Self { id, params }
    }
}
