//! Server configuration.

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

use crate::{Error, Result};

/// Server configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// Maximum request body size in bytes.
    pub body_limit: Option<usize>,

    /// Request timeout in seconds.
    #[serde(default, with = "opt_duration_serde")]
    pub request_timeout: Option<Duration>,

    /// Handler execution timeout in seconds.
    #[serde(default, with = "opt_duration_serde")]
    pub handler_timeout: Option<Duration>,

    /// Enable HTTP/2 support.
    #[serde(default)]
    pub http2: bool,

    /// Maximum number of concurrent connections.
    pub max_connections: Option<usize>,

    /// TCP keep-alive duration in seconds.
    #[serde(default, with = "opt_duration_serde")]
    pub keep_alive: Option<Duration>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            body_limit: None,
            request_timeout: None,
            handler_timeout: None,
            http2: false,
            max_connections: None,
            keep_alive: None,
        }
    }
}

impl ServerConfig {
    /// Create a new empty configuration.
    pub fn new() -> Self {
        Self::default()
    }

    /// Load configuration from a TOML file.
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self> {
        let contents = std::fs::read_to_string(path.as_ref())
            .map_err(|e| Error::Custom(format!("Failed to read config file: {}", e)))?;

        toml::from_str(&contents)
            .map_err(|e| Error::Custom(format!("Failed to parse config file: {}", e)))
    }
}

mod opt_duration_serde {
    use serde::{Deserialize, Deserializer, Serializer};
    use std::time::Duration;

    pub fn serialize<S>(duration: &Option<Duration>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match duration {
            Some(d) => serializer.serialize_some(&d.as_secs()),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Duration>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let secs: Option<u64> = Option::deserialize(deserializer)?;
        Ok(secs.map(Duration::from_secs))
    }
}
