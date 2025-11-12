//! HTTP request wrapper.

use bytes::Bytes;
use http_body_util::BodyExt;
use hyper::{Method, Request, Uri, body::Incoming, header};
use std::collections::HashMap;

use crate::extensions::Extensions;
use crate::{Error, Result};

static EMPTY_BYTES: Bytes = Bytes::new();

/// HTTP request with parsed body and extensions.
pub struct Req {
    inner: Request<Incoming>,
    path_params: HashMap<String, String>,
    body_bytes: Option<Bytes>,
    extensions: Extensions,
}

impl Req {
    /// Create from hyper request.
    pub fn from_hyper(inner: Request<Incoming>) -> Self {
        Self {
            inner,
            path_params: HashMap::new(),
            body_bytes: None,
            extensions: Extensions::new(),
        }
    }

    /// Get HTTP method.
    pub fn method(&self) -> &Method {
        self.inner.method()
    }

    /// Get request URI.
    pub fn uri(&self) -> &Uri {
        self.inner.uri()
    }

    /// Get request path.
    pub fn path(&self) -> &str {
        self.inner.uri().path()
    }

    /// Get query string.
    pub fn query(&self) -> Option<&str> {
        self.inner.uri().query()
    }

    /// Get header value by name.
    pub fn header(&self, name: &str) -> Option<&str> {
        self.inner.headers().get(name).and_then(|v| v.to_str().ok())
    }

    /// Get all headers.
    pub fn headers(&self) -> &header::HeaderMap {
        self.inner.headers()
    }

    /// Get path parameter by name.
    pub fn param(&self, name: &str) -> Option<&str> {
        self.path_params.get(name).map(|s| s.as_str())
    }

    /// Get all path parameters.
    pub fn params(&self) -> &HashMap<String, String> {
        &self.path_params
    }

    /// Get path parameters for extractors.
    pub fn path_params(&self) -> &HashMap<String, String> {
        &self.path_params
    }

    /// Get request body as bytes.
    pub fn body(&self) -> &Bytes {
        self.body_bytes.as_ref().unwrap_or(&EMPTY_BYTES)
    }

    /// Get content type header value.
    pub fn content_type(&self) -> Option<&str> {
        self.header(header::CONTENT_TYPE.as_str())
    }

    /// Check if Content-Type is JSON.
    pub fn is_json(&self) -> bool {
        self.content_type()
            .map(|ct| ct.contains("application/json"))
            .unwrap_or(false)
    }

    /// Get request extensions.
    pub fn extensions(&self) -> &Extensions {
        &self.extensions
    }

    /// Get mutable request extensions.
    pub fn extensions_mut(&mut self) -> &mut Extensions {
        &mut self.extensions
    }

    /// Convert to underlying hyper request.
    pub fn into_hyper(self) -> Request<Incoming> {
        self.inner
    }

    pub(crate) fn set_path_params(&mut self, params: HashMap<String, String>) {
        self.path_params = params;
    }

    pub(crate) async fn consume_body(mut self) -> Result<Self> {
        let body = self.inner.body_mut();
        let collected = body
            .collect()
            .await
            .map_err(|e| Error::Custom(format!("Failed to read body: {}", e)))?;

        self.body_bytes = Some(collected.to_bytes());
        Ok(self)
    }
}
