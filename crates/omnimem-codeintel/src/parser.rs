//! Tree-sitter based source parsing.

use crate::error::{CodeIntelError, Result};
use std::path::Path;

/// Kind of a code symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    /// A free function.
    Function,
    /// A method inside a class/impl.
    Method,
    /// A class / struct / interface.
    Class,
    /// A module.
    Module,
}

/// A symbol extracted from source code.
#[derive(Debug, Clone)]
pub struct Symbol {
    /// Symbol name.
    pub name: String,
    /// Symbol kind.
    pub kind: SymbolKind,
    /// 1-based start line.
    pub start_line: usize,
    /// 1-based end line.
    pub end_line: usize,
}

/// Detect the language from a file path.
pub fn language_for_path(path: &Path) -> Option<&'static str> {
    let ext = path.extension().and_then(|e| e.to_str())?.to_lowercase();
    match ext.as_str() {
        "rs" => Some("rust"),
        "py" => Some("python"),
        "ts" | "tsx" => Some("typescript"),
        "go" => Some("go"),
        "js" | "jsx" => Some("javascript"),
        _ => None,
    }
}

/// Parse `source` and extract top-level symbols.
pub fn parse_symbols(path: &Path, source: &str) -> Result<Vec<Symbol>> {
    let lang = language_for_path(path)
        .ok_or_else(|| CodeIntelError::UnsupportedLanguage(path.display().to_string()))?;
    match lang {
        "rust" => parse_rust(source),
        "python" => parse_python(source),
        "typescript" | "javascript" => parse_typescript(source),
        "go" => parse_go(source),
        _ => Err(CodeIntelError::UnsupportedLanguage(lang.to_string())),
    }
}

fn parse_rust(source: &str) -> Result<Vec<Symbol>> {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_rust::LANGUAGE;
    parser
        .set_language(&language.into())
        .map_err(|e| CodeIntelError::Parse(e.to_string()))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| CodeIntelError::Parse("parse returned None".to_string()))?;
    let mut out = Vec::new();
    collect_rust(&tree.root_node(), source, &mut out);
    Ok(out)
}

fn collect_rust<'a>(node: &tree_sitter::Node<'a>, source: &str, out: &mut Vec<Symbol>) {
    let kind = node.kind();
    if kind == "function_item" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            out.push(Symbol {
                name,
                kind: SymbolKind::Function,
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
    } else if kind == "struct_item" || kind == "enum_item" || kind == "trait_item" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            out.push(Symbol {
                name,
                kind: SymbolKind::Class,
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_rust(&child, source, out);
    }
}

fn parse_python(source: &str) -> Result<Vec<Symbol>> {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_python::LANGUAGE;
    parser
        .set_language(&language.into())
        .map_err(|e| CodeIntelError::Parse(e.to_string()))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| CodeIntelError::Parse("parse returned None".to_string()))?;
    let mut out = Vec::new();
    collect_python(&tree.root_node(), source, &mut out);
    Ok(out)
}

fn collect_python<'a>(node: &tree_sitter::Node<'a>, source: &str, out: &mut Vec<Symbol>) {
    let kind = node.kind();
    if kind == "function_definition" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            out.push(Symbol {
                name,
                kind: SymbolKind::Function,
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
    } else if kind == "class_definition" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            out.push(Symbol {
                name,
                kind: SymbolKind::Class,
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_python(&child, source, out);
    }
}

fn parse_typescript(source: &str) -> Result<Vec<Symbol>> {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_typescript::LANGUAGE_TYPESCRIPT;
    parser
        .set_language(&language.into())
        .map_err(|e| CodeIntelError::Parse(e.to_string()))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| CodeIntelError::Parse("parse returned None".to_string()))?;
    let mut out = Vec::new();
    collect_ts(&tree.root_node(), source, &mut out);
    Ok(out)
}

fn collect_ts<'a>(node: &tree_sitter::Node<'a>, source: &str, out: &mut Vec<Symbol>) {
    let kind = node.kind();
    if kind == "function_declaration" || kind == "method_definition" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            out.push(Symbol {
                name,
                kind: if kind == "method_definition" {
                    SymbolKind::Method
                } else {
                    SymbolKind::Function
                },
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
    } else if kind == "class_declaration" || kind == "interface_declaration" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            out.push(Symbol {
                name,
                kind: SymbolKind::Class,
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_ts(&child, source, out);
    }
}

fn parse_go(source: &str) -> Result<Vec<Symbol>> {
    let mut parser = tree_sitter::Parser::new();
    let language = tree_sitter_go::LANGUAGE;
    parser
        .set_language(&language.into())
        .map_err(|e| CodeIntelError::Parse(e.to_string()))?;
    let tree = parser
        .parse(source, None)
        .ok_or_else(|| CodeIntelError::Parse("parse returned None".to_string()))?;
    let mut out = Vec::new();
    collect_go(&tree.root_node(), source, &mut out);
    Ok(out)
}

fn collect_go<'a>(node: &tree_sitter::Node<'a>, source: &str, out: &mut Vec<Symbol>) {
    let kind = node.kind();
    if kind == "function_declaration" || kind == "method_declaration" {
        if let Some(name_node) = node.child_by_field_name("name") {
            let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
            out.push(Symbol {
                name,
                kind: if kind == "method_declaration" {
                    SymbolKind::Method
                } else {
                    SymbolKind::Function
                },
                start_line: node.start_position().row + 1,
                end_line: node.end_position().row + 1,
            });
        }
    } else if kind == "type_declaration" {
        // best-effort: capture type name
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            if child.kind() == "type_spec" {
                if let Some(name_node) = child.child_by_field_name("name") {
                    let name = name_node.utf8_text(source.as_bytes()).unwrap_or("").to_string();
                    out.push(Symbol {
                        name,
                        kind: SymbolKind::Class,
                        start_line: child.start_position().row + 1,
                        end_line: child.end_position().row + 1,
                    });
                }
            }
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_go(&child, source, out);
    }
}
