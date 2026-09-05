pub trait NodeHandler<M, E> {
    fn process(&mut self, inputs: &[M]) -> Result<Vec<M>, E>;
}

pub trait NodeFactory<M, NP, E> {
    fn create(&self, params: &NP) -> Result<Box<dyn NodeHandler<M, E>>, E>;
}

pub type BoxedHandler<M, E> = Box<dyn NodeHandler<M, E>>;
pub type BoxedFactory<M, NP, E> = Box<dyn NodeFactory<M, NP, E>>;

pub struct FnFactory<F>(pub F);

impl<M, NP, E, F, H> NodeFactory<M, NP, E> for FnFactory<F>
where
    F: for<'a> Fn(&'a NP) -> Result<H, E>,
    H: NodeHandler<M, E> + 'static,
{
    fn create(&self, params: &NP) -> Result<BoxedHandler<M, E>, E> {
        Ok(Box::new((self.0)(params)?))
    }
}
