//! Query string extractor

use serde::de::DeserializeOwned;

/// Query string extractor
pub struct Query<T>(pub T);

impl<T: DeserializeOwned> Query<T> {
    /// Extract query parameters
    pub fn extract(_query: &str) -> Result<Self, serde_json::Error> {
        // TODO: Implement query string parsing
        todo!("Query extraction not yet implemented")
    }
}
