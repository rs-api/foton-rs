//! Request handlers with macro-generated extractors.

use async_trait::async_trait;
use std::sync::Arc;

use crate::extractors::FromRequest;
use crate::{IntoRes, Req, Res};

/// Convert function to handler.
pub trait IntoHandler<S, T> {
    fn into_handler(self) -> Arc<dyn Handler<S>>;
}

/// Request handler trait.
#[async_trait]
pub trait Handler<S = ()>: Send + Sync + 'static {
    /// Handle request.
    async fn call(&self, req: Req, state: Arc<S>) -> Res;
}

/// Extract or return error response.
macro_rules! extract_or_return {
    ($req:expr, $state:expr, $extractor:ty) => {
        match <$extractor as FromRequest<_>>::from_request($req, $state).await {
            Ok(v) => v,
            Err(e) => return e.into_res(),
        }
    };
}

/// Generate handler for N extractors.
macro_rules! impl_handler {
    (0) => {
        /// Handler for functions taking only Req.
        pub struct FnHandler<F>(pub F);

        impl<F> FnHandler<F> {
            /// Create handler.
            #[inline]
            pub fn new(handler: F) -> Self {
                Self(handler)
            }
        }

        #[async_trait]
        impl<F, Fut, S> Handler<S> for FnHandler<F>
        where
            F: Fn(Req) -> Fut + Send + Sync + 'static,
            Fut: std::future::Future + Send + 'static,
            Fut::Output: IntoRes,
            S: Send + Sync + 'static,
        {
            #[inline]
            async fn call(&self, req: Req, _state: Arc<S>) -> Res {
                (self.0)(req).await.into_res()
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
            #[inline]
            fn into_handler(self) -> Arc<dyn Handler<S>> {
                Arc::new(FnHandler::new(self))
            }
        }
    };

    ($n:tt, $($extractor:ident),+) => {
        paste::paste! {
            #[doc = "Handler with " $n " extractor(s)."]
            pub struct [<FnHandler $n>]<F, $($extractor),+> {
                handler: F,
                _marker: std::marker::PhantomData<($($extractor,)+)>,
            }

            impl<F, $($extractor),+> [<FnHandler $n>]<F, $($extractor),+> {
                #[doc = "Create handler."]
                #[inline]
                pub fn new(handler: F) -> Self {
                    Self {
                        handler,
                        _marker: std::marker::PhantomData,
                    }
                }
            }

            #[async_trait]
            impl<F, Fut, S, $($extractor),+> Handler<S> for [<FnHandler $n>]<F, $($extractor),+>
            where
                F: Fn($($extractor),+) -> Fut + Send + Sync + 'static,
                Fut: std::future::Future + Send + 'static,
                Fut::Output: IntoRes,
                S: Clone + Send + Sync + 'static,
                $($extractor: FromRequest<S> + Send + Sync + 'static,)+
            {
                #[inline]
                async fn call(&self, mut req: Req, state: Arc<S>) -> Res {
                    $(
                        let [<e $extractor:lower>] = extract_or_return!(&mut req, &state, $extractor);
                    )+
                    (self.handler)($([<e $extractor:lower>]),+).await.into_res()
                }
            }

            #[doc = "Marker for " $n " extractor(s)."]
            pub struct [<Extractor $n>];

            impl<F, Fut, S, $($extractor),+> IntoHandler<S, ([<Extractor $n>], $($extractor),+)> for F
            where
                F: Fn($($extractor),+) -> Fut + Send + Sync + 'static,
                Fut: std::future::Future + Send + 'static,
                Fut::Output: IntoRes,
                S: Clone + Send + Sync + 'static,
                $($extractor: FromRequest<S> + Send + Sync + 'static,)+
            {
                #[inline]
                fn into_handler(self) -> Arc<dyn Handler<S>> {
                    Arc::new([<FnHandler $n>]::new(self))
                }
            }
        }
    };
}

impl_handler!(0);
impl_handler!(1, E1);
impl_handler!(2, E1, E2);
impl_handler!(3, E1, E2, E3);
impl_handler!(4, E1, E2, E3, E4);
impl_handler!(5, E1, E2, E3, E4, E5);
impl_handler!(6, E1, E2, E3, E4, E5, E6);
impl_handler!(7, E1, E2, E3, E4, E5, E6, E7);
impl_handler!(8, E1, E2, E3, E4, E5, E6, E7, E8);
