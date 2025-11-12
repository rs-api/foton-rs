//! Path parameter extractor

use serde::de::DeserializeOwned;

/// Path parameter extractor
pub struct Path<T>(pub T);

impl<T: DeserializeOwned> Path<T> {
    /// Extract path parameters
    pub fn extract(
        _params: &std::collections::HashMap<String, String>,
    ) -> Result<Self, serde_json::Error> {
        // TODO: Implement path param extraction
        todo!("Path extraction not yet implemented")
    }
}
