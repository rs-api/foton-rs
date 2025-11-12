//! Convert types into HTTP responses
//!
//! The [`IntoRes`] trait allows handlers to return various types
//! that are automatically converted to HTTP responses.

use crate::{Error, Res};
use serde::Serialize;

/// Types that can become HTTP responses
pub trait IntoRes {
    /// Convert into response
    fn into_res(self) -> Res;
}

impl IntoRes for Res {
    fn into_res(self) -> Res {
        self
    }
}

impl IntoRes for String {
    fn into_res(self) -> Res {
        Res::text(self)
    }
}

impl IntoRes for &'static str {
    fn into_res(self) -> Res {
        Res::text(self)
    }
}

impl IntoRes for () {
    fn into_res(self) -> Res {
        Res::status(204) // No Content
    }
}

impl<T: IntoRes> IntoRes for Result<T, Error> {
    fn into_res(self) -> Res {
        match self {
            Ok(value) => value.into_res(),
            Err(err) => err.into_res(),
        }
    }
}

impl IntoRes for Error {
    fn into_res(self) -> Res {
        match self {
            Error::Status(code, Some(msg)) => {
                Res::builder().status(code).json(&serde_json::json!({
                    "error": msg,
                    "status": code
                }))
            }
            Error::Status(code, None) => Res::status(code),
            Error::Json(e) => Res::builder().status(400).json(&serde_json::json!({
                "error": format!("JSON error: {}", e),
                "status": 400
            })),
            Error::Hyper(e) => Res::builder().status(500).json(&serde_json::json!({
                "error": format!("HTTP error: {}", e),
                "status": 500
            })),
            Error::Io(e) => Res::builder().status(500).json(&serde_json::json!({
                "error": format!("IO error: {}", e),
                "status": 500
            })),
            Error::Custom(msg) => Res::builder().status(500).json(&serde_json::json!({
                "error": msg,
                "status": 500
            })),
        }
    }
}

/// Wrapper for JSON responses
pub struct Json<T>(pub T);

impl<T: Serialize> IntoRes for Json<T> {
    fn into_res(self) -> Res {
        Res::json(&self.0)
    }
}

/// Wrapper for HTML responses
pub struct Html(pub String);

impl IntoRes for Html {
    fn into_res(self) -> Res {
        Res::html(self.0)
    }
}

impl IntoRes for &Html {
    fn into_res(self) -> Res {
        Res::html(self.0.clone())
    }
}
