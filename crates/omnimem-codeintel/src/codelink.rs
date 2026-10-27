//! Code <-> memory links.

use crate::graph::CodeGraph;
use std::path::PathBuf;

/// A link between a memory node and a code symbol.
#[derive(Debug, Clone)]
pub struct CodeLink {
    /// Memory node id (opaque string).
    pub memory_id: String,
    /// File containing the symbol.
    pub file: PathBuf,
    /// Symbol name.
    pub symbol: String,
    /// Optional note.
    pub note: Option<String>,
}

/// Find code symbols referenced in `text` by matching against the
/// code graph's symbol names. Returns (symbol_name, file) pairs.
pub fn find_referenced_symbols(graph: &CodeGraph, text: &str) -> Vec<(String, PathBuf)> {
    let mut out = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (file, syms) in &graph.files {
        for sym in syms {
            if sym.name.len() >= 3 && text.contains(&sym.name) && !seen.contains(&sym.name) {
                seen.insert(sym.name.clone());
                out.push((sym.name.clone(), file.clone()));
            }
        }
    }
    out
}

/// Build code links from a text (e.g. a memory node's content) by
/// matching symbol names in the code graph.
pub fn build_links(memory_id: &str, graph: &CodeGraph, text: &str) -> Vec<CodeLink> {
    find_referenced_symbols(graph, text)
        .into_iter()
        .map(|(symbol, file)| CodeLink {
            memory_id: memory_id.to_string(),
            file,
            symbol,
            note: None,
        })
        .collect()
}