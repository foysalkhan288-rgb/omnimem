# Architecture

## Overview

OmniMem is a single Rust binary with a workspace of focused crates. All state lives in a content-addressed store (redb) under `.omnimem/` (project) or `~/.omnimem/` (global).

## Crate Layout

| Crate | Responsibility |
|---|---|
| `omnimem-core` | Node/edge model, commit DAG, content-addressing (BLAKE3), branch/merge, tombstone/supersession, bi-temporal facts, named stores |
| `omnimem-backend-redb` | ACID single-writer store, multi-reader |
| `omnimem-ann` | In-process HNSW vector index |
| `omnimem-sparse-providers` | BM25 / SPLADE |
| `omnimem-embed-providers` | ONNX MiniLM default; pluggable Ollama/OpenAI/Cohere |
| `omnimem-ingest` | File-type detection, MD heading chunker, AST code (tree-sitter), PDF sliding window, deterministic, cached |
| `omnimem-extract` | Entity/relation heuristics (rule-based NER, no LLM) |
| `omnimem-graphrag` | Hybrid RRF retrieval, multi-hop expansion, token-budget packing, grounded abstain |
| `omnimem-lifecycle` | Decay, access-count boosting, GC, consolidation, conflict detection, privacy receipts |
| `omnimem-codeintel` | Tree-sitter code graph, impact analysis, community detection, audit (Phase 5, in progress) |
| `omnimem-mcp` / `omnimem-http` / `omnimem-cli` / `omnimem-transport` | Surfaces |
| `omnimem-hooks` | Auto-capture installers for 18+ agents (Phase 6) |
| `omnimem-viewer` | Real-time UI + session replay (Phase 6) |
| `omnimem-config` | Configuration loading |
| `omnimem-py` / `omnimem-wasm` | Optional bindings |

## Storage

- `.omnimem/` in project root (committed alongside code), content-addressed.
- `~/.omnimem/` for global/cross-project store and viewer state.
- Single-writer lock (redb) for correctness; unlimited readers.

## Surfaces

- **CLI**: `omnimem init|ingest|retrieve|add|branch|merge|diff|revert|tombstone|...`
- **MCP stdio server**: 54+ tools, profiles micro/lite/team/full
- **HTTP server + viewer + session replay**
- **Auto-setup**: `omnimem setup --agent <name> --global`

See [OMNIMEM_PLAN.md](../OMNIMEM_PLAN.md) for the full specification.