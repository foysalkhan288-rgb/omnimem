# OmniMem

**Git for AI Agent Knowledge** — a local-first, offline, single-binary agent memory and code-intelligence tool.

OmniMem gives your AI coding agents (Claude Code, Cursor, Codex, Gemini CLI, OpenCode, and 13 more) a persistent, versioned, queryable knowledge graph that survives across sessions — with zero LLM cost at ingest.

## Quickstart

```bash
# Install (cargo)
cargo install --locked omnimem-cli --features bundled-embedder

# Initialize in your project
cd your-project
omnimem init

# Ingest a codebase / docs / PDFs
omnimem ingest .

# Retrieve
omnimem retrieve "what did we decide about the API rate limit?"

# Branch knowledge like code
omnimem branch create experiment
omnimem merge experiment
```

## Features (v1)

- Git primitives on a knowledge graph: branch, diff, merge, revert, rollback, 3-way merge
- Deterministic, $0 LLM ingestion (AST / heading / sliding-window parsing)
- Single ~40–80 MB native binary, fully offline, no external database
- Hybrid retrieval (vector + BM25 + graph) with explicit token-budget transparency
- Auto-capture via lifecycle hooks, secret scrubbing, offline replay
- Tree-sitter code graph with impact analysis (N-hop blast radius)
- Real-time viewer + session replay
- Bi-temporal facts, grounded abstain, privacy receipts
- Cross-agent shared memory (18+ agents) with auto-setup
- Apache-2.0, local-first, `.mnem/` committed alongside code

## Documentation

- [Quickstart](docs/src/quickstart.md)
- [Core Concepts](docs/src/core-concepts.md)
- [Configuration](docs/src/configuration.md)
- [Architecture](docs/src/architecture.md)
- Full spec: [OMNIMEM_PLAN.md](OMNIMEM_PLAN.md)
- Tasklist: [OMNIMEM_TASKLIST.md](OMNIMEM_TASKLIST.md)

## License

Apache-2.0. See [LICENSE](LICENSE).