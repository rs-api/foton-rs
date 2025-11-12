//! Request extractors
//!
//! Extractors provide type-safe extraction of data from requests.
//! They implement the [`FromRequest`] trait.
//!
//! # Example
//!
//! ```ignore
//! use rust_api::prelude::*;
//! use serde::Deserialize;
//!
//! #[derive(Deserialize)]
//! struct Params {
//!     name: String,
//!     age: u32,
//! }
//!
//! async fn handler(Query(params): Query<Params>) -> Res {
//!     Res::text(format!("Hello {}, age {}", params.name, params.age))
//! }
//! ```

use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::sync::Arc;

use crate::{Error, Req, Result};

/// Extract data from request
#[async_trait]
pub trait FromRequest<S = ()>: Sized {
    /// Extract from request
    async fn from_request(req: &mut Req, state: &Arc<S>) -> Result<Self>;
}

/// Extract query parameters from URL
///
/// # Example
///
/// ```ignore
/// #[derive(Deserialize)]
/// struct SearchParams {
///     q: String,
///     page: Option<u32>,
/// }
///
/// async fn search(Query(params): Query<SearchParams>) -> Res {
///     // Use params.q and params.page
/// }
/// ```
pub struct Query<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync + 'static,
{
    async fn from_request(req: &mut Req, _state: &Arc<S>) -> Result<Self> {
        let query = req
            .uri()
            .query()
            .ok_or_else(|| Error::bad_request("Missing query string"))?;

        let value = serde_urlencoded::from_str::<T>(query)
            .map_err(|e| Error::bad_request(format!("Invalid query parameters: {}", e)))?;

        Ok(Query(value))
    }
}

/// Extract form data from request body
///
/// Content-Type must be `application/x-www-form-urlencoded`.
///
/// # Example
///
/// ```ignore
/// #[derive(Deserialize)]
/// struct LoginForm {
///     username: String,
///     password: String,
/// }
///
/// async fn login(Form(form): Form<LoginForm>) -> Res {
///     // Use form.username and form.password
/// }
/// ```
pub struct Form<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Form<T>
where
    T: DeserializeOwned,
    S: Send + Sync + 'static,
{
    async fn from_request(req: &mut Req, _state: &Arc<S>) -> Result<Self> {
        let content_type = req
            .headers()
            .get(hyper::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.starts_with("application/x-www-form-urlencoded") {
            return Err(Error::bad_request(
                "Content-Type must be application/x-www-form-urlencoded",
            ));
        }

        let body = req.body();
        let value = serde_urlencoded::from_bytes::<T>(body.as_ref())
            .map_err(|e| Error::unprocessable(format!("Invalid form data: {}", e)))?;

        Ok(Form(value))
    }
}

/// Extract JSON from request body
///
/// Content-Type must be `application/json`.
///
/// # Example
///
/// ```ignore
/// #[derive(Deserialize)]
/// struct CreateUser {
///     name: String,
///     email: String,
/// }
///
/// async fn create(Json(user): Json<CreateUser>) -> Res {
///     // Use user.name and user.email
/// }
/// ```
pub struct Json<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync + 'static,
{
    async fn from_request(req: &mut Req, _state: &Arc<S>) -> Result<Self> {
        let content_type = req
            .headers()
            .get(hyper::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.starts_with("application/json") {
            return Err(Error::bad_request("Content-Type must be application/json"));
        }

        let body = req.body();
        let value = serde_json::from_slice(body)
            .map_err(|e| Error::bad_request(format!("Invalid JSON: {}", e)))?;

        Ok(Json(value))
    }
}

/// Extract path parameters
///
/// # Example
///
/// ```ignore
/// #[derive(Deserialize)]
/// struct UserPath {
///     id: u32,
/// }
///
/// async fn get_user(Path(params): Path<UserPath>) -> Res {
///     // Use params.id
/// }
/// ```
pub struct Path<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Path<T>
where
    T: DeserializeOwned,
    S: Send + Sync + 'static,
{
    async fn from_request(req: &mut Req, _state: &Arc<S>) -> Result<Self> {
        let params = req.path_params();

        let value = serde_json::from_value(serde_json::to_value(params)?)
            .map_err(|e| Error::bad_request(format!("Invalid path parameters: {}", e)))?;

        Ok(Path(value))
    }
}

/// Extract application state
///
/// # Example
///
/// ```ignore
/// async fn handler(State(db): State<Database>) -> Res {
///     // Use db
/// }
/// ```
pub struct State<S>(pub S);

#[async_trait]
impl<S> FromRequest<S> for State<S>
where
    S: Clone + Send + Sync + 'static,
{
    async fn from_request(_req: &mut Req, state: &Arc<S>) -> Result<Self> {
        Ok(State((**state).clone()))
    }
}
