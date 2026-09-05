use super::stage::StageId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PipelineError<E> {
    UnknownStage(StageId),
    Stage { id: StageId, source: E },
}
