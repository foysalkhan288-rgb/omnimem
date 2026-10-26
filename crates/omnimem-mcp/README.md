# omnimem mcp

Model Context Protocol server for omnimem - the AI-native, local-first memory substrate for agents.

Ships the `omnimem mcp` binary, which speaks MCP JSON-RPC 2.0 over stdio. Any
MCP-aware host (Claude Desktop, Cursor, Windsurf, Claude Code, custom
clients) can point at it and omnimem's tools show up: `omnimem_stats`,
`omnimem_schema`, `omnimem_search`, `omnimem_vector_search`, `omnimem_retrieve`,
`omnimem_get_node`, `omnimem_traverse`, `omnimem_commit`, `omnimem_resolve_or_create`,
`omnimem_recent`. Every response carries `_meta` with `bytes`,
`latency_micros`, and `tokens_estimate` so the caller can reason about the
cost of its own calls; writes propagate `agent_id` and `task_id` into commit
and operation metadata so provenance stays queryable.

```bash
omnimem mcp --repo ./my-omnimem-repo
# or
omnimem_REPO=./my-omnimem-repo omnimem mcp
```

Workspace top: [`../../README.md`](../../README.md). MCP guide:
[`../../docs/guide/mcp.md`](../../docs/guide/mcp.md).

Licensed under Apache-2.0.
