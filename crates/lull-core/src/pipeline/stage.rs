use lull_spec::enums::EngineType;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct StageId(EngineType);

impl StageId {
    pub fn new(inner: EngineType) -> Self {
        Self(inner)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PipelineStage<SP> {
    pub id: StageId,
    pub params: SP,
}

impl<SP> PipelineStage<SP> {
    pub fn new(id: StageId, params: SP) -> Self {
        Self { id, params }
    }
}
