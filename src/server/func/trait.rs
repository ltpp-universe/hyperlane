use crate::*;

pub trait Func:
    Fn(ArcRwLockControllerData) -> Pin<Box<dyn Future<Output = ()> + Send + 'static>>
    + Send
    + Sync
    + 'static
{
}

pub trait FuncWithoutPin<Fut>: Fn(ArcRwLockControllerData) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}
