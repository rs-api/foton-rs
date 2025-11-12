//! Per-route middleware support.

use hyper::Method;
use std::sync::Arc;

use crate::{Handler, Middleware, handler::IntoHandler, middleware::FnMiddleware};

/// Route with optional middleware.
pub struct Route<S = ()> {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) handler: Arc<dyn Handler<S>>,
    pub(crate) middlewares: Vec<Arc<dyn Middleware<S>>>,
}

impl<S: Send + Sync + 'static> Route<S> {
    pub(crate) fn new(method: Method, path: String, handler: Arc<dyn Handler<S>>) -> Self {
        Self {
            method,
            path,
            handler,
            middlewares: Vec::new(),
        }
    }

    /// Add middleware to this route.
    pub fn layer<F, Fut>(mut self, middleware: F) -> Self
    where
        F: Fn(crate::Req, Arc<S>, crate::Next<S>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = crate::Res> + Send + 'static,
    {
        self.middlewares.push(Arc::new(FnMiddleware(middleware)));
        self
    }

    /// Add middleware from trait implementation.
    pub fn layer_middleware<M: Middleware<S>>(mut self, middleware: M) -> Self {
        self.middlewares.push(Arc::new(middleware));
        self
    }

    /// Create GET route.
    pub fn get<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::GET, path.into(), handler.into_handler())
    }

    /// Create POST route.
    pub fn post<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::POST, path.into(), handler.into_handler())
    }

    /// Create PUT route.
    pub fn put<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::PUT, path.into(), handler.into_handler())
    }

    /// Create DELETE route.
    pub fn delete<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::DELETE, path.into(), handler.into_handler())
    }

    /// Create PATCH route.
    pub fn patch<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::PATCH, path.into(), handler.into_handler())
    }
}
