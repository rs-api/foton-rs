//! HTTP response with optimized serialization.

use bytes::Bytes;
use http_body_util::Full;
use hyper::{Response, StatusCode, header};
use serde::Serialize;

static CONTENT_TYPE_TEXT: header::HeaderValue =
    header::HeaderValue::from_static("text/plain; charset=utf-8");
static CONTENT_TYPE_HTML: header::HeaderValue =
    header::HeaderValue::from_static("text/html; charset=utf-8");
static CONTENT_TYPE_JSON: header::HeaderValue =
    header::HeaderValue::from_static("application/json");

/// HTTP response.
pub struct Res {
    inner: Response<Full<Bytes>>,
}

impl Res {
    /// Create empty 200 response.
    #[inline]
    pub fn new() -> Self {
        Self {
            inner: Response::new(Full::new(Bytes::new())),
        }
    }

    /// Wrap hyper response.
    #[inline]
    pub fn from_hyper(inner: Response<Full<Bytes>>) -> Self {
        Self { inner }
    }

    /// Unwrap to hyper response.
    #[inline]
    pub fn into_hyper(self) -> Response<Full<Bytes>> {
        self.inner
    }

    /// Text response.
    pub fn text(body: impl Into<String>) -> Self {
        let body_str = body.into();
        let mut res = Response::new(Full::new(Bytes::from(body_str)));
        res.headers_mut()
            .insert(header::CONTENT_TYPE, CONTENT_TYPE_TEXT.clone());
        Self { inner: res }
    }

    /// HTML response.
    pub fn html(body: impl Into<String>) -> Self {
        let body_str = body.into();
        let mut res = Response::new(Full::new(Bytes::from(body_str)));
        res.headers_mut()
            .insert(header::CONTENT_TYPE, CONTENT_TYPE_HTML.clone());
        Self { inner: res }
    }

    /// JSON response (serializes to Vec<u8> directly).
    pub fn json<T: Serialize>(value: &T) -> Self {
        match serde_json::to_vec(value) {
            Ok(bytes) => {
                let mut res = Response::new(Full::new(Bytes::from(bytes)));
                res.headers_mut()
                    .insert(header::CONTENT_TYPE, CONTENT_TYPE_JSON.clone());
                Self { inner: res }
            }
            Err(e) => {
                let error_msg = format!(r#"{{"error": "JSON serialization failed: {}"}}"#, e);
                let mut res = Response::new(Full::new(Bytes::from(error_msg)));
                *res.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
                res.headers_mut()
                    .insert(header::CONTENT_TYPE, CONTENT_TYPE_JSON.clone());
                Self { inner: res }
            }
        }
    }

    /// Status-only response.
    pub fn status(code: u16) -> Self {
        let mut res = Response::new(Full::new(Bytes::new()));
        *res.status_mut() = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        Self { inner: res }
    }

    /// Create builder.
    pub fn builder() -> ResBuilder {
        ResBuilder::new()
    }

    /// Get status code.
    pub fn status_code(&self) -> StatusCode {
        self.inner.status()
    }

    /// Add header.
    #[inline]
    pub fn with_header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        if let (Ok(name), Ok(value)) = (
            header::HeaderName::from_bytes(name.as_ref().as_bytes()),
            header::HeaderValue::from_str(value.as_ref()),
        ) {
            self.inner.headers_mut().insert(name, value);
        }
        self
    }

    /// Get mutable headers.
    #[inline]
    pub fn headers_mut(&mut self) -> &mut header::HeaderMap {
        self.inner.headers_mut()
    }

    /// Get headers.
    #[inline]
    pub fn headers(&self) -> &header::HeaderMap {
        self.inner.headers()
    }
}

impl Default for Res {
    fn default() -> Self {
        Self::new()
    }
}

/// Response builder with pre-allocated headers.
pub struct ResBuilder {
    status: StatusCode,
    headers: header::HeaderMap,
}

impl ResBuilder {
    /// Create builder.
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: header::HeaderMap::with_capacity(4),
        }
    }

    /// Set status code.
    pub fn status(mut self, code: u16) -> Self {
        self.status = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        self
    }

    /// Add header.
    pub fn header(mut self, name: impl AsRef<str>, value: impl AsRef<str>) -> Self {
        if let (Ok(name), Ok(value)) = (
            header::HeaderName::from_bytes(name.as_ref().as_bytes()),
            header::HeaderValue::from_str(value.as_ref()),
        ) {
            self.headers.insert(name, value);
        }
        self
    }

    /// Build text response.
    pub fn text(mut self, body: impl Into<String>) -> Res {
        let body_str = body.into();
        let mut res = Response::new(Full::new(Bytes::from(body_str)));
        *res.status_mut() = self.status;

        if !self.headers.contains_key(header::CONTENT_TYPE) {
            self.headers
                .insert(header::CONTENT_TYPE, CONTENT_TYPE_TEXT.clone());
        }

        *res.headers_mut() = self.headers;
        Res { inner: res }
    }

    /// Build HTML response.
    pub fn html(mut self, body: impl Into<String>) -> Res {
        let body_str = body.into();
        let mut res = Response::new(Full::new(Bytes::from(body_str)));
        *res.status_mut() = self.status;

        if !self.headers.contains_key(header::CONTENT_TYPE) {
            self.headers
                .insert(header::CONTENT_TYPE, CONTENT_TYPE_HTML.clone());
        }

        *res.headers_mut() = self.headers;
        Res { inner: res }
    }

    /// Build JSON response.
    pub fn json<T: Serialize>(mut self, value: &T) -> Res {
        match serde_json::to_vec(value) {
            Ok(bytes) => {
                let mut res = Response::new(Full::new(Bytes::from(bytes)));
                *res.status_mut() = self.status;

                if !self.headers.contains_key(header::CONTENT_TYPE) {
                    self.headers
                        .insert(header::CONTENT_TYPE, CONTENT_TYPE_JSON.clone());
                }

                *res.headers_mut() = self.headers;
                Res { inner: res }
            }
            Err(_) => Res::builder().status(500).text("Failed to serialize JSON"),
        }
    }

    /// Build with custom body.
    pub fn body(self, bytes: impl Into<Bytes>) -> Res {
        let mut res = Response::new(Full::new(bytes.into()));
        *res.status_mut() = self.status;
        *res.headers_mut() = self.headers;
        Res { inner: res }
    }
}

impl Default for ResBuilder {
    fn default() -> Self {
        Self::new()
    }
}
