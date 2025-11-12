//! Application state extractor

use std::sync::Arc;

/// Application state extractor
pub struct State<T>(pub Arc<T>);

impl<T> State<T> {
    /// Extract state
    pub fn extract(state: Arc<T>) -> Self {
        State(state)
    }
}

impl<T> Clone for State<T> {
    fn clone(&self) -> Self {
        State(Arc::clone(&self.0))
    }
}
