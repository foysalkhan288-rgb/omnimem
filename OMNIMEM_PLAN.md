# OmniMem — Product Specification & Implementation Plan

**Version:** 0.1 (Draft for review)
**Status:** Pending approval of open decisions (§10)
**Date:** 2026-10-26
**Document type:** Technical specification + implementation plan
**Working name:** OmniMem (rename TBD)

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Vision & Positioning](#2-vision--positioning)
3. [Competitive Landscape & Deep Analysis](#3-competitive-landscape--deep-analysis)
4. [Feature Inventory (Full Candidate List)](#4-feature-inventory-full-candidate-list)
5. [Feature Selection — v1 Scope](#5-feature-selection--v1-scope)
6. [System Architecture](#6-system-architecture)
7. [Feature-to-Source Traceability Matrix](#7-feature-to-source-traceability-matrix)
8. [Implementation Roadmap](#8-implementation-roadmap)
9. [Technology Stack & Key Decisions](#9-technology-stack--key-decisions)
10. [Open Decisions](#10-open-decisions)
11. [Risk Register](#11-risk-register)
12. [Success Criteria & Acceptance](#12-success-criteria--acceptance)
13. [Appendix A — Glossary](#appendix-a--glossary)
14. [Appendix B — Source Repositories](#appendix-b--source-repositories)
15. [Appendix C — Change Log](#appendix-c--change-log)

---

## 1. Executive Summary

OmniMem is a **local-first, offline, single-binary agent memory and code-intelligence tool** that unifies the unique strengths of ten (10) leading open-source projects in the agent-memory / knowledge-graph space.

It is built on top of `Uranid/mnem` (Git-versioned knowledge graph with deterministic, $0-cost ingestion) and extends it with capabilities drawn from `rohitg00/agentmemory`, `DeusData/codebase-memory-mcp`, `mnemon-dev/mnemon`, `Coding-Dev-Tools/engraphis`, `AVIDS2/memorix`, `Lyellr88/marm-memory`, `agentralabs/agentic-memory`, `Mnemo-mcp/Mnemo`, `basicmachines-co/basic-memory`, and `safishamsi/graphify`.

**Core promise:** *Git for AI Agent Knowledge — extended with auto-capture, deep code intelligence, bi-temporal facts, and grounded retrieval. Fully offline, single binary, $0 ingest.*

**v1 scope (Option A — Recommended Core):** 30 features across storage/versioning, ingestion, retrieval, lifecycle, code intelligence, safety, and multi-agent integration. Heavy/optional capabilities (multimodal vision ingest, IaC indexing, cross-repo intelligence, Smart CI, tiered storage, encryption at rest, multi-device sync, wiki compiler, 3D visualization, orchestration, controlled media, novel cognitive queries) are deferred to v2+ as plug-in modules.

**Non-negotiable identity (inherited from mnem):**
- Single ~40–80 MB native binary, no daemon, no external database, fully offline.
- Deterministic, $0 LLM ingestion (AST / heading / sliding-window parsing). Same input → same graph. Re-ingesting an unchanged file is a no-op.
- Content-addressed storage (BLAKE3 CID) with Git primitives (branch, diff, merge, revert, rollback, 3-way merge).
- Hybrid retrieval (vector + BM25 + graph) with explicit token-budget transparency.
- First-class MCP + CLI + HTTP surfaces.
- Apache-2.0, local-first, `.mnem/` committed alongside code.

---

## 2. Vision & Positioning

### 2.1 Problem Statement

AI coding agents suffer from amnesia across sessions. Existing solutions fall into three camps, each with a critical gap:

1. **Flat files** (`.cursorrules`, `AGENTS.md`, `CLAUDE.md`) — not queryable, not versionable, grow unbounded.
2. **Vector/RAG stores** (Mem0, Letta, Zep) — require LLM calls at ingest ($$$), no Git primitives, no offline single-binary story.
3. **Code-intelligence graphs** (GitNexus, codebase-memory-mcp) — deep on code, but not on conversational/decision memory, and often heavyweight or commercially licensed.

OmniMem closes all three gaps in one product.

### 2.2 Target Users

- **Solo developers** using Claude Code / Cursor / Codex / Gemini CLI / OpenCode who want their agent to remember decisions, conventions, and code structure across sessions — offline, for free.
- **Teams** sharing a versioned knowledge baseline via Git (commit `.mnem/`).
- **Agent builders** (MCP hosts) needing a drop-in, multi-agent memory substrate.

### 2.3 Positioning vs. Alternatives

| Dimension | OmniMem (v1) | mnem | agentmemory | codebase-memory-mcp | memorix |
|---|---|---|---|---|---|
| Single binary, offline | ✅ | ✅ | ❌ (Node+iii) | ✅ (C) | ❌ (Node) |
| Git primitives on graph | ✅ | ✅ | ❌ | ❌ | ❌ |
| $0 deterministic ingest | ✅ | ✅ | ❌ (LLM) | ✅ | partial |
| Auto-capture hooks | ✅ | ❌ | ✅ | partial | ✅ |
| Code graph + impact | ✅ | partial | ❌ | ✅ (deep) | partial |
| Real-time viewer | ✅ | ❌ | ✅ | ✅ (CBM Lens) | ✅ |
| Bi-temporal facts | ✅ | ❌ | ❌ | ❌ | ❌ |
| License | Apache-2.0 | Apache-2.0 | Apache-2.0 | MIT | Apache-2.0 |

---

## 3. Competitive Landscape & Deep Analysis

Deep analysis performed via direct clones of all source repositories (see Appendix B). READMEs, docs, and source tree structure were inspected.

### 3.1 Per-Source Extraction Summary

| Source | Stars | Extracted Strength | Deliberately Excluded |
|---|---|---|---|
| `Uranid/mnem` | 137 | Baseline: Git primitives, content-addressed commits, $0 ingest, single binary, token-budget transparency, tombstone/supersession | — |
| `rohitg00/agentmemory` | ~30k | 12 auto-capture hooks, secret scrubbing, offline replay, 4-tier consolidation+decay, session replay, real-time viewer, JSONL import, 95.2% R@5 (LongMemEval-S) | iii-engine runtime, cloud dashboard |
| `DeusData/codebase-memory-mcp` | ~47k | 162 tree-sitter grammars, hybrid LSP type resolution, IaC indexing, cross-service linking, cross-repo intelligence, Smart CI, CBM Lens 3D viz, auto-sync watcher | C-only stack, commercial scale |
| `mnemon-dev/mnemon` | ~657 | LLM-supervised pattern, intent-native protocol (remember/link/recall), four-graph, importance decay+GC, privacy receipts, markdown-installable harness, web workspace, named stores | Go/SQLite stack |
| `Coding-Dev-Tools/engraphis` | ~127 | Bi-temporal facts, 6-term recall score, Ebbinghaus decay, grounded abstain, Spec Studio, encryption at rest, code↔memory links, communities/hotspots, privacy receipts | Python stack, hosted Jev |
| `AVIDS2/memorix` | ~868 | Cross-agent shared memory (18+ agents), Memory Autopilot (bounded Workset), curated long-term memory, Git Memory, Reasoning Memory, Knowledge Workspace, orchestration, memcode, controlled media, multi-device sync, tool profiles | TS/Node stack |
| `Lyellr88/marm-memory` | ~422 | 3-in-1 (Core+Code+Concept graph), I/P/B memory frames, MARM Console (distill/terminal/connections), credential redaction, cold-memory detection, local-model chat | Python/FastAPI stack |
| `agentralabs/agentic-memory` | growing | Immortal append-only BLAKE3 storage, 5 indexes, tiered storage (20yr), Ghost Writer, WAL+CRC32 crash recovery, 5 novel cognitive queries, single `.amem` portability | Rust lib (reuse patterns) |
| `Mnemo-mcp/Mnemo` | growing | Tiered decay (hot/warm/cold/evicted), 14-lang code graph (LadybugDB), 56 MCP tools, auto-capture lifecycle hooks, dead code/convention/security scan, planning with deps, error patterns/incidents/lessons, API discovery, branch-aware memories | Python/LadybugDB stack |
| `basicmachines-co/basic-memory` | ~4.3k | Markdown-as-truth two-way editing, wikilink/observation/relation grammar, Obsidian integration, progressive MCP tool discovery with behavior hints, schema infer/validate/diff, cross-encoder reranking, cloud sync+snapshots | AGPL-3.0 (patterns only — license conflict) |
| `safishamsi/graphify` | growing | Multimodal ingestion (code/PDF/images via vision), EXTRACTED/INFERRED/AMBIGUOUS edge tagging, god nodes + surprising connections + suggested questions, token-benchmark reporting, auto-sync watch, git-commit-hook rebuild, wiki export, SHA256 cache | Claude-Code-skill only, no MCP server |

### 3.2 Key Insight

No single tool occupies the intersection of **(a) Git-versioned knowledge graph**, **(b) deterministic $0 ingest**, **(c) single offline binary**, and **(d) deep code + decision memory**. OmniMem is designed to be the first tool in that intersection, while selectively absorbing the best features from each competitor.

---

## 4. Feature Inventory (Full Candidate List)

70+ candidate unique features, grouped by capability area, tagged with source. This is the pre-selection list; §5 narrows it to v1.

### 4.1 Storage / Versioning / Identity
- Git primitives on knowledge graph: branch, diff, merge, revert, rollback, 3-way graph+embedding merge, pull/push — **mnem**
- Content-addressed commits (BLAKE3 CID), signed Ed25519 history — **mnem**
- Single-file portable binary format with BLAKE3 integrity chains, never deletes — **agentic-memory**
- Tiered storage Hot→Warm→Cold→Frozen, 20yr in ~500MB — **agentic-memory**
- Crash recovery via WAL + CRC32 — **agentic-memory**
- Encryption at rest (SQLCipher AES-256) — **engraphis**
- Team-shared compressed graph artifact — **codebase-memory-mcp**
- Named stores for project/agent isolation — **mnemon-dev/mnemon**
- Markdown-as-truth two-way editing — **basic-memory**

### 4.2 Ingestion (deterministic, $0 LLM)
- Deterministic $0 LLM ingest, reproducible, no-op on unchanged — **mnem**
- 162 tree-sitter grammars + hybrid LSP type resolution — **codebase-memory-mcp**
- Infrastructure-as-code indexing (Docker, K8s, Helm, Terraform, Ansible, CI) — **codebase-memory-mcp**
- Multimodal ingestion (code, PDF, images, diagrams via vision) — **graphify**
- EXTRACTED/INFERRED/AMBIGUOUS edge tagging — **graphify**
- SHA256 cache — only changed files reprocessed — **graphify**
- Auto-sync watcher with per-session ownership — **codebase-memory-mcp**
- 10 harvest adapters (ChatGPT, Claude, Claude Code, Gemini, Perplexity, PDF, MD, text, URL, universal) — **hive-mind**
- Import JSONL transcripts — **agentmemory**
- Controlled media (image/PDF/audio derivations, MiniMax gen) — **memorix**

### 4.3 Retrieval
- Token budget transparency: `tokens_used`, `candidates_seen`, `dropped` — **mnem**
- Hybrid RRF fusion (vector + BM25 + graph) — **mnem / memorix / Mnemo**
- 11-signal scoring (TF-IDF, RRI, AST profiles, data flow, MinHash, module proximity, graph diffusion) — **codebase-memory-mcp**
- Bi-temporal 6-term recall score (retention, semantic, lexical, graph, importance, recency) — **engraphis**
- Intent-aware recall, query-type routing (WHY/WHEN/GENERAL) — **mnemon-dev/mnemon**
- Cross-encoder reranking (local FastEmbed or LiteLLM) — **basic-memory**
- Grounded answers with abstain when evidence weak — **engraphis**
- FTS5 + vector hybrid with RRF — **marm-memory**
- Structural search (regex name, label, degree, file scoping) — **codebase-memory-mcp**
- Semantic search via bundled Nomic embed-code, no API key — **codebase-memory-mcp**
- Zero-LLM query expansion (entity extraction, case detection, path matching) — **Mnemo**

### 4.4 Memory Lifecycle
- Auto-capture via 12 native hooks, secret scrubbing, offline replay — **agentmemory**
- 4-tier consolidation + decay + auto-forget — **agentmemory**
- Importance decay + access-count boosting + GC — **mnemon-dev/mnemon**
- Ebbinghaus forgetting-curve decay, interaction-aware reinforcement — **engraphis / Mnemo**
- Retention tiers hot/warm/cold/evicted with pinned categories — **Mnemo**
- Curated long-term memory (episodic/semantic/procedural, evidence, portable user items) — **memorix**
- I/P/B memory frames (intra/predicted/bidirectional) with auto-compaction — **marm-memory / hive-mind**
- Tombstone/forget first-class, `revoked_by` supersession — **mnem**
- Conflict detection — **kraklabs/mie**
- Deduplication (exact + near, auto-replace) — **mnemon-dev/mnemon / Mnemo**
- Distill: transcript → proposals with duplicate evidence, guardrails review — **memorix / marm-memory**

### 4.5 Graph / Code Intelligence
- Code graph: files, classes, methods, CALLS edges, scope-resolved — **Mnemo / codebase-memory-mcp**
- Impact analysis: N-hop BFS blast radius — **Mnemo / codebase-memory-mcp**
- Dead code detection, convention checking, security scan — **Mnemo**
- Louvain/Leiden community detection — **codebase-memory-mcp / graphify**
- Cross-service linking: HTTP/gRPC/GraphQL/tRPC routes, channels — **codebase-memory-mcp**
- Cross-repo intelligence (CROSS_* edges) — **codebase-memory-mcp**
- Multi-repo workspace linking + cross-impact — **Mnemo**
- Code↔memory links, communities/hotspots, git/PR impact — **engraphis**
- Knowledge base: markdown docs indexed for semantic retrieval — **Mnemo**
- API discovery: OpenAPI + controller annotations — **Mnemo**
- Service-level lookup (one call → full architecture) — **Mnemo**

### 4.6 Temporal
- Temporal fact management with validity windows — **graphiti**
- Episodes & provenance — **graphiti**
- Bi-temporal facts — **engraphis**
- Branch-aware memories — **Mnemo**
- Drift detection (belief trajectory over sessions) — **agentic-memory**

### 4.7 Multi-agent / Integration
- MCP-native with 16–58 tools, tool profiles (micro/lite/team/full) — **memorix / Mnemo**
- 18–45 client integrations with auto-setup — **memorix / codebase-memory-mcp**
- Multi-agent shared server with leases + signals — **agentmemory**
- Orchestration: task planning, handoffs, file locks, verification gates, review loops — **memorix**
- Ghost Writer auto-sync to Claude/Cursor/Windsurf/Cody — **agentic-memory**
- LLM-supervised pattern: host LLM = supervisor, no embedded LLM — **mnemon-dev/mnemon**
- Intent-native protocol (remember/link/recall) — **mnemon-dev/mnemon**
- Markdown-installable harness (SKILL.md, hooks) — **mnemon-dev/mnemon**
- Optional multi-device sync (GitHub JSONL/S3/Postgres relay) — **memorix**

### 4.8 Observability / UI
- Real-time viewer with live memory build — **agentmemory**
- Session replay (timeline scrubber, play/pause, speed) — **agentmemory**
- Web knowledge workspace — **mnemon-dev/mnemon**
- Self-hosted WebUI with dashboard — **engraphis / marm-memory / memorix**
- MARM Console: distill, terminal, connections — **marm-memory**
- CBM Lens 3D visualization (Galaxy, architecture map, local chat) — **codebase-memory-mcp**
- Wiki compiler (interlinked wiki pages via LLM) — **hive-mind**
- Wiki export (agent-crawlable) — **graphify**
- Export as JSON/Datalog — **kraklabs/mie**
- Privacy-safe receipts (hashed audit) — **mnemon-dev/mnemon / engraphis**

### 4.9 Safety / Privacy
- Secret stripping auto-removes tokens/keys/passwords — **Mnemo / marm-memory**
- Credential redaction before storage with refusal of credential identifiers — **marm-memory**
- Pre-tool-use hook blocks catastrophic commands — **Mnemo**
- Security scan (hardcoded secrets, SQL injection, eval, shell injection) — **Mnemo**
- Encryption at rest — **engraphis**
- Grounded answers, cite or abstain — **engraphis**
- Privacy-safe receipts (SHA-256 chain) — **engraphis / mnemon-dev/mnemon**

### 4.10 Novel Query Types
- Belief revision (counterfactual cascade, read-only) — **agentic-memory**
- Reasoning gaps (structural health audit, 5 categories) — **agentic-memory**
- Analogical reasoning (subgraph pattern matching across domains) — **agentic-memory**
- Consolidation (dedup, link contradictions, promote inferences) — **agentic-memory**
- Drift detection (belief trajectory) — **agentic-memory**
- Shortest path (BFS) — **agentic-memory**
- Centrality (PageRank) — **agentic-memory**
- Smart CI / test impact selection — **codebase-memory-mcp**

### 4.11 Benchmarks / Quality
- 95.2% R@5 LongMemEval-S — **agentmemory**
- 100% R@5 coding-agent-life-v1 — **agentmemory**
- LoCoMo SOTA 86.49% — **hive-mind**
- 100% R@5 Mnemo — **Mnemo**
- Benchmark harness + reproducible scorecards — **agentmemory / mnem**

---

## 5. Feature Selection — v1 Scope

### 5.1 Selection Rationale

v1 targets **Option A — Recommended Core (30 features)**. Rationale:
- Preserves OmniMem's single-binary, offline, $0-ingest identity — the niche with no direct equivalent.
- Closes the largest competitive gaps (auto-capture, code graph, impact analysis, viewer, bi-temporal, grounded abstain, privacy receipts).
- Keeps scope shippable in ~20 weeks by a small team.
- Heavy/optional capabilities (multimodal vision ingest, IaC indexing, cross-repo intelligence, Smart CI, tiered storage, encryption at rest, multi-device sync, wiki compiler, 3D viz, orchestration, controlled media, novel cognitive queries, cross-encoder reranking, Obsidian two-way markdown) are deferred to v2+ as **plug-in modules** that slot into the architecture without rework.

### 5.2 v1 Selected Features (30)

| # | Feature | Source | Module |
|---|---|---|---|
| 1 | Git primitives on knowledge graph (branch/diff/merge/revert/rollback/3-way merge) | mnem | core |
| 2 | Content-addressed commits (BLAKE3 CID), signed history | mnem | core |
| 3 | Deterministic $0 LLM ingest, reproducible, no-op on unchanged | mnem | ingest |
| 4 | Single native binary, no daemon, no external DB, fully offline | mnem | build |
| 5 | Token budget transparency (`tokens_used`, `candidates_seen`, `dropped`) | mnem | graphrag |
| 6 | Tombstone/forget first-class + `revoked_by` supersession | mnem | core |
| 7 | Hybrid RRF retrieval (vector + BM25 + graph) | mnem/memorix/Mnemo | graphrag |
| 8 | First-class MCP + CLI + HTTP | mnem | mcp/http/cli |
| 9 | Auto-capture via lifecycle hooks, secret scrubbing, offline replay | agentmemory | hooks |
| 10 | 4-tier consolidation + decay + auto-forget | agentmemory | lifecycle |
| 11 | Real-time viewer with live memory build | agentmemory | viewer |
| 12 | Session replay (timeline scrubber) | agentmemory | viewer |
| 13 | JSONL transcript import | agentmemory | ingest |
| 14 | Tree-sitter code graph (files/classes/methods/CALLS) | codebase-memory-mcp/Mnemo | codeintel |
| 15 | Impact analysis (N-hop BFS blast radius) | codebase-memory-mcp/Mnemo | codeintel |
| 16 | Auto-sync watcher with per-session ownership | codebase-memory-mcp | codeintel |
| 17 | Community detection (Leiden) | codebase-memory-mcp/graphify | codeintel |
| 18 | LLM-supervised pattern (host LLM = supervisor, no embedded LLM) | mnemon-dev/mnemon | design |
| 19 | Intent-native protocol (remember/link/recall) | mnemon-dev/mnemon | core/mcp |
| 20 | Importance decay + access-count boosting + GC | mnemon-dev/mnemon | lifecycle |
| 21 | Privacy-safe receipts (hashed audit) | mnemon-dev/mnemon/engraphis | core |
| 22 | Markdown-installable harness (SKILL.md, hooks) | mnemon-dev/mnemon | hooks |
| 23 | Named stores for isolation | mnemon-dev/mnemon | core |
| 24 | Bi-temporal facts | engraphis | core |
| 25 | Grounded answers with abstain | engraphis | graphrag |
| 26 | Code↔memory links, communities/hotspots, git/PR impact | engraphis | codeintel |
| 27 | Cross-agent shared memory (18+ agents) + auto-setup | memorix | hooks/mcp |
| 28 | Tool profiles (micro/lite/team/full) | memorix | mcp |
| 29 | Secret stripping + credential redaction | Mnemo/marm-memory | ingest/lifecycle |
| 30 | Branch-aware memories | Mnemo | core |

### 5.3 Deferred to v2+ (Option B items)

Multimodal vision ingest, IaC indexing, cross-service/repo intelligence, Smart CI, tiered storage, encryption at rest, multi-device sync, wiki compiler, 3D viz, orchestration/locks, controlled media, novel cognitive queries (belief revision, reasoning gaps, analogical, consolidation, drift), cross-encoder reranking, Obsidian two-way markdown.

> To change scope, swap the selected list above; the architecture in §6 is designed so deferred modules plug in as optional components without rework.

---

## 6. System Architecture

### 6.1 High-Level

```
┌──────────────────────────────────────────────────────────────────┐
│                     OmniMem (single binary)                      │
├──────────────┬───────────────┬──────────────┬────────────────────┤
│  CLI         │  MCP server   │  HTTP server │ (opt) Py/WASM      │
├──────────────┴───────────────┴──────────────┴────────────────────┤
│  Surfaces: hooks installer, auto-capture, viewer, session replay  │
├──────────────────────────────────────────────────────────────────┤
│  Memory layer: nodes/edges, commit DAG, branches, merge, tombstone│
├──────────────┬──────────────┬──────────────┬─────────────────────┤
│  Ingest      │  Retrieval   │  Lifecycle   │  Code intelligence  │
│  (AST/MD/PDF)│  (RRF fusion) │  (decay/GC)  │  (tree-sitter)      │
├──────────────┴──────────────┴──────────────┴─────────────────────┤
│  Storage: content-addressed redb + HNSW + BM25/SPLADE + ONNX     │
└──────────────────────────────────────────────────────────────────┘
```

### 6.2 Crate/Module Layout (Rust workspace, mirroring mnem)

| Crate | Responsibility |
|---|---|
| `core` | Node/edge model, commit DAG, content-addressing (BLAKE3), branch/merge, tombstone/supersession, bi-temporal facts, named stores |
| `backend-redb` | ACID single-writer store, multi-reader |
| `ann` | In-process HNSW vector index |
| `sparse-providers` | BM25 / SPLADE |
| `embed-providers` | ONNX MiniLM default; pluggable Ollama/OpenAI/Cohere |
| `ingest` | File-type detection, heading-aware MD, AST code (tree-sitter), sliding-window PDF, deterministic, cached by content hash, JSONL import, secret stripping |
| `extract` | Entity/relation heuristics (rule-based NER + KeyBERT-style keyphrases), no LLM |
| `graphrag` | Hybrid RRF retrieval, multi-hop expansion, token-budget packing, grounded abstain, intent-aware routing |
| `lifecycle` | Decay, access-count boosting, GC, 4-tier consolidation, conflict detection, privacy receipts |
| `codeintel` | Tree-sitter code graph, CALLS edges, impact analysis, community detection, dead code, conventions, security scan, code↔memory links, auto-sync watcher |
| `mcp` / `http` / `cli` / `transport` | Surfaces |
| `hooks` | Auto-capture installers for 18+ agents, markdown harness (SKILL.md) |
| `viewer` | Real-time UI + session replay |
| `py` / `wasm` | Optional bindings |

### 6.3 Data Layout

- `.mnem/` directory in project root (committed alongside code), content-addressed.
- `~/.omnimem/` (or `$XDG_DATA_HOME`) for global/cross-project store and viewer state.
- Named stores via `OMNIMEM_STORE` env or `store create|set`.

### 6.4 Concurrency

- Single-writer lock (redb) for correctness; read concurrency unlimited.
- Multi-agent shared server mode (HTTP) with leases + signals for coordination (from agentmemory).

### 6.5 Surfaces

- **CLI:** `omnimem init|ingest|retrieve|add|branch|merge|diff|revert|tombstone|hooks|viewer|...`
- **MCP stdio server:** 54+ tools, profiles micro/lite/team/full.
- **HTTP server +** real-time viewer + session replay.
- **Auto-setup** for 18+ agents: `omnimem setup --agent <name> --global`.
- **Markdown-installable harness** (SKILL.md + hooks) for minimal runtimes.

---

## 7. Feature-to-Source Traceability Matrix

See §5.2 (v1 features) and §4 (full inventory) for source attribution per feature. Every v1 feature maps to at least one source repository (column "Source") and one implementation module (column "Module").

---

## 8. Implementation Roadmap

| Phase | Weeks | Deliverable |
|---|---|---|
| 0 — Foundation | 1–2 | Fork mnem workspace; rename crates; verify build + tests; CI, lint, release pipeline; naming + config (`omnimem.toml`) |
| 1 — Core memory | 3–5 | Content-addressed store, commit DAG, branch/merge/revert, tombstone/supersession, named stores, bi-temporal facts; port mnem tests |
| 2 — Ingest | 6–7 | File-type detection, MD heading chunker, tree-sitter code chunker (Rust/TS/Python/Go), PDF sliding window, deterministic hashing, cache, JSONL import, secret stripping |
| 3 — Retrieval | 8–9 | Hybrid RRF (vector+BM25+graph), multi-hop expansion, token-budget packing with transparency, grounded abstain, intent-aware query routing |
| 4 — Lifecycle | 10–11 | Importance decay, access-count boosting, GC, 4-tier consolidation, conflict detection, privacy receipts |
| 5 — Code intelligence | 12–14 | Tree-sitter code graph, CALLS edges, impact analysis, community detection, dead code, conventions, security scan, code↔memory links, auto-sync watcher |
| 6 — Surfaces & hooks | 15–17 | MCP server (profiles), CLI, HTTP, viewer + session replay, auto-setup for 18+ agents, markdown harness (SKILL.md + hooks) |
| 7 — Quality & release | 18–20 | Benchmark harness (LongMemEval, coding-agent-life, LoCoMo), reproducible scorecards, docs, packaging (cargo/npm/pip/brew/scoop/aur), v1.0 release |

**v2+ (deferred):** multimodal vision ingest, IaC indexing, cross-service/repo intelligence, Smart CI, tiered storage, encryption at rest, multi-device sync, wiki compiler, 3D viz, orchestration/locks, controlled media, novel cognitive queries, cross-encoder reranking, Obsidian two-way markdown.

---

## 9. Technology Stack & Key Decisions

### 9.1 Stack
- **Language:** Rust (single binary, offline, performance) — inherits mnem. Optional Python/WASM bindings.
- **Storage:** redb (ACID, single-writer) + content-addressed BLAKE3. (No Neo4j, no Postgres.)
- **Vector index:** in-process HNSW.
- **Sparse:** BM25 / SPLADE.
- **Embedder:** ONNX MiniLM-L6-v2 default (~40MB binary); pluggable Ollama/OpenAI/Cohere via config.
- **Code parsing:** tree-sitter (multi-lang).
- **Graph traversal:** petgraph or custom.
- **UI:** embedded web viewer (axum + static assets), no Node runtime required.
- **License:** Apache-2.0.
- **MSRV:** Rust 1.95+ (match mnem).

### 9.2 Key Decisions to Confirm
- **D1.** Keep redb single-writer or move to SQLite for multi-writer? → Recommend redb (correctness, matches mnem, simpler). SQLite only if multi-writer becomes a blocker.
- **D2.** Embedder default: MiniLM (free, offline) vs bge-small. → MiniLM (matches mnem/agentmemory).
- **D3.** Viewer: ship embedded (axum) vs separate binary. → Embedded (single binary identity).
- **D4.** Code graph storage: same redb graph vs separate LadybugDB. → Same graph (unified, matches mnem).

---

## 10. Open Decisions — CONFIRMED (2026-10-26)

All six decisions confirmed by project owner; recommended values adopted.

| # | Decision | Confirmed value |
|---|---|---|
| 1 | Product name | OmniMem |
| 2 | Scope | Option A (Recommended Core, 30 features) |
| 3 | Language stack | Rust |
| 4 | Code-graph depth in v1 | tree-sitter CALLS only (LSP deferred to v2) |
| 5 | Viewer in v1 | yes (real-time viewer + session replay) |
| 6 | Auto-setup agents in v1 | top 5 (Claude Code, Cursor, Codex, Gemini CLI, OpenCode) |

---

## 11. Risk Register

| Risk | Likelihood | Impact | Mitigation |
|---|---|---|---|
| Scope creep (30 features) | High | High | Phase-gated; v1 ships phases 0–7 only; deferred list explicit |
| Single-writer bottleneck (redb) | Medium | Medium | HTTP shared-server mode + leases/signals (agentmemory pattern) |
| Heuristic (no-LLM) extraction misses semantic relations | Medium | Medium | Keep mnem's stance: $0 ingest; optional LLM pass is v2 add-on, never required |
| Maturity vs 47k-star competitors | High | Medium | Benchmark harness + reproducible scorecards; target ≥95% R@5 on LongMemEval-S |
| License contamination (basic-memory AGPL) | Low | High | Extract patterns only; no code copy |
| Cross-platform build complexity (WASM, Python) | Medium | Low | Defer WASM/Python to late phase; CLI+MCP+HTTP first |

---

## 12. Success Criteria & Acceptance

- Single binary builds on Linux/macOS/Windows; ≤80MB.
- `omnimem init && omnimem ingest .` on a 500-file repo completes offline with $0 API cost.
- `omnimem retrieve "..."` returns grounded results with token budget report.
- Branch/diff/merge/tombstone work end-to-end with a test suite.
- ≥95% R@5 on LongMemEval-S (reproducible harness).
- MCP server connects to Claude Code + Cursor + Codex (3 agents) with auto-setup.
- All v1 features have at least one integration test.

---

## Appendix A — Glossary

- **CID:** Content Identifier (BLAKE3 hash of canonical DAG-CBOR encoding).
- **HNSW:** Hierarchical Navigable Small World — approximate nearest-neighbor vector index.
- **RRF:** Reciprocal Rank Fusion — combining multiple ranked lists.
- **MCP:** Model Context Protocol — tool-calling protocol for AI agents.
- **redb:** Embedded ACID key-value store (Rust), single-writer.
- **Tombstone:** Logical "forget" that filters a node from retrieval without deleting content.
- **Supersession:** Linking an old fact to its replacement via `revoked_by` edge.
- **Bi-temporal:** Tracking both transaction time and valid time for facts.
- **Grounded abstain:** Returning "insufficient evidence" instead of hallucinating.

---

## Appendix B — Source Repositories

Cloned (shallow) into `C:\Users\Ashiqe\AppData\Local\Temp\kilo\repos\`:

| Repository | Local path |
|---|---|
| `Uranid/mnem` | `repos/mnem` |
| `rohitg00/agentmemory` | `repos/agentmemory` |
| `DeusData/codebase-memory-mcp` | `repos/codebase-memory-mcp` |
| `mnemon-dev/mnemon` | `repos/mnemon` |
| `Coding-Dev-Tools/engraphis` | `repos/engraphis` |
| `AVIDS2/memorix` | `repos/memorix` |
| `Lyellr88/marm-memory` | `repos/marm-memory` |
| `agentralabs/agentic-memory` | `repos/agentic-memory` |
| `basicmachines-co/basic-memory` | `repos/basic-memory` |
| `safishamsi/graphify` | `repos/graphify` |

Additional references (not cloned, README-level only): `getzep/graphiti`, `ontogr/agentic-graphrag`, `kraklabs/mie`, `neo4j-labs/agent-memory`, `marolinik/hive-mind`, `TheC4pt/Mnemo`.

---

## Appendix C — Change Log

| Version | Date | Notes |
|---|---|---|
| 0.1 | 2026-10-26 | Initial draft: deep analysis of 10 repos, full feature inventory (70+), v1 selection (Option A, 30 features), architecture, roadmap, risks, 6 open decisions. |

---

*End of document.*
