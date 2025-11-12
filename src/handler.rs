//! Request handlers with extractor support.

use async_trait::async_trait;
use std::sync::Arc;

use crate::extractors::FromRequest;
use crate::{IntoRes, Req, Res};

/// Convert function into boxed handler.
pub trait IntoHandler<S, T> {
    fn into_handler(self) -> Arc<dyn Handler<S>>;
}

/// Request handler.
#[async_trait]
pub trait Handler<S = ()>: Send + Sync + 'static {
    /// Handle incoming request.
    async fn call(&self, req: Req, state: Arc<S>) -> Res;
}

/// Function handler wrapper.
pub struct FnHandler<F>(pub F);

#[async_trait]
impl<F, Fut, S> Handler<S> for FnHandler<F>
where
    F: Fn(Req) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: IntoRes,
    S: Send + Sync + 'static,
{
    async fn call(&self, req: Req, _state: Arc<S>) -> Res {
        (self.0)(req).await.into_res()
    }
}

/// Handler with one extractor.
pub struct FnHandler1<F, E1> {
    handler: F,
    _marker: std::marker::PhantomData<E1>,
}

impl<F, E1> FnHandler1<F, E1> {
    /// Create handler wrapper.
    pub fn new(handler: F) -> Self {
        Self {
            handler,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<F, Fut, S, E1> Handler<S> for FnHandler1<F, E1>
where
    F: Fn(E1) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: IntoRes,
    S: Clone + Send + Sync + 'static,
    E1: FromRequest<S> + Send + Sync + 'static,
{
    async fn call(&self, mut req: Req, state: Arc<S>) -> Res {
        let e1 = match E1::from_request(&mut req, &state).await {
            Ok(v) => v,
            Err(e) => return e.into_res(),
        };

        (self.handler)(e1).await.into_res()
    }
}

/// Handler with two extractors.
pub struct FnHandler2<F, E1, E2> {
    handler: F,
    _marker: std::marker::PhantomData<(E1, E2)>,
}

impl<F, E1, E2> FnHandler2<F, E1, E2> {
    /// Create handler wrapper.
    pub fn new(handler: F) -> Self {
        Self {
            handler,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<F, Fut, S, E1, E2> Handler<S> for FnHandler2<F, E1, E2>
where
    F: Fn(E1, E2) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: IntoRes,
    S: Clone + Send + Sync + 'static,
    E1: FromRequest<S> + Send + Sync + 'static,
    E2: FromRequest<S> + Send + Sync + 'static,
{
    async fn call(&self, mut req: Req, state: Arc<S>) -> Res {
        let e1 = match E1::from_request(&mut req, &state).await {
            Ok(v) => v,
            Err(e) => return e.into_res(),
        };

        let e2 = match E2::from_request(&mut req, &state).await {
            Ok(v) => v,
            Err(e) => return e.into_res(),
        };

        (self.handler)(e1, e2).await.into_res()
    }
}

/// Handler with three extractors.
pub struct FnHandler3<F, E1, E2, E3> {
    handler: F,
    _marker: std::marker::PhantomData<(E1, E2, E3)>,
}

impl<F, E1, E2, E3> FnHandler3<F, E1, E2, E3> {
    /// Create handler wrapper.
    pub fn new(handler: F) -> Self {
        Self {
            handler,
            _marker: std::marker::PhantomData,
        }
    }
}

#[async_trait]
impl<F, Fut, S, E1, E2, E3> Handler<S> for FnHandler3<F, E1, E2, E3>
where
    F: Fn(E1, E2, E3) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: IntoRes,
    S: Clone + Send + Sync + 'static,
    E1: FromRequest<S> + Send + Sync + 'static,
    E2: FromRequest<S> + Send + Sync + 'static,
    E3: FromRequest<S> + Send + Sync + 'static,
{
    async fn call(&self, mut req: Req, state: Arc<S>) -> Res {
        let e1 = match E1::from_request(&mut req, &state).await {
            Ok(v) => v,
            Err(e) => return e.into_res(),
        };

        let e2 = match E2::from_request(&mut req, &state).await {
            Ok(v) => v,
            Err(e) => return e.into_res(),
        };

        let e3 = match E3::from_request(&mut req, &state).await {
            Ok(v) => v,
            Err(e) => return e.into_res(),
        };

        (self.handler)(e1, e2, e3).await.into_res()
    }
}

/// Marker for no extractors.
pub struct NoExtractors;

impl<F, Fut, S> IntoHandler<S, NoExtractors> for F
where
    F: Fn(Req) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: IntoRes,
    S: Send + Sync + 'static,
{
    fn into_handler(self) -> Arc<dyn Handler<S>> {
        Arc::new(FnHandler(self))
    }
}

/// Marker for one extractor.
pub struct OneExtractor;

impl<F, Fut, S, E1> IntoHandler<S, (OneExtractor, E1)> for F
where
    F: Fn(E1) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: IntoRes,
    S: Clone + Send + Sync + 'static,
    E1: FromRequest<S> + Send + Sync + 'static,
{
    fn into_handler(self) -> Arc<dyn Handler<S>> {
        Arc::new(FnHandler1::new(self))
    }
}

/// Marker for two extractors.
pub struct TwoExtractors;

impl<F, Fut, S, E1, E2> IntoHandler<S, (TwoExtractors, E1, E2)> for F
where
    F: Fn(E1, E2) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: IntoRes,
    S: Clone + Send + Sync + 'static,
    E1: FromRequest<S> + Send + Sync + 'static,
    E2: FromRequest<S> + Send + Sync + 'static,
{
    fn into_handler(self) -> Arc<dyn Handler<S>> {
        Arc::new(FnHandler2::new(self))
    }
}

/// Marker for three extractors.
pub struct ThreeExtractors;

impl<F, Fut, S, E1, E2, E3> IntoHandler<S, (ThreeExtractors, E1, E2, E3)> for F
where
    F: Fn(E1, E2, E3) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future + Send + 'static,
    Fut::Output: IntoRes,
    S: Clone + Send + Sync + 'static,
    E1: FromRequest<S> + Send + Sync + 'static,
    E2: FromRequest<S> + Send + Sync + 'static,
    E3: FromRequest<S> + Send + Sync + 'static,
{
    fn into_handler(self) -> Arc<dyn Handler<S>> {
        Arc::new(FnHandler3::new(self))
    }
}
