//! Error type for the code-intelligence subsystem.

use thiserror::Error;

/// Errors produced by the code-intelligence subsystem.
#[derive(Debug, Error)]
pub enum CodeIntelError {
    /// A source file could not be parsed.
    #[error("parse error: {0}")]
    Parse(String),
    /// An I/O error occurred.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    /// An underlying OmniMem core error.
    #[error("core error: {0}")]
    Core(String),
    /// An unsupported language.
    #[error("unsupported language: {0}")]
    UnsupportedLanguage(String),
}

/// Result type for code-intelligence operations.
pub type Result<T> = std::result::Result<T, CodeIntelError>;