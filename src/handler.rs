//! Request handler trait
//!
//! Handlers are async functions that take a [`Req`](crate::Req)
//! and return anything that implements [`IntoRes`](crate::IntoRes).

use async_trait::async_trait;
use std::sync::Arc;

use crate::{IntoRes, Req, Res};

/// Request handler
#[async_trait]
pub trait Handler<S = ()>: Send + Sync + 'static {
    /// Handle a request
    async fn call(&self, req: Req, state: Arc<S>) -> Res;
}

/// Wrapper for function handlers
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
