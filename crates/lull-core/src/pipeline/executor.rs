use super::error::PipelineError;
use super::handler::BoxedStage;
use super::stage::StageId;

pub(super) struct PreparedStage<M, E> {
    pub id: StageId,
    pub handler: BoxedStage<M, E>,
}

pub(super) fn execute<M, E>(
    stages: &mut [PreparedStage<M, E>],
    mut input: M,
) -> Result<M, PipelineError<E>> {
    for stage in stages {
        input = stage
            .handler
            .process(input)
            .map_err(|source| PipelineError::Stage {
                id: stage.id,
                source,
            })?;
    }
    Ok(input)
}
