use serde::{Deserialize, Serialize};

mod error;
pub use error::ConfigError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Storage {
    pub path: String,
}

impl Default for Storage {
    fn default() -> Self {
        Self { path: "./data".into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Embed {
    pub provider: String,
    pub model: String,
}

impl Default for Embed {
    fn default() -> Self {
        Self { provider: "local".into(), model: "default".into() }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingest {
    pub chunk_size: usize,
}

impl Default for Ingest {
    fn default() -> Self {
        Self { chunk_size: 512 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Retrieval {
    pub limit: usize,
}

impl Default for Retrieval {
    fn default() -> Self {
        Self { limit: 10 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lifecycle {
    pub retention_days: u64,
}

impl Default for Lifecycle {
    fn default() -> Self {
        Self { retention_days: 0 }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Viewer {
    pub port: u16,
}

impl Default for Viewer {
    fn default() -> Self {
        Self { port: 8080 }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    pub storage: Storage,
    pub embed: Embed,
    pub ingest: Ingest,
    pub retrieval: Retrieval,
    pub lifecycle: Lifecycle,
    pub viewer: Viewer,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        Ok(Self::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config() {
        let c = Config::default();
        assert_eq!(c.storage.path, "./data");
        assert_eq!(c.viewer.port, 8080);
    }
}