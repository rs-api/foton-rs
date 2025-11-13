//! Type-safe request extractors.

use crate::{Error, Req, Result};
use async_trait::async_trait;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::sync::Arc;

/// Extract data from request.
#[async_trait]
pub trait FromRequest<S = ()>: Sized {
    /// Extract from request.
    async fn from_request(req: &mut Req, state: &Arc<S>) -> Result<Self>;
}

/// Application state extractor.
pub struct State<S>(pub S);

#[async_trait]
impl<S> FromRequest<S> for State<S>
where
    S: Clone + Send + Sync + 'static,
{
    #[inline]
    async fn from_request(_req: &mut Req, state: &Arc<S>) -> Result<Self> {
        Ok(State((**state).clone()))
    }
}

/// Query parameters extractor.
pub struct Query<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Query<T>
where
    T: DeserializeOwned,
    S: Send + Sync + 'static,
{
    #[inline]
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

/// Form data extractor.
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

        let body = req.body().await?;
        let value = serde_urlencoded::from_bytes::<T>(body.as_ref())
            .map_err(|e| Error::unprocessable(format!("Invalid form data: {}", e)))?;

        Ok(Form(value))
    }
}

/// JSON request body extractor.
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

        let body = req.body().await?;
        let value = serde_json::from_slice(body)
            .map_err(|e| Error::bad_request(format!("Invalid JSON: {}", e)))?;

        Ok(Json(value))
    }
}

/// Path parameters extractor (deserializes HashMap directly).
pub struct Path<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for Path<T>
where
    T: DeserializeOwned,
    S: Send + Sync + 'static,
{
    #[inline]
    async fn from_request(req: &mut Req, _state: &Arc<S>) -> Result<Self> {
        let params = req.path_params();
        let value = deserialize_path_params(params).map_err(|e| {
            Error::bad_request(format!(
                "Invalid path parameters: {}. Use String type for path segments",
                e
            ))
        })?;

        Ok(Path(value))
    }
}

/// Deserialize HashMap<String, String> directly to T.
fn deserialize_path_params<T: DeserializeOwned>(
    params: &HashMap<String, String>,
) -> std::result::Result<T, serde::de::value::Error> {
    use serde::de::value::MapDeserializer;
    let deserializer = MapDeserializer::new(params.iter().map(|(k, v)| (k.as_str(), v.as_str())));
    T::deserialize(deserializer)
}

/// Headers extractor.
pub struct Headers(pub hyper::HeaderMap);

#[async_trait]
impl<S> FromRequest<S> for Headers
where
    S: Send + Sync + 'static,
{
    #[inline]
    async fn from_request(req: &mut Req, _state: &Arc<S>) -> Result<Self> {
        Ok(Headers(req.headers().clone()))
    }
}

/// Raw body bytes extractor.
pub struct BodyBytes(pub bytes::Bytes);

#[async_trait]
impl<S> FromRequest<S> for BodyBytes
where
    S: Send + Sync + 'static,
{
    #[inline]
    async fn from_request(req: &mut Req, _state: &Arc<S>) -> Result<Self> {
        let body = req.body().await?;
        Ok(BodyBytes(body.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_path_deserialize() {
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct Params {
            id: String,
            name: String,
        }

        let mut map = HashMap::new();
        map.insert("id".to_string(), "123".to_string());
        map.insert("name".to_string(), "alice".to_string());

        let result: Params = deserialize_path_params(&map).unwrap();
        assert_eq!(result.id, "123");
        assert_eq!(result.name, "alice");
    }

    #[test]
    fn test_path_deserialize_numbers() {
        #[derive(serde::Deserialize, Debug, PartialEq)]
        struct Params {
            id: String,
        }

        let mut map = HashMap::new();
        map.insert("id".to_string(), "456".to_string());

        let result: Params = deserialize_path_params(&map).unwrap();
        assert_eq!(result.id, "456");
    }
}
