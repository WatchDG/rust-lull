pub trait Stage<M, E> {
    fn process(&mut self, input: M) -> Result<M, E>;
}

pub trait StageFactory<M, SP, E> {
    fn create(&self, params: &SP) -> Result<Box<dyn Stage<M, E>>, E>;
}

pub type BoxedStage<M, E> = Box<dyn Stage<M, E>>;
pub type BoxedStageFactory<M, SP, E> = Box<dyn StageFactory<M, SP, E>>;

pub struct FnStageFactory<F>(pub F);

impl<M, SP, E, F, H> StageFactory<M, SP, E> for FnStageFactory<F>
where
    F: for<'a> Fn(&'a SP) -> Result<H, E>,
    H: Stage<M, E> + 'static,
{
    fn create(&self, params: &SP) -> Result<BoxedStage<M, E>, E> {
        Ok(Box::new((self.0)(params)?))
    }
}
