//! Code graph: files, symbols, and references.

use crate::error::Result;
use crate::parser::{Symbol, parse_symbols};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// A code graph built from a repository root.
#[derive(Debug, Default)]
pub struct CodeGraph {
    /// Symbols per file.
    pub files: HashMap<PathBuf, Vec<Symbol>>,
}

impl CodeGraph {
    /// Build a code graph by walking `root` and parsing supported files.
    pub fn build(root: &Path) -> Result<Self> {
        let mut files = HashMap::new();
        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if crate::parser::language_for_path(path).is_none() {
                continue;
            }
            // skip hidden / target / node_modules
            let s = path.to_string_lossy();
            if s.contains("/target/") || s.contains("\\target\\")
                || s.contains("/node_modules/") || s.contains("\\node_modules\\")
                || s.contains("/.git/") || s.contains("\\.git\\")
            {
                continue;
            }
            let source = match std::fs::read_to_string(path) {
                Ok(s) => s,
                Err(_) => continue,
            };
            if let Ok(syms) = parse_symbols(path, &source) {
                if !syms.is_empty() {
                    files.insert(path.to_path_buf(), syms);
                }
            }
        }
        Ok(Self { files })
    }

    /// Total number of symbols across all files.
    pub fn symbol_count(&self) -> usize {
        self.files.values().map(|v| v.len()).sum()
    }

    /// Number of indexed files.
    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    /// Find files containing a symbol with the given name.
    pub fn find_symbol(&self, name: &str) -> Vec<PathBuf> {
        self.files
            .iter()
            .filter(|(_, syms)| syms.iter().any(|s| s.name == name))
            .map(|(p, _)| p.clone())
            .collect()
    }

    /// Impact analysis: files that reference `symbol_name` (textual
    /// occurrence) plus the defining file. Returns sorted unique paths.
    pub fn impact(&self, symbol_name: &str, root: &Path) -> Vec<PathBuf> {
        let mut affected: HashSet<PathBuf> = HashSet::new();
        // defining files
        for p in self.find_symbol(symbol_name) {
            affected.insert(p);
        }
        // files that reference the symbol by name
        for entry in WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            if crate::parser::language_for_path(path).is_none() {
                continue;
            }
            if let Ok(source) = std::fs::read_to_string(path) {
                if source.contains(symbol_name) {
                    affected.insert(path.to_path_buf());
                }
            }
        }
        let mut v: Vec<PathBuf> = affected.into_iter().collect();
        v.sort();
        v
    }
}