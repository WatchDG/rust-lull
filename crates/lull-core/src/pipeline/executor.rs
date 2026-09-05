use super::error::PipelineError;
use super::handler::BoxedStage;
use super::stage::StageId;

pub(super) struct PreparedStage<SID, M, E> {
    pub id: StageId<SID>,
    pub handler: BoxedStage<M, E>,
}

pub(super) fn execute<SID, M, E>(
    stages: &mut [PreparedStage<SID, M, E>],
    mut input: M,
) -> Result<M, PipelineError<SID, E>>
where
    SID: Clone,
{
    for stage in stages {
        input = stage
            .handler
            .process(input)
            .map_err(|source| PipelineError::Stage {
                id: stage.id.clone(),
                source,
            })?;
    }
    Ok(input)
}
