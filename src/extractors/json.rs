//! JSON body extractor

use serde::de::DeserializeOwned;

/// JSON body extractor
pub struct Json<T>(pub T);

impl<T: DeserializeOwned> Json<T> {
    /// Extract JSON from request body
    pub async fn extract(body: &[u8]) -> Result<Self, serde_json::Error> {
        let value = serde_json::from_slice(body)?;
        Ok(Json(value))
    }
}
