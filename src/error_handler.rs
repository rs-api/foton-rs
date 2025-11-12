//! Error handler trait.

use crate::{Error, Res};

/// Convert errors to responses.
pub trait ErrorHandler: Send + Sync + 'static {
    /// Handle error.
    fn handle(&self, error: Error) -> Res;
}
