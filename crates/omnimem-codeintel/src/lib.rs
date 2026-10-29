//! Code intelligence for OmniMem.
//!
//! Provides tree-sitter-based parsing of source files into a code graph
//! (files, folders, classes, methods, functions), scope-resolved CALLS edges,
//! N-hop impact analysis, community detection, and audit passes (dead code,
//! naming conventions, security scan).

pub mod api;
pub mod audit;
pub mod codelink;
pub mod community;
pub mod error;
pub mod graph;
pub mod knowledge;
pub mod parser;
pub mod watcher;
pub mod workspace;

pub use api::{ApiEndpoint, ApiService, service_lookup};
pub use audit::{DeadCodeFinding, SecurityFinding, dead_code, security_scan};
pub use codelink::{CodeLink, build_links, find_referenced_symbols};
pub use community::{CommunityMap, detect_communities};
pub use error::{CodeIntelError, Result};
pub use graph::CodeGraph;
pub use knowledge::{DocSection, KnowledgeBase};
pub use parser::{Symbol, SymbolKind, language_for_path, parse_symbols};
pub use watcher::{Watcher, auto_watch_enabled, set_auto_watch};
pub use workspace::Workspace;

use std::path::Path;

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
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Build the code graph.
    pub fn build(&self) -> Result<CodeGraph> {
        CodeGraph::build(&self.root)
    }

    /// Run impact analysis for `symbol_name`.
    pub fn impact(&self, symbol_name: &str) -> Vec<std::path::PathBuf> {
        let graph = match self.build() {
            Ok(g) => g,
            Err(_) => return Vec::new(),
        };
        graph.impact(symbol_name, &self.root)
    }

    /// Run dead-code audit.
    pub fn dead_code(&self) -> Vec<DeadCodeFinding> {
        let graph = match self.build() {
            Ok(g) => g,
            Err(_) => return Vec::new(),
        };
        audit::dead_code(&graph, &self.root)
    }

    /// Run security scan.
    pub fn security_scan(&self) -> Vec<SecurityFinding> {
        audit::security_scan(&self.root)
    }

    /// Detect communities in the code graph.
    pub fn communities(&self) -> CommunityMap {
        let graph = match self.build() {
            Ok(g) => g,
            Err(_) => return CommunityMap::new(),
        };
        community::detect_communities(&graph)
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

    #[test]
    fn parse_rust_symbols() {
        let src = "fn foo() {}\nstruct Bar {}\nimpl Bar { fn method(&self) {} }\n";
        let path = std::path::Path::new("test.rs");
        let syms = parse_symbols(path, src).unwrap();
        let names: Vec<&str> = syms.iter().map(|s| s.name.as_str()).collect();
        assert!(names.contains(&"foo"));
        assert!(names.contains(&"Bar"));
    }
}