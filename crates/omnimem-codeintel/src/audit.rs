//! Audit passes: dead code, naming conventions, security scan.

use crate::graph::CodeGraph;
use crate::parser::SymbolKind;
use std::path::Path;

/// A dead-code finding.
#[derive(Debug, Clone)]
pub struct DeadCodeFinding {
    /// File containing the symbol.
    pub file: std::path::PathBuf,
    /// Symbol name.
    pub symbol: String,
    /// Start line.
    pub line: usize,
}

/// A security finding.
#[derive(Debug, Clone)]
pub struct SecurityFinding {
    /// File containing the issue.
    pub file: std::path::PathBuf,
    /// Line number.
    pub line: usize,
    /// Description.
    pub description: String,
}

/// Detect symbols that are never referenced by name in any other file
/// (best-effort, textual). Entry-point-like names are excluded.
pub fn dead_code(graph: &CodeGraph, root: &Path) -> Vec<DeadCodeFinding> {
    let mut findings = Vec::new();
    // Collect all source text for reference checking.
    let mut all_text = String::new();
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        if let Ok(s) = std::fs::read_to_string(entry.path()) {
            all_text.push_str(&s);
            all_text.push('\n');
        }
    }
    for (file, syms) in &graph.files {
        for sym in syms {
            if matches!(sym.kind, SymbolKind::Function) && !is_entry_point(&sym.name) {
                // count occurrences of the name across the whole codebase
                let count = all_text.matches(&sym.name).count();
                if count <= 1 {
                    findings.push(DeadCodeFinding {
                        file: file.clone(),
                        symbol: sym.name.clone(),
                        line: sym.start_line,
                    });
                }
            }
        }
    }
    findings
}

fn is_entry_point(name: &str) -> bool {
    matches!(
        name,
        "main" | "run" | "start" | "init" | "setup" | "default" | "new"
    )
}

/// Scan for hardcoded secrets (tokens, keys, passwords).
pub fn security_scan(root: &Path) -> Vec<SecurityFinding> {
    let mut findings = Vec::new();
    let patterns: &[(&str, &str)] = &[
        ("AWS access key", r"AKIA[0-9A-Z]{16}"),
        ("GitHub token", r"ghp_[A-Za-z0-9]{36}"),
        ("Generic API key", r#"(?i)(api[_-]?key|secret|password)\s*[:=]\s*["'][^"']{8,}["']"#),
        ("Private key", r"-----BEGIN (RSA |EC |OPENSSH )?PRIVATE KEY-----"),
    ];
    let compiled: Vec<(&str, regex::Regex)> = patterns
        .iter()
        .filter_map(|(desc, pat)| regex::Regex::new(pat).ok().map(|r| (*desc, r)))
        .collect();
    for entry in walkdir::WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("").to_lowercase();
        let allowed = matches!(ext.as_str(), "rs" | "py" | "ts" | "js" | "go" | "toml" | "env");
        if !allowed {
            continue;
        }
        let content = match std::fs::read_to_string(path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        for (i, line) in content.lines().enumerate() {
            for (desc, re) in &compiled {
                if re.is_match(line) {
                    findings.push(SecurityFinding {
                        file: path.to_path_buf(),
                        line: i + 1,
                        description: desc.to_string(),
                    });
                }
            }
        }
    }
    findings
}