use lull_core::{CorePipelineBuilder, EngineType, PipelineError, PipelineStage, StageId};

#[test]
fn builder_returns_empty_pipeline() {
    let pipeline = CorePipelineBuilder::<(), (), ()>::new().build().unwrap();
    assert!(pipeline.is_empty());
}

#[test]
fn builder_rejects_unknown_stage() {
    let result = CorePipelineBuilder::<(), (), ()>::new()
        .stage(PipelineStage::new(StageId::new(EngineType::Risk), ()))
        .build();
    assert!(matches!(
        result,
        Err(PipelineError::UnknownStage(id)) if id == StageId::new(EngineType::Risk)
    ));
}
