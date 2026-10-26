//! Cross-repo workspace linking.

use crate::graph::CodeGraph;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A workspace spanning multiple repository roots.
#[derive(Debug, Default)]
pub struct Workspace {
    /// Registered repo roots.
    pub roots: Vec<PathBuf>,
    /// Cached code graphs per root.
    pub graphs: HashMap<PathBuf, CodeGraph>,
}

impl Workspace {
    /// Create an empty workspace.
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a repository root.
    pub fn add_repo(&mut self, root: impl Into<PathBuf>) {
        let root = root.into();
        if !self.roots.contains(&root) {
            self.roots.push(root);
        }
    }

    /// Build (or rebuild) the code graph for a single root.
    pub fn build_repo(&mut self, root: &Path) -> crate::error::Result<()> {
        let graph = CodeGraph::build(root)?;
        self.graphs.insert(root.to_path_buf(), graph);
        if !self.roots.contains(&root.to_path_buf()) {
            self.roots.push(root.to_path_buf());
        }
        Ok(())
    }

    /// Build graphs for all registered roots.
    pub fn build_all(&mut self) -> crate::error::Result<()> {
        let roots = self.roots.clone();
        for root in &roots {
            self.build_repo(root)?;
        }
        Ok(())
    }

    /// Search for a symbol across all repos.
    pub fn find_symbol(&self, name: &str) -> Vec<(PathBuf, PathBuf)> {
        let mut results = Vec::new();
        for (root, graph) in &self.graphs {
            for file in graph.find_symbol(name) {
                results.push((root.clone(), file));
            }
        }
        results
    }

    /// Impact analysis across all repos: returns (root, affected_file) pairs.
    pub fn impact(&self, symbol_name: &str) -> Vec<(PathBuf, PathBuf)> {
        let mut results = Vec::new();
        for (root, graph) in &self.graphs {
            for file in graph.impact(symbol_name, root) {
                results.push((root.clone(), file));
            }
        }
        results
    }

    /// Total symbol count across all repos.
    pub fn total_symbols(&self) -> usize {
        self.graphs.values().map(|g| g.symbol_count()).sum()
    }

    /// Total file count across all repos.
    pub fn total_files(&self) -> usize {
        self.graphs.values().map(|g| g.file_count()).sum()
    }
}