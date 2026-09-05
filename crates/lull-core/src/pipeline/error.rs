use super::stage::StageId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineError<SID, E> {
    UnknownStage(StageId<SID>),
    Stage { id: StageId<SID>, source: E },
}
