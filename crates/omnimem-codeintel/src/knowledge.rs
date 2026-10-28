//! Knowledge base: markdown docs indexed for semantic retrieval.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// A section extracted from a markdown document.
#[derive(Debug, Clone)]
pub struct DocSection {
    /// Source file.
    pub file: PathBuf,
    /// Heading text (empty for preamble).
    pub heading: String,
    /// Heading level (1-6, 0 for preamble).
    pub level: u8,
    /// Section body text.
    pub body: String,
}

/// A knowledge base built from markdown docs in a directory.
#[derive(Debug, Default)]
pub struct KnowledgeBase {
    /// All sections.
    pub sections: Vec<DocSection>,
    /// Sections indexed by file.
    pub by_file: HashMap<PathBuf, Vec<usize>>,
}

impl KnowledgeBase {
    /// Build from a directory, scanning for .md/.mdx/.rst files.
    pub fn build(root: &Path) -> Self {
        let mut kb = Self::default();
        for entry in walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !matches!(ext, "md" | "mdx" | "rst" | "txt") {
                continue;
            }
            // skip hidden/target/node_modules
            let s = path.to_string_lossy();
            if s.contains("/target/") || s.contains("\\target\\")
                || s.contains("/node_modules/") || s.contains("\\node_modules\\")
                || s.contains("/.git/") || s.contains("\\.git\\")
            {
                continue;
            }
            if let Ok(content) = std::fs::read_to_string(path) {
                kb.index_file(path.to_path_buf(), &content);
            }
        }
        kb
    }

    /// Index a single file's content.
    pub fn index_file(&mut self, file: PathBuf, content: &str) {
        let mut current_heading = String::new();
        let mut current_level: u8 = 0;
        let mut body = String::new();
        let start_idx = self.sections.len();

        for line in content.lines() {
            if let Some((level, heading)) = parse_heading(line) {
                // flush previous section
                if !body.trim().is_empty() || !current_heading.is_empty() {
                    self.sections.push(DocSection {
                        file: file.clone(),
                        heading: current_heading.clone(),
                        level: current_level,
                        body: body.trim().to_string(),
                    });
                }
                current_heading = heading;
                current_level = level;
                body.clear();
            } else {
                body.push_str(line);
                body.push('\n');
            }
        }
        // flush last
        if !body.trim().is_empty() || !current_heading.is_empty() {
            self.sections.push(DocSection {
                file: file.clone(),
                heading: current_heading,
                level: current_level,
                body: body.trim().to_string(),
            });
        }

        // record indices
        let indices: Vec<usize> = (start_idx..self.sections.len()).collect();
        if !indices.is_empty() {
            self.by_file.insert(file, indices);
        }
    }

    /// Search sections by keyword (case-insensitive substring).
    pub fn search(&self, query: &str) -> Vec<&DocSection> {
        let q = query.to_lowercase();
        self.sections
            .iter()
            .filter(|s| {
                s.heading.to_lowercase().contains(&q)
                    || s.body.to_lowercase().contains(&q)
            })
            .collect()
    }

    /// Number of sections.
    pub fn section_count(&self) -> usize {
        self.sections.len()
    }

    /// Number of files.
    pub fn file_count(&self) -> usize {
        self.by_file.len()
    }
}

fn parse_heading(line: &str) -> Option<(u8, String)> {
    let trimmed = line.trim_start();
    if !trimmed.starts_with('#') {
        return None;
    }
    let level = trimmed.chars().take_while(|c| *c == '#').count() as u8;
    if level == 0 || level > 6 {
        return None;
    }
    let heading = trimmed[level as usize..].trim().to_string();
    Some((level, heading))
}