//! Code intelligence for OmniMem.
//!
//! Provides tree-sitter-based parsing of source files into a code graph
//! (files, folders, classes, methods, functions), scope-resolved CALLS edges,
//! N-hop impact analysis, community detection, and audit passes (dead code,
//! naming conventions, security scan).
//!
//! This crate is a skeleton in v1; full implementation lands in Phase 5.

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
}

/// Result type for code-intelligence operations.
pub type Result<T> = std::result::Result<T, CodeIntelError>;

/// High-level entry point for building a code graph from a repository root.
pub struct CodeGraphBuilder {
    root: std::path::PathBuf,
}

impl CodeGraphBuilder {
    /// Create a new builder rooted at `root`.
    pub fn new(root: impl Into<std::path::PathBuf>) -> Self {
        Self { root: root.into() }
    }

    /// Return the repository root this builder targets.
    pub fn root(&self) -> &std::path::Path {
        &self.root
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builder_stores_root() {
        let b = CodeGraphBuilder::new("/tmp/repo");
        assert_eq!(b.root(), std::path::Path::new("/tmp/repo"));
    }
}