use crate::*;

impl<F> Func for F where
    F: Fn(ArcRwLock<ControllerData>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
        + Send
        + Sync
        + 'static
{
}

impl<F, Fut> FuncWithoutPin<Fut> for F
where
    F: Fn(ArcRwLock<ControllerData>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
}
