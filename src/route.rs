//! Per-route middleware support
//!
//! Routes can have their own middleware that only applies to that specific route.

use hyper::Method;
use std::sync::Arc;

use crate::{Handler, Middleware, handler::IntoHandler, middleware::FnMiddleware};

/// A route with optional middleware
pub struct Route<S = ()> {
    pub(crate) method: Method,
    pub(crate) path: String,
    pub(crate) handler: Arc<dyn Handler<S>>,
    pub(crate) middlewares: Vec<Arc<dyn Middleware<S>>>,
}

impl<S: Send + Sync + 'static> Route<S> {
    /// Create a new route
    pub(crate) fn new(method: Method, path: String, handler: Arc<dyn Handler<S>>) -> Self {
        Self {
            method,
            path,
            handler,
            middlewares: Vec::new(),
        }
    }

    /// Add middleware to this route
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use rust_api::prelude::*;
    ///
    /// let app = RustApi::new()
    ///     .route(
    ///         Route::get("/admin", admin_handler)
    ///             .layer(auth_middleware)
    ///             .layer(rate_limit_middleware)
    ///     );
    /// ```
    pub fn layer<F, Fut>(mut self, middleware: F) -> Self
    where
        F: Fn(crate::Req, Arc<S>, crate::Next<S>) -> Fut + Send + Sync + 'static,
        Fut: std::future::Future<Output = crate::Res> + Send + 'static,
    {
        self.middlewares.push(Arc::new(FnMiddleware(middleware)));
        self
    }

    /// Add middleware from a Middleware trait implementation
    pub fn layer_middleware<M: Middleware<S>>(mut self, middleware: M) -> Self {
        self.middlewares.push(Arc::new(middleware));
        self
    }

    /// Helper to create a GET route
    pub fn get<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::GET, path.into(), handler.into_handler())
    }

    /// Helper to create a POST route
    pub fn post<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::POST, path.into(), handler.into_handler())
    }

    /// Helper to create a PUT route
    pub fn put<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::PUT, path.into(), handler.into_handler())
    }

    /// Helper to create a DELETE route
    pub fn delete<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::DELETE, path.into(), handler.into_handler())
    }

    /// Helper to create a PATCH route
    pub fn patch<H, T>(path: impl Into<String>, handler: H) -> Self
    where
        H: IntoHandler<S, T>,
    {
        Self::new(Method::PATCH, path.into(), handler.into_handler())
    }
}

// The .route() method is implemented in api.rs to access private fields
