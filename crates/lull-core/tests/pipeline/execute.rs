use lull_core::{
    CorePipelineBuilder, FnStageFactory, PipelineError, PipelineStage, Stage, StageId,
};
use lull_spec::enums::TradeSignal;

struct PassThrough;

impl Stage<TradeSignal, ()> for PassThrough {
    fn process(&mut self, input: TradeSignal) -> Result<TradeSignal, ()> {
        Ok(input)
    }
}

struct EmitBuy;

impl Stage<TradeSignal, ()> for EmitBuy {
    fn process(&mut self, _input: TradeSignal) -> Result<TradeSignal, ()> {
        Ok(TradeSignal::Buy)
    }
}

fn pass_through(_: &()) -> Result<PassThrough, ()> {
    Ok(PassThrough)
}

fn emit_buy(_: &()) -> Result<EmitBuy, ()> {
    Ok(EmitBuy)
}

fn builder() -> CorePipelineBuilder<&'static str, TradeSignal, (), ()> {
    CorePipelineBuilder::new()
        .register(
            StageId::new("strategy"),
            Box::new(FnStageFactory(emit_buy)),
        )
        .register(
            StageId::new("execution"),
            Box::new(FnStageFactory(pass_through)),
        )
        .register(StageId::new("risk"), Box::new(FnStageFactory(pass_through)))
}

#[test]
fn executes_stages_in_list_order() {
    let pipeline = builder()
        .stage(PipelineStage::new(StageId::new("strategy"), ()))
        .stage(PipelineStage::new(StageId::new("execution"), ()))
        .stage(PipelineStage::new(StageId::new("risk"), ()))
        .build()
        .unwrap();
    assert_eq!(pipeline.len(), 3);
    assert_eq!(
        pipeline.execute(TradeSignal::Hold).unwrap(),
        TradeSignal::Buy
    );
}

#[test]
fn empty_pipeline_returns_the_input() {
    let pipeline = builder().build().unwrap();
    assert_eq!(
        pipeline.execute(TradeSignal::Hold).unwrap(),
        TradeSignal::Hold
    );
}

#[test]
fn stage_error_keeps_the_stage_id() {
    struct Boom;

    impl Stage<TradeSignal, &'static str> for Boom {
        fn process(&mut self, _input: TradeSignal) -> Result<TradeSignal, &'static str> {
            Err("blocked")
        }
    }

    fn boom(_: &()) -> Result<Boom, &'static str> {
        Ok(Boom)
    }

    let pipeline = CorePipelineBuilder::new()
        .register(StageId::new("risk"), Box::new(FnStageFactory(boom)))
        .stage(PipelineStage::new(StageId::new("risk"), ()))
        .build()
        .unwrap();
    let error = pipeline.execute(TradeSignal::Buy).unwrap_err();
    assert_eq!(
        error,
        PipelineError::Stage {
            id: StageId::new("risk"),
            source: "blocked",
        }
    );
}
