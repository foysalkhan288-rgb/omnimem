//! Community detection: label propagation over the code graph.

use crate::graph::CodeGraph;
use std::collections::{HashMap, HashSet};

/// A community assignment: file -> community id.
pub type CommunityMap = HashMap<std::path::PathBuf, usize>;

/// Detect communities using label propagation over file co-occurrence.
/// Files that share symbols (via CALLS/IMPORTS) get the same label.
pub fn detect_communities(graph: &CodeGraph) -> CommunityMap {
    let files: Vec<std::path::PathBuf> = graph.files.keys().cloned().collect();
    if files.is_empty() {
        return CommunityMap::new();
    }
    // Build adjacency: two files are adjacent if they share a symbol name.
    let mut adjacency: HashMap<usize, HashSet<usize>> = HashMap::new();
    for (i, fi) in files.iter().enumerate() {
        let syms_i: HashSet<&str> = graph
            .files
            .get(fi)
            .map(|v| v.iter().map(|s| s.name.as_str()).collect())
            .unwrap_or_default();
        for (j, fj) in files.iter().enumerate() {
            if i == j {
                continue;
            }
            let syms_j: HashSet<&str> = graph
                .files
                .get(fj)
                .map(|v| v.iter().map(|s| s.name.as_str()).collect())
                .unwrap_or_default();
            if !syms_i.is_disjoint(&syms_j) {
                adjacency.entry(i).or_default().insert(j);
            }
        }
    }
    // Label propagation: each node takes the most common label among neighbors.
    let mut labels: Vec<usize> = (0..files.len()).collect();
    for _ in 0..10 {
        let mut changed = false;
        for i in 0..files.len() {
            if let Some(neigh) = adjacency.get(&i) {
                // count labels
                let mut counts: HashMap<usize, usize> = HashMap::new();
                for &n in neigh {
                    *counts.entry(labels[n]).or_insert(0) += 1;
                }
                if let Some((&best, _)) = counts.iter().max_by_key(|(_, c)| *c) {
                    if best != labels[i] {
                        labels[i] = best;
                        changed = true;
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    // Renumber labels to be contiguous 0..n
    let mut remap: HashMap<usize, usize> = HashMap::new();
    let mut next = 0;
    for l in &labels {
        if !remap.contains_key(l) {
            remap.insert(*l, next);
            next += 1;
        }
    }
    let mut result = CommunityMap::new();
    for (i, file) in files.iter().enumerate() {
        result.insert(file.clone(), remap[&labels[i]]);
    }
    result
}