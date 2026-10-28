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
/// code graph's symbol names. Uses word boundaries to avoid false
/// positives from substring matches.
pub fn find_referenced_symbols(graph: &CodeGraph, text: &str) -> Vec<(String, PathBuf)> {
    let mut out = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    for (file, syms) in &graph.files {
        for sym in syms {
            if sym.name.len() < 3 || seen.contains(&sym.name) {
                continue;
            }
            if is_word_boundary_match(text, &sym.name) {
                seen.insert(sym.name.clone());
                out.push((sym.name.clone(), file.clone()));
            }
        }
    }
    out
}

fn is_word_boundary_match(text: &str, needle: &str) -> bool {
    let mut start = 0;
    while let Some(pos) = text[start..].find(needle) {
        let abs = start + pos;
        let before_ok = abs == 0
            || (!text.as_bytes()[abs - 1].is_ascii_alphanumeric()
                && text.as_bytes()[abs - 1] != b'_');
        let after = abs + needle.len();
        let after_ok = after >= text.len()
            || (!text.as_bytes()[after].is_ascii_alphanumeric()
                && text.as_bytes()[after] != b'_');
        if before_ok && after_ok {
            return true;
        }
        start = abs + 1;
    }
    false
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