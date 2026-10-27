//! API discovery: parse OpenAPI specs and extract endpoints.

use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

/// An API endpoint extracted from an OpenAPI spec.
#[derive(Debug, Clone)]
pub struct ApiEndpoint {
    /// HTTP method (GET, POST, etc.).
    pub method: String,
    /// Path template (e.g. /users/{id}).
    pub path: String,
    /// Operation id if present.
    pub operation_id: Option<String>,
    /// Summary if present.
    pub summary: Option<String>,
}

/// An API surface extracted from a spec.
#[derive(Debug, Default)]
pub struct ApiService {
    /// Title of the API.
    pub title: Option<String>,
    /// Version.
    pub version: Option<String>,
    /// Endpoints.
    pub endpoints: Vec<ApiEndpoint>,
}

impl ApiService {
    /// Parse an OpenAPI JSON file.
    pub fn from_openapi_json(path: &Path) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        let value: serde_json::Value = serde_json::from_str(&content).ok()?;
        let mut svc = ApiService::default();
        if let Some(info) = value.get("info") {
            svc.title = info.get("title").and_then(|v| v.as_str()).map(String::from);
            svc.version = info.get("version").and_then(|v| v.as_str()).map(String::from);
        }
        if let Some(paths) = value.get("paths").and_then(|v| v.as_object()) {
            for (path, methods) in paths {
                if let Some(methods_obj) = methods.as_object() {
                    for (method, op) in methods_obj {
                        if method.starts_with('x') {
                            continue;
                        }
                        let method = method.to_uppercase();
                        if !matches!(method.as_str(), "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS") {
                            continue;
                        }
                        let operation_id = op.get("operationId").and_then(|v| v.as_str()).map(String::from);
                        let summary = op.get("summary").and_then(|v| v.as_str()).map(String::from);
                        svc.endpoints.push(ApiEndpoint {
                            method,
                            path: path.clone(),
                            operation_id,
                            summary,
                        });
                    }
                }
            }
        }
        svc.endpoints.sort_by(|a, b| a.path.cmp(&b.path).then(a.method.cmp(&b.method)));
        Some(svc)
    }

    /// Discover OpenAPI specs in a directory (openapi*.json, swagger*.json).
    pub fn discover(root: &Path) -> Vec<(PathBuf, ApiService)> {
        let mut results = Vec::new();
        for entry in walkdir::WalkDir::new(root)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_file())
        {
            let path = entry.path();
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("").to_lowercase();
            if (name.contains("openapi") || name.contains("swagger")) && name.ends_with(".json") {
                if let Some(svc) = Self::from_openapi_json(path) {
                    if !svc.endpoints.is_empty() {
                        results.push((path.to_path_buf(), svc));
                    }
                }
            }
        }
        results
    }

    /// Summary string.
    pub fn summary(&self) -> String {
        let title = self.title.as_deref().unwrap_or("API");
        let version = self.version.as_deref().unwrap_or("?");
        format!("{} v{}: {} endpoints", title, version, self.endpoints.len())
    }
}

/// Service-level lookup: build a summary of all discovered APIs in a root.
pub fn service_lookup(root: &Path) -> Vec<(PathBuf, String)> {
    ApiService::discover(root)
        .into_iter()
        .map(|(p, svc)| (p, svc.summary()))
        .collect()
}