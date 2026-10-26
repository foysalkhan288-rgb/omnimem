//! Tool profiles: micro / lite / team / full.

/// A tool profile tier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Profile {
    /// Minimal surface (9 tools): core read/write.
    Micro,
    /// Lite (21 tools): + search, traverse, schema.
    Lite,
    /// Team (29 tools): + ingest, tombstone, branch, merge.
    Team,
    /// Full surface (all tools).
    Full,
}

impl Profile {
    /// Parse from a string.
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "micro" => Some(Self::Micro),
            "lite" => Some(Self::Lite),
            "team" => Some(Self::Team),
            "full" => Some(Self::Full),
            _ => None,
        }
    }

    /// Tool names included at this profile tier (cumulative).
    pub fn tool_names(&self) -> &'static [&'static str] {
        match self {
            Self::Micro => &[
                "omnimem_stats",
                "omnimem_search",
                "omnimem_retrieve",
                "omnimem_get_node",
                "omnimem_commit",
                "omnimem_resolve_or_create",
                "omnimem_tombstone_node",
                "omnimem_list_nodes",
                "omnimem_recent",
            ],
            Self::Lite => &[
                // micro +
                "omnimem_stats",
                "omnimem_search",
                "omnimem_retrieve",
                "omnimem_get_node",
                "omnimem_commit",
                "omnimem_resolve_or_create",
                "omnimem_tombstone_node",
                "omnimem_list_nodes",
                "omnimem_recent",
                "omnimem_traverse",
                "omnimem_schema",
                "omnimem_incoming_edges",
                "omnimem_vector_search",
                "omnimem_list_tags",
                "omnimem_get_code_snippet",
                "omnimem_search_code",
                "omnimem_search_graph",
                "omnimem_trace_path",
                "omnimem_detect_changes",
                "omnimem_get_architecture",
                "omnimem_ingest",
            ],
            Self::Team => &[
                "omnimem_stats",
                "omnimem_search",
                "omnimem_retrieve",
                "omnimem_get_node",
                "omnimem_commit",
                "omnimem_resolve_or_create",
                "omnimem_tombstone_node",
                "omnimem_list_nodes",
                "omnimem_recent",
                "omnimem_traverse",
                "omnimem_schema",
                "omnimem_incoming_edges",
                "omnimem_vector_search",
                "omnimem_list_tags",
                "omnimem_get_code_snippet",
                "omnimem_search_code",
                "omnimem_search_graph",
                "omnimem_trace_path",
                "omnimem_detect_changes",
                "omnimem_get_architecture",
                "omnimem_ingest",
                "omnimem_commit_relation",
                "omnimem_delete_node",
                "omnimem_global_add",
                "omnimem_global_ingest",
                "omnimem_global_retrieve",
                "omnimem_global_tombstone_node",
                "omnimem_community_summarize",
                "omnimem_manage_adr",
            ],
            Self::Full => &[],
        }
    }

    /// Whether a tool name is included at this profile tier.
    pub fn includes(&self, name: &str) -> bool {
        match self {
            Self::Full => true,
            _ => self.tool_names().contains(&name),
        }
    }
}