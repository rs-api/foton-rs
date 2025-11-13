//! Response conversion trait.

use crate::{Error, Res};
use std::borrow::Cow;

/// Convert type to HTTP response.
pub trait IntoRes {
    /// Convert to response.
    fn into_res(self) -> Res;
}

impl IntoRes for Res {
    #[inline]
    fn into_res(self) -> Res {
        self
    }
}

impl IntoRes for String {
    #[inline]
    fn into_res(self) -> Res {
        Res::text(self)
    }
}

impl IntoRes for &'static str {
    #[inline]
    fn into_res(self) -> Res {
        Res::text(self)
    }
}

impl IntoRes for () {
    #[inline]
    fn into_res(self) -> Res {
        Res::status(204)
    }
}

impl<T: IntoRes> IntoRes for Result<T, Error> {
    #[inline]
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
            Error::Status(code, Some(msg)) => Res::builder()
                .status(code)
                .text(format!("{} {}", code, msg)),
            Error::Status(code, None) => Res::status(code),
            Error::Json(e) => Res::builder()
                .status(400)
                .text(format!("JSON error: {}", e)),
            Error::Hyper(e) => Res::builder()
                .status(500)
                .text(format!("HTTP error: {}", e)),
            Error::Io(e) => Res::builder().status(500).text(format!("IO error: {}", e)),
            Error::Custom(msg) => Res::builder().status(500).text(msg),
        }
    }
}

/// HTML response wrapper.
pub struct Html(pub Cow<'static, str>);

impl Html {
    /// Create HTML from static string (zero-copy).
    #[inline]
    pub fn from_static(html: &'static str) -> Self {
        Html(Cow::Borrowed(html))
    }

    /// Create HTML from owned string.
    #[inline]
    pub fn from_string(html: String) -> Self {
        Html(Cow::Owned(html))
    }
}

impl From<&'static str> for Html {
    #[inline]
    fn from(s: &'static str) -> Self {
        Html::from_static(s)
    }
}

impl From<String> for Html {
    #[inline]
    fn from(s: String) -> Self {
        Html::from_string(s)
    }
}

impl IntoRes for Html {
    #[inline]
    fn into_res(self) -> Res {
        Res::html(self.0.into_owned())
    }
}

impl IntoRes for &Html {
    fn into_res(self) -> Res {
        match &self.0 {
            Cow::Borrowed(s) => Res::html(*s),
            Cow::Owned(s) => Res::html(s.clone()),
        }
    }
}

/// Common HTTP status responses.
#[allow(dead_code)]
pub mod status {
    use super::*;

    /// 200 OK.
    #[inline]
    pub fn ok() -> Res {
        Res::status(200)
    }

    /// 201 Created.
    #[inline]
    pub fn created() -> Res {
        Res::status(201)
    }

    /// 204 No Content.
    #[inline]
    pub fn no_content() -> Res {
        Res::status(204)
    }

    /// 400 Bad Request.
    #[inline]
    pub fn bad_request(msg: impl Into<String>) -> Res {
        Res::builder().status(400).text(msg.into())
    }

    /// 401 Unauthorized.
    #[inline]
    pub fn unauthorized() -> Res {
        Res::builder().status(401).text("Unauthorized")
    }

    /// 403 Forbidden.
    #[inline]
    pub fn forbidden() -> Res {
        Res::builder().status(403).text("Forbidden")
    }

    /// 404 Not Found.
    #[inline]
    pub fn not_found() -> Res {
        Res::builder().status(404).text("Not Found")
    }

    /// 500 Internal Server Error.
    #[inline]
    pub fn internal_error() -> Res {
        Res::builder().status(500).text("Internal Server Error")
    }

    /// 503 Service Unavailable.
    #[inline]
    pub fn service_unavailable() -> Res {
        Res::builder().status(503).text("Service Unavailable")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_html_from_static() {
        let html = Html::from_static("<h1>Hello</h1>");
        assert!(matches!(html.0, Cow::Borrowed(_)));
    }

    #[test]
    fn test_html_from_string() {
        let html = Html::from_string("<h1>Hello</h1>".to_string());
        assert!(matches!(html.0, Cow::Owned(_)));
    }

    #[test]
    fn test_status_helpers() {
        let res = status::ok();
        assert_eq!(res.status_code().as_u16(), 200);

        let res = status::not_found();
        assert_eq!(res.status_code().as_u16(), 404);
    }
}
