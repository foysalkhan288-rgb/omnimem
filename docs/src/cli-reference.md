# CLI Reference

## Global flags

- `--cwd <path>` — bind to a specific project directory.
- `--json` — output as JSON for scripting.
- `--store <name>` — use a named store.

## Commands

### omnimem init
Initialize a new `.omnimem/` directory in the current (or specified) project.

### omnimem ingest <path>
Ingest a file or directory. Auto-detects type (Markdown headings, code AST, PDF sliding window).

### omnimem retrieve "<query>"
Hybrid retrieval with token budget. Reports `tokens_used`, `candidates_seen`, `dropped`.

### omnimem search "<pattern>"
Structural/semantic search over the knowledge graph.

### omnimem add node|edge
Add a node or edge manually.

### omnimem branch / merge / diff / revert
Git primitives on the knowledge graph.

### omnimem tombstone <id> "<reason>"
Forget a memory node.

### omnimem resolve <id>
Follow supersession chain to current truth.

### omnimem gc
Garbage collect evicted nodes.

### omnimem conflicts
List detected conflicts.

### omnimem receipts export
Export privacy-safe receipts.

### omnimem distill
Distill transcript into memory proposals.

### omnimem setup --agent <name>
Auto-setup hooks for an agent.

### omnimem doctor
Diagnostics and health check.

### omnimem status / stop / upgrade
Server lifecycle management.

### omnimem schema
Show node labels and edge types.

### omnimem stats
Repository overview.