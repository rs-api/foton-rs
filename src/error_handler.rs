//! Error handler trait.

use crate::{Error, Res};

/// Convert errors to HTTP responses.
pub trait ErrorHandler: Send + Sync + 'static {
    /// Handle error and return response.
    fn handle(&self, error: Error) -> Res;
}
