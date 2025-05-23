use crate::*;

#[derive(Clone, Getter, Setter)]
pub struct Server {
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) config: ArcRwLockServerConfig<'static>,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) route: ArcRwLockHashMapRouteFuncBox,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) route_matcher: ArcRwLockRouteMatcher,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) request_middleware: ArcRwLockMiddlewareFuncBox,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) response_middleware: ArcRwLockMiddlewareFuncBox,
}

#[derive(Clone)]
pub(crate) struct RequestHandlerImmutableParams<'a> {
    pub(super) stream: &'a ArcRwLockStream,
    pub(super) config: &'a ServerConfig<'a>,
    pub(super) request_middleware: &'a ArcRwLockMiddlewareFuncBox,
    pub(super) response_middleware: &'a ArcRwLockMiddlewareFuncBox,
    pub(super) route_func: &'a ArcRwLockHashMapRouteFuncBox,
    pub(super) route_matcher: &'a ArcRwLockRouteMatcher,
}
