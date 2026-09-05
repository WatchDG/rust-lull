use lull_core::{CorePipelineBuilder, PipelineError, PipelineStage, StageId};

#[test]
fn builder_returns_empty_pipeline() {
    let pipeline = CorePipelineBuilder::<&'static str, (), (), ()>::new()
        .build()
        .unwrap();
    assert!(pipeline.is_empty());
}

#[test]
fn builder_rejects_unknown_stage() {
    let result = CorePipelineBuilder::<_, (), (), ()>::new()
        .stage(PipelineStage::new(StageId::new("risk"), ()))
        .build();
    assert!(matches!(
        result,
        Err(PipelineError::UnknownStage(id)) if id == StageId::new("risk")
    ));
}
