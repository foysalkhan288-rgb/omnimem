# MCP Tool Reference

OmniMem exposes 50+ tools via the Model Context Protocol (MCP) over stdio.

## Profiles

Tools are grouped into profiles: `micro` (9), `lite` (21), `team` (29), `full` (all). Set via `OMNIMEM_MCP_PROFILE` env var or `--profile` flag.

## Core tools

### omnimem_stats
Repository overview: op-head, head commit, ref summary, known labels.

### omnimem_search
Search nodes by label + property equality. Supports `where`, `with_outgoing`, `limit`.

### omnimem_retrieve
Hybrid retrieval (vector + BM25 + graph) with token budget packing. Returns `tokens_used`, `candidates_seen`, `dropped`.

### omnimem_get_node
Fetch a single node by UUID.

### omnimem_traverse
One-hop outgoing neighbors by edge label.

### omnimem_schema
List node labels, edge types, and index presence.

### omnimem_commit
Commit nodes and/or edges as a single operation.

### omnimem_resolve_or_create
Find-or-create a node by primary key (name/kind or prop_name/value).

### omnimem_commit_relation
Compound write: resolve-or-create subject, resolve-or-create object, connect with typed edge.

### omnimem_tombstone_node
Logically "forget" a node (filters from retrieval, preserves audit trail).

### omnimem_delete_node
Remove a node from the current head.

### omnimem_list_nodes
Enumerate nodes at current head, optionally filtered by label.

### omnimem_recent
Walk the op-log from current head backwards.

### omnimem_list_tags
List all tags in the repository.

### omnimem_ingest
Ingest a source (file path or inline text) as Doc + Chunk + Entity subgraph.

### omnimem_search_code
Graph-augmented code search over indexed files.

### omnimem_search_graph
BM25 full-text search over the knowledge graph.

### omnimem_trace_path
Trace call paths (callers/callees/data flow/cross-service).

### omnimem_detect_changes
Detect code changes and their impact.

### omnimem_get_architecture
High-level architecture overview with community detection.

### omnimem_get_code_snippet
Read source code for a function/class by qualified name.

### omnimem_incoming_edges
List edges pointing TO a node.

### omnimem_vector_search
Cosine-similarity nearest-neighbor search over stored embeddings.

### omnimem_global_add / global_ingest / global_retrieve / global_tombstone_node
Global graph operations (cross-session, cross-project).

### omnimem_community_summarize
MMR summarization over a set of node UUIDs.

### omnimem_manage_adr
Create/update Architecture Decision Records.

### omnimem_ingest_traces
Ingest runtime traces to enhance the knowledge graph.

See `docs/` for full parameter documentation.