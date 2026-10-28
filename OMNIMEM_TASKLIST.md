# OmniMem — Master Tasklist

**Document:** Companion to `OMNIMEM_PLAN.md` (v0.1)
**Scope:** v1 = Option A (Recommended Core, 30 features) + 7-phase roadmap
**Format:** Markdown checkboxes. Check off as completed. Grouped by phase → module → task.

> Rule: A task is `completed` only when its code, tests, and docs are merged. Phase exit requires all phase tasks checked + phase acceptance criteria (§12 of plan) met.

---

## Phase 0 — Foundation (Weeks 1–2)

### 0.1 Repository bootstrap
 - [x] 0.1.1 Create new Git repository `omnimem` (private → public later)
 - [x] 0.1.2 Initialize Rust workspace (`Cargo.toml` with members)
 - [x] 0.1.3 Fork mnem workspace into `crates/` (copy structure, rename crate prefixes `mnem-*` → `omnimem-*`)
- [x] 0.1.4 Verify `cargo build` succeeds on Linux/macOS/Windows
- [x] 0.1.5 Verify `cargo test --workspace` passes on clean fork
 - [x] 0.1.6 Set up `.gitignore`, `.gitattributes`, `rust-toolchain.toml` (MSRV 1.95)
 - [x] 0.1.7 Add `LICENSE` (Apache-2.0) + `NOTICE`
 - [x] 0.1.8 Add `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`

### 0.2 Tooling & CI
 - [x] 0.2.1 GitHub Actions: build matrix (Linux, macOS, Windows)
 - [x] 0.2.2 GitHub Actions: `cargo test --workspace`
 - [x] 0.2.3 GitHub Actions: `cargo clippy -- -D warnings`
 - [x] 0.2.4 GitHub Actions: `cargo fmt --check`
- [ ] 0.2.5 Pre-commit hooks (rustfmt, clippy)
 - [x] 0.2.6 Release workflow (tag → build binaries → GitHub Release)
 - [x] 0.2.7 Dependabot / cargo-deny for supply chain

### 0.3 Configuration & naming
 - [x] 0.3.1 Finalize product name (OmniMem vs rename) — decision D1
 - [x] 0.3.2 Define config file schema (`omnimem.toml`) with sections: `[storage]`, `[embed]`, `[ingest]`, `[retrieval]`, `[lifecycle]`, `[viewer]`
 - [x] 0.3.3 Implement config loader (TOML, env overrides, defaults)
 - [x] 0.3.4 Define env vars: `OMNIMEM_STORE`, `OMNIMEM_DATA_DIR`, `OMNIMEM_EMBED_PROVIDER`, etc.
 - [x] 0.3.5 Define `.omnimem/` (project) vs `~/.omnimem/` (global) data layout

### 0.4 Documentation skeleton
 - [x] 0.4.1 Create `docs/` mdBook structure (book.toml)
 - [x] 0.4.2 Write `docs/README.md` (quickstart)
 - [x] 0.4.3 Write `docs/core-concepts.md`
 - [x] 0.4.4 Write `docs/configuration.md`

**Phase 0 exit criteria:** clean build on all 3 OSes, tests pass, CI green, config loads.

---

## Phase 1 — Core Memory (Weeks 3–5)

### 1.1 Node & edge model
 - [x] 1.1.1 Define `Node` struct: id (UUID), cid (BLAKE3), ntype, summary, content, props, created_at, updated_at, valid_from, valid_to (bi-temporal)
 - [x] 1.1.2 Define `Edge` struct: src, dst, etype, props, weight, created_at, valid_from, valid_to
 - [x] 1.1.3 Define node types: `Fact`, `Decision`, `Convention`, `Entity:*`, `Doc`, `Chunk`, `CodeSymbol`, `Task`, `Incident`, `Lesson`
 - [x] 1.1.4 Implement bi-temporal fields (valid_from/valid_to) on nodes and edges (engraphis)
 - [x] 1.1.5 Unit tests for node/edge serialization (DAG-CBOR)

### 1.2 Content addressing
 - [x] 1.2.1 Implement canonical DAG-CBOR encoding
 - [x] 1.2.2 Implement BLAKE3 CID generation
 - [x] 1.2.3 Verify determinism: same input → same CID on all platforms
 - [x] 1.2.4 Implement CID-based deduplication

### 1.3 Storage backend (redb)
 - [x] 1.3.1 Integrate redb (single-writer, multi-reader)
 - [x] 1.3.2 Implement node table, edge table, index tables
 - [x] 1.3.3 Implement transactional commit (all-or-nothing)
 - [x] 1.3.4 Implement read snapshot isolation
 - [x] 1.3.5 Concurrency tests (1 writer + N readers)

### 1.4 Commit DAG & versioning
 - [x] 1.4.1 Define `Commit` struct: cid, parent(s), author, agent_id, task_id, message, timestamp, tree_cid
 - [x] 1.4.2 Implement commit creation
 - [x] 1.4.3 Implement commit history walk (`log`)
 - [x] 1.4.4 Implement `diff` between commits (node/edge added/removed/changed)
 - [x] 1.4.5 Implement `revert` to a prior commit
 - [x] 1.4.6 Implement signed commits (Ed25519) — optional, behind feature flag

### 1.5 Branching & merging
 - [x] 1.5.1 Implement branch refs (`refs/heads/*`)
 - [x] 1.5.2 Implement `branch create|list|delete`
 - [x] 1.5.3 Implement 3-way merge (graph + embedding merge)
 - [x] 1.5.4 Implement merge conflict detection → `.mnem/MERGE_CONFLICTS.json`
 - [x] 1.5.5 Implement `merge --strategy=ours|theirs`, `--dry-run`, `--continue`, `--abort`
 - [x] 1.5.6 Implement `pull` (fast-forward remote branch)
 - [x] 1.5.7 Merge tests (clean merge, conflict, abort)

### 1.6 Tombstone & supersession
 - [x] 1.6.1 Implement `tombstone_node(id, reason)` — logical forget
 - [x] 1.6.2 Filter tombstoned nodes from all retrieval paths
 - [x] 1.6.3 Implement `revoked_by` edge for supersession (old → new)
 - [x] 1.6.4 Implement `resolve(id)` — follow supersession chain to current truth
 - [x] 1.6.5 Audit trail preserved (prior commits still addressable)

### 1.7 Named stores
 - [x] 1.7.1 Implement store create/set/list
 - [x] 1.7.2 Implement `MNEMON_STORE`-style env var isolation (`OMNIMEM_STORE`)
 - [x] 1.7.3 Per-store data directory layout
 - [x] 1.7.4 Cross-store query (optional, read-only)

### 1.8 Branch-aware memories
 - [x] 1.8.1 Tag memories with current git branch
 - [x] 1.8.2 Filter recall by branch (default: current branch + main)
 - [x] 1.8.3 `--all-branches` flag for cross-branch recall

### 1.9 Intent-native protocol
 - [x] 1.9.1 Define MCP/CLI primitives: `remember`, `link`, `recall` (mnemon pattern)
 - [x] 1.9.2 Implement `remember(content, label, props)` → commit node
 - [x] 1.9.3 Implement `link(src, dst, predicate)` → commit edge
 - [x] 1.9.4 Implement `recall(query, budget)` → ranked, budget-packed nodes
 - [x] 1.9.5 Structured JSON output with signal transparency

**Phase 1 exit criteria:** commit DAG, branch/merge, tombstone, resolve, named stores all tested; `omnimem init` creates `.mnem/`.

---

## Phase 2 — Ingestion (Weeks 6–7)

### 2.1 File-type detection & routing
- [x] 2.1.1 Implement file-type detection by extension + content sniffing
- [x] 2.1.2 Route: `.md/.rst/.adoc` → heading chunker; code → AST chunker; `.pdf` → sliding window; other → paragraph
- [x] 2.1.3 Implement `--chunker` override (auto/paragraph/recursive/sentence/session/structural)

### 2.2 Markdown heading chunker
- [x] 2.2.1 Split on headings (h1–h6), preserve hierarchy
- [x] 2.2.2 Extract frontmatter (title, tags, permalink)
- [x] 2.2.3 Create `Doc` + `Chunk` nodes with `CONTAINS` edges

### 2.3 Code AST chunker (tree-sitter)
- [x] 2.3.1 Integrate tree-sitter (start: Rust, TypeScript/JS, Python, Go)
- [x] 2.3.2 Extract functions, classes, methods, structs, traits, interfaces
- [x] 2.3.3 Create `CodeSymbol` nodes with `DEFINES` edges (file → symbol)
- [x] 2.3.4 Extract call sites → `CALLS` edges (scope-resolved)
- [x] 2.3.5 Extract imports → `IMPORTS` edges
- [x] 2.3.6 Parse caching: skip unchanged files (hash-based)
- [x] 2.3.7 Add languages: Java, C/C++, C#, Kotlin, Swift, Scala, Ruby, PHP, Rust (full), Go (full)

### 2.4 PDF sliding-window chunker
- [x] 2.4.1 Extract text from PDF (text-layer only, no OCR in v1)
- [x] 2.4.2 Sliding window with configurable `max_tokens` + `overlap`
- [x] 2.4.3 Create `Chunk` nodes with page references

### 2.5 Deterministic hashing & dedup
- [x] 2.5.1 Content hash (BLAKE3) per chunk/node
- [x] 2.5.2 Skip ingest if content hash matches existing node (no-op)
- [x] 2.5.3 Cross-machine dedup verification test

### 2.6 JSONL transcript import
- [x] 2.6.1 Parse Claude Code JSONL transcript format
- [x] 2.6.2 Convert messages → memory nodes (user/assistant/tool)
- [x] 2.6.3 Mark origin channel `import`; index for search
- [x] 2.6.4 Trigger crystal/lesson extraction (optional, LLM-supervised)

### 2.7 Secret stripping & redaction
- [x] 2.7.1 Regex-based secret detection (tokens, keys, passwords, API keys)
- [x] 2.7.2 Replace detected secrets with `[redacted:<kind>]`
- [x] 2.7.3 Report redaction count in response
- [x] 2.7.4 Refuse credential identifiers (session/project/platform names containing secrets)
- [x] 2.7.5 Credential redaction before storage (marm-memory pattern)

### 2.8 Ingest CLI & MCP
- [x] 2.8.1 `omnimem ingest <path>` (file or directory, auto-detect)
- [x] 2.8.2 `omnimem ingest --text "..."` (inline)
- [x] 2.8.3 `omnimem ingest --url <url>` (deferred to v2, or basic)
- [x] 2.8.4 MCP tool `ingest` with `path`/`text`/`source` params
- [x] 2.8.5 Progress reporting + summary (nodes/chunks/entities/relations counts)

**Phase 2 exit criteria:** ingest of a 500-file mixed repo produces identical graph on re-run; secrets stripped; JSONL import works.

---

## Phase 3 — Retrieval (Weeks 8–9)

### 3.1 Vector search (HNSW)
- [x] 3.1.1 Integrate HNSW index (instant-distance or custom)
- [x] 3.1.2 Embed nodes via ONNX MiniLM-L6-v2 (default)
- [x] 3.1.3 Pluggable embedder: Ollama, OpenAI, Cohere via config
- [x] 3.1.4 Cosine similarity search with model binding (skip mismatched models)

### 3.2 Sparse search (BM25/SPLADE)
- [x] 3.2.1 Implement BM25 over node summaries/content
- [x] 3.2.2 SPLADE sparse vector support (optional)
- [x] 3.2.3 Tokenizer: camelCase/snake_case aware (cbm_camel_split pattern)

### 3.3 Graph traversal
- [x] 3.3.1 Multi-hop expansion (1–4 hops, configurable)
- [x] 3.3.2 Graph decay scoring (decay^depth)
- [x] 3.3.3 PPR (personalized PageRank) mode — optional
- [x] 3.3.4 Edge-type allowlist for traversal
- [x] 3.3.5 Per-seed outgoing-edge cap (prevent star-node starvation)

### 3.4 Fusion & ranking
- [x] 3.4.1 RRF fusion (vector + BM25 + graph)
- [x] 3.4.2 Convex min-max fusion (Bruch 2023) as alternative
- [x] 3.4.3 Intent-aware query routing (WHY/WHEN/GENERAL → adjust weights)
- [x] 3.4.4 Zero-LLM query expansion (entity extraction, case detection, path matching) — Mnemo pattern

### 3.5 Token budget packing
- [x] 3.5.1 Greedy pack under token budget
- [x] 3.5.2 Report `tokens_used`, `candidates_seen`, `dropped`
- [x] 3.5.3 No silent truncation

### 3.6 Grounded abstain
- [x] 3.6.1 Confidence threshold; return "insufficient evidence" when below
- [x] 3.6.2 Cite supporting nodes in response (provenance)
- [x] 3.6.3 Abstain tests (weak-evidence queries return abstain)

### 3.7 Structural search
- [x] 3.7.1 `search_graph` with label, name_pattern, qn_pattern, file_pattern, min/max degree, where
- [x] 3.7.2 `search_code` (graph-augmented grep over indexed files only)
- [x] 3.7.3 Cypher-like query support (`MATCH ... RETURN ...`)

### 3.8 Bi-temporal recall
- [x] 3.8.1 `as_of(timestamp)` query parameter
- [x] 3.8.2 Filter facts by valid_from/valid_to
- [x] 3.8.3 Timeline view (closed_only superseded versions)

### 3.9 MCP/CLI retrieval tools
- [x] 3.9.1 `retrieve` (hybrid, budget-packed)
- [x] 3.9.2 `search` (structural/semantic)
- [x] 3.9.3 `get_node`, `traverse`, `incoming_edges`
- [x] 3.9.4 `schema` (labels + edge types)
- [x] 3.9.5 `stats` (repo overview)

**Phase 3 exit criteria:** hybrid retrieval returns grounded results; token budget reported; bi-temporal `as_of` works; abstain on weak evidence.

---

## Phase 4 — Lifecycle (Weeks 10–11)

### 4.1 Decay & scoring
- [x] 4.1.1 Importance decay: `exp(-0.01 * days_old)` (Mnemo)
- [x] 4.1.2 Access-count boosting (reinforcement, capped 0.3)
- [x] 4.1.3 6-term recall score: retention, semantic, lexical, graph, importance, recency (engraphis)
- [x] 4.1.4 Retention tiers: hot (≥0.5), warm (≥0.25), cold (<0.25), evicted (<0.1 + age>60d)
- [x] 4.1.5 Pinned categories never evicted (architecture, decision, preference)

### 4.2 Consolidation (4-tier)
- [x] 4.2.1 Tier 1: exact dedup (content hash)
- [x] 4.2.2 Tier 2: near-dedup (MinHash/LSH similarity)
- [x] 4.2.3 Tier 3: conflict detection (contradiction threshold 0.6)
- [x] 4.2.4 Tier 4: auto-forget (decay below threshold + age)
- [x] 4.2.5 Consolidation report (what merged/forgotten)

### 4.3 Importance & GC
- [x] 4.3.1 Importance score per node (access frequency × recency × salience)
- [x] 4.3.2 Garbage collection pass (remove evicted nodes from active tree)
- [x] 4.3.3 GC respects tombstones (never hard-delete without explicit flag)
- [x] 4.3.4 `omnimem gc` CLI command

### 4.4 Conflict detection
- [x] 4.4.1 Detect contradicting facts (similarity ≥0.6 + opposing polarity)
- [x] 4.4.2 Auto-supersede: new fact links `revoked_by` to old
- [x] 4.4.3 Surface conflicts in viewer / CLI
- [x] 4.4.4 `omnimem conflicts` command

### 4.5 Privacy receipts
- [x] 4.5.1 Hashed operation receipts (SHA-256 chain)
- [x] 4.5.2 Export receipts without raw memory contents/queries
- [x] 4.5.3 `omnimem receipts export` command

### 4.6 Distill
- [x] 4.6.1 Transcript → proposals (sentence-level verbatim by default)
- [x] 4.6.2 Resolve each proposal vs store (new/duplicate/near)
- [x] 4.6.3 Guardrails review mode (auto-apply only if all deterministic checks pass)
- [x] 4.6.4 `distill propose|review|apply|discard` MCP/CLI

**Phase 4 exit criteria:** decay/GC runs without data loss; conflicts detected and superseded; receipts exportable; distill produces reviewable proposals.

---

## Phase 5 — Code Intelligence (Weeks 12–14)

### 5.1 Code graph
 - [x] 5.0.1 Create omnimem-codeintel crate skeleton (Cargo.toml, lib.rs, error.rs, README.md) -- Phase 5 started
- [x] 5.1.1 Tree-sitter parsing for 14 languages (Python, JS/TS, C#, Go, Java, Rust, Ruby, PHP, C/C++, Kotlin, Swift, Scala)
- [x] 5.1.2 Create `File`, `Folder`, `Class`, `Method`, `Function`, `Interface` nodes
- [x] 5.1.3 `DEFINES` edges (file → symbol)
- [x] 5.1.4 `CALLS` edges (scope-resolved, confidence scored)
- [x] 5.1.5 `IMPORTS` edges
- [x] 5.1.6 `IMPLEMENTS`, `INHERITS` edges
- [x] 5.1.7 `SIMILAR_TO` (MinHash + LSH near-clone detection)

### 5.2 Impact analysis
- [x] 5.2.1 N-hop BFS blast radius (upstream + downstream)
- [x] 5.2.2 Risk classification (CRITICAL/HIGH/MEDIUM/LOW by hop distance)
- [x] 5.2.3 `impact(symbol)` MCP/CLI tool
- [x] 5.2.4 Affected tests detection (via `CALLS` + test naming conventions)

### 5.3 Community detection
- [x] 5.3.1 Leiden/Louvain clustering on call graph
- [x] 5.3.2 Label communities (functional modules)
- [x] 5.3.3 Community membership on nodes

### 5.4 Code quality scans
- [x] 5.4.1 Dead code detection (no incoming edges, excluding entry points)
- [x] 5.4.2 Convention checking (naming per language)
- [x] 5.4.3 Security scan (hardcoded secrets, SQL injection, eval, shell injection)
- [x] 5.4.4 `audit` MCP/CLI tool

### 5.5 Code↔memory links
- [x] 5.5.1 Link memory nodes to code symbols (file path, symbol name)
- [x] 5.5.2 Communities/hotspots (fan-in, complexity signals)
- [x] 5.5.3 git/PR impact analysis (changed files → affected symbols → tests)

### 5.6 Knowledge base (docs)
- [x] 5.6.1 Index markdown docs → semantic retrieval
- [x] 5.6.2 API discovery (OpenAPI specs + controller annotations)
- [x] 5.6.3 Service-level lookup (one call → full architecture)

### 5.7 Auto-sync watcher
- [x] 5.7.1 File watcher (notify crate) per registered project
- [x] 5.7.2 Per-session ownership (watch removed when last owner disconnects)
- [x] 5.7.3 Re-index on change (incremental, hash-based skip)
- [ ] 5.7.4 `auto_watch` global toggle

### 5.8 Cross-repo (basic)
- [x] 5.8.1 Multi-repo workspace linking (`omnimem link <path>`)
- [x] 5.8.2 Cross-repo search (code + memory + knowledge)
- [x] 5.8.3 Cross-impact analysis (what breaks in other services)

**Phase 5 exit criteria:** code graph built on 5+ languages; impact analysis returns blast radius; dead code + security scan run; auto-sync re-indexes on change.

---

## Phase 6 — Surfaces & Hooks (Weeks 15–17)

### 6.1 MCP server
- [ ] 6.1.1 MCP stdio server (JSON-RPC 2.0)
- [x] 6.1.2 Tool profiles: micro (9), lite (21), team (29), full (48+)
- [ ] 6.1.3 Tool annotations (readOnlyHint, destructiveHint, idempotentHint, openWorldHint)
- [ ] 6.1.4 MCP resources (`omnimem://node/{id}`, `.../session/{id}`, `.../graph/stats`)
- [ ] 6.1.5 MCP prompts (`remember`, `reflect`, `correct`, `summarize`)
- [ ] 6.1.6 Content-Length framing, 8 MiB limit, validation (agentic-memory pattern)

### 6.2 CLI
- [ ] 6.2.1 `omnimem init` (per-repo or global)
- [ ] 6.2.2 `omnimem ingest` (file/dir/text)
- [ ] 6.2.3 `omnimem retrieve` / `search` / `recall`
- [ ] 6.2.4 `omnimem add node|edge`
- [ ] 6.2.5 `omnimem branch` / `merge` / `diff` / `revert`
- [ ] 6.2.6 `omnimem tombstone` / `resolve`
- [ ] 6.2.7 `omnimem gc` / `conflicts` / `receipts`
- [ ] 6.2.8 `omnimem distill`
- [ ] 6.2.9 `omnimem setup --agent <name>` (auto-setup)
- [ ] 6.2.10 `omnimem doctor` (diagnostics)
- [ ] 6.2.11 `omnimem status` / `stop` / `upgrade`
- [ ] 6.2.12 `--json` output for scripting
- [ ] 6.2.13 `--cwd` for project binding

### 6.3 HTTP server
- [ ] 6.3.1 Axum HTTP server (REST + MCP-over-HTTP)
- [ ] 6.3.2 Auth (bearer token, auto-generated secret)
- [ ] 6.3.3 Health endpoints (`/livez`, `/health`)
- [ ] 6.3.4 Viewer serving (static assets embedded)
- [ ] 6.3.5 Shared server mode (multi-agent, leases + signals)

### 6.4 Real-time viewer
- [x] 6.4.1 Live memory build view
- [x] 6.4.2 Memory browser (filter, edit, delete, forget)
- [ ] 6.4.3 Graph visualization (basic: nodes/edges, search)
- [ ] 6.4.4 Session replay (timeline scrubber, play/pause, speed 0.5x–4x)
- [ ] 6.4.5 Keyboard shortcuts (space toggle, arrows step)

### 6.5 Auto-capture hooks
- [x] 6.5.1 Hook installer for Claude Code (12 hooks)
- [x] 6.5.2 Hook installer for Codex (6 hooks)
- [x] 6.5.3 Hook installer for Cursor (lifecycle hooks)
- [x] 6.5.4 Hook installer for OpenCode, Gemini CLI, Copilot CLI, Windsurf, Kiro, Antigravity, Trae, OpenClaw, Hermes, Pi, Oh My Pi, DeepSeek Harness, WorkBuddy, Grok
- [x] 6.5.5 Secret scrubbing in capture
- [ ] 6.5.6 Offline replay (spool + drain)
- [ ] 6.5.7 `omnimem capture` / `capture --drain`

### 6.6 Markdown harness
- [x] 6.6.1 `SKILL.md` template (commands, judgment)
- [x] 6.6.2 `GUIDELINE.md` (when to remember/recall)
- [x] 6.6.3 4 lifecycle phases: Prime, Remind, Nudge, Compact
- [x] 6.6.4 `omnimem setup --target <agent>` deploys harness

### 6.7 Cross-agent shared memory
- [x] 6.7.1 Shared store across agents (one server, many clients)
- [x] 6.7.2 Project attribution (per-session project binding)
- [x] 6.7.3 `memorix_session_start`-style project binding

**Phase 6 exit criteria:** MCP server connects to Claude Code + Cursor + Codex; viewer + replay functional; hooks capture on 3 agents; `omnimem setup` works.

---

## Phase 7 — Quality & Release (Weeks 18–20)

### 7.1 Benchmark harness
- [x] 7.1.1 Implement eval harness (adapter-pluggable, NDJSON output)
- [x] 7.1.2 LongMemEval-S adapter (500 questions) — target ≥95% R@5
- [x] 7.1.3 coding-agent-life-v1 adapter (15 sessions) — target 100% R@5
- [x] 7.1.4 LoCoMo adapter (10 conversations) — track R@5
- [x] 7.1.5 Reproducible scorecards in `docs/benchmarks/`
- [x] 7.1.6 Token cost measurement (tokens/yr, $/yr)

### 7.2 Testing
- [ ] 7.2.1 Unit tests for all modules (target ≥80% coverage)
- [ ] 7.2.2 Integration tests: commit DAG, branch/merge, tombstone, resolve
- [ ] 7.2.3 Integration tests: ingest determinism (same input → same graph)
- [ ] 7.2.4 Integration tests: retrieval (hybrid, budget, abstain, bi-temporal)
- [ ] 7.2.5 Integration tests: code graph + impact analysis
- [ ] 7.2.6 Integration tests: hooks capture + replay
- [ ] 7.2.7 Cross-platform tests (Linux, macOS, Windows)
- [ ] 7.2.8 Fuzz tests (ingest, retrieval, merge)

### 7.3 Documentation
- [x] 7.3.1 User guide (quickstart, concepts, config)
- [ ] 7.3.2 MCP tool reference (all tools, params, examples)
- [ ] 7.3.3 CLI reference
- [x] 7.3.4 Architecture doc
- [ ] 7.3.5 Benchmark methodology + reproduction
- [ ] 7.3.6 Migration guide (from mnem, memorix, etc.)
- [ ] 7.3.7 AGENTS.md / CLAUDE.md integration snippets

### 7.4 Packaging & distribution
- [ ] 7.4.1 Cargo publish (`omnimem-cli`, `omnimem-core`, etc.)
- [ ] 7.4.2 npm package (`@omnimem/cli`, `@omnimem/mcp`)
- [ ] 7.4.3 PyPI package (`omnimem`)
- [ ] 7.4.4 Homebrew formula
- [ ] 7.4.5 Scoop manifest (Windows)
- [ ] 7.4.6 AUR package (Arch)
- [x] 7.4.7 Docker image (optional, for shared server)
- [x] 7.4.8 Install scripts (install.sh, install.ps1)
- [ ] 7.4.9 Auto-update mechanism

### 7.5 Release
- [ ] 7.5.1 v1.0.0 release notes
- [ ] 7.5.2 GitHub Release with binaries (Linux, macOS, Windows)
- [ ] 7.5.3 Checksums + SBOM
- [ ] 7.5.4 Announcement (README, docs, social)

**Phase 7 exit criteria:** all v1 success criteria (§12 of plan) met; benchmarks ≥ targets; docs complete; packages published.

---

## v2+ Deferred Backlog (not scheduled)

### Ingestion
- [ ] V2-ING-1 Multimodal vision ingest (images, diagrams, PDF OCR) — graphify
- [ ] V2-ING-2 Infrastructure-as-code indexing (Docker, K8s, Helm, Terraform, Ansible, CI) — codebase-memory-mcp
- [ ] V2-ING-3 10 harvest adapters (ChatGPT, Claude, Gemini, Perplexity, URL, universal) — hive-mind
- [ ] V2-ING-4 Controlled media (image/PDF/audio derivations, MiniMax gen) — memorix

### Storage
- [ ] V2-STO-1 Tiered storage (Hot/Warm/Cold/Frozen, 20yr) — agentic-memory
- [ ] V2-STO-2 Encryption at rest (SQLCipher AES-256) — engraphis
- [ ] V2-STO-3 Immortal append-only BLAKE3 storage (never delete) — agentic-memory
- [ ] V2-STO-4 Crash recovery via WAL + CRC32 (already partial) — agentic-memory
- [ ] V2-STO-5 Team-shared compressed graph artifact — codebase-memory-mcp

### Retrieval
- [ ] V2-RET-1 Cross-encoder reranking (local FastEmbed or LiteLLM) — basic-memory
- [ ] V2-RET-2 11-signal scoring (TF-IDF, RRI, AST profiles, data flow, MinHash, module proximity, graph diffusion) — codebase-memory-mcp
- [ ] V2-RET-3 Semantic search via bundled Nomic embed-code — codebase-memory-mcp
- [ ] V2-RET-4 FTS5 + vector hybrid with RRF (SQLite backend option) — marm-memory
- [ ] V2-RET-5 EXTRACTED/INFERRED/AMBIGUOUS edge tagging — graphify
- [ ] V2-RET-6 Zero-LLM query expansion (entity extraction, case detection, path matching) — Mnemo (already partial)

### Code intelligence
- [ ] V2-CI-1 Hybrid LSP type resolution (Python, TS/JS, PHP, C#, Go, C/C++, Java, Kotlin, Rust, Perl) — codebase-memory-mcp
- [ ] V2-CI-2 Cross-service linking (HTTP/gRPC/GraphQL/tRPC routes, channels) — codebase-memory-mcp
- [ ] V2-CI-3 Cross-repo intelligence (CROSS_* edges) — codebase-memory-mcp
- [ ] V2-CI-4 Smart CI / test impact selection — codebase-memory-mcp
- [ ] V2-CI-5 API discovery (OpenAPI + controller annotations) — Mnemo
- [ ] V2-CI-6 Dead code, convention, security scan (already partial) — Mnemo

### Temporal & cognitive
- [ ] V2-COG-1 Temporal fact management with validity windows — graphiti
- [ ] V2-COG-2 Episodes & provenance — graphiti
- [ ] V2-COG-3 Drift detection (belief trajectory) — agentic-memory
- [ ] V2-COG-4 Belief revision (counterfactual cascade, read-only) — agentic-memory
- [ ] V2-COG-5 Reasoning gaps (structural health audit, 5 categories) — agentic-memory
- [ ] V2-COG-6 Analogical reasoning (subgraph pattern matching) — agentic-memory
- [ ] V2-COG-7 Consolidation (dedup, link contradictions, promote inferences) — agentic-memory
- [ ] V2-COG-8 Shortest path (BFS) — agentic-memory
- [ ] V2-COG-9 Centrality (PageRank) — agentic-memory

### Multi-agent & sync
- [ ] V2-MA-1 Orchestration (task planning, handoffs, file locks, verification gates, review loops) — memorix
- [ ] V2-MA-2 Ghost Writer auto-sync to Claude/Cursor/Windsurf/Cody — agentic-memory
- [ ] V2-MA-3 Optional multi-device sync (GitHub JSONL/S3/Postgres relay) — memorix
- [ ] V2-MA-4 Curated long-term memory (episodic/semantic/procedural, portable user items) — memorix
- [ ] V2-MA-5 I/P/B memory frames with auto-compaction — marm-memory/hive-mind

### UI & observability
- [ ] V2-UI-1 CBM Lens 3D visualization (Galaxy, architecture map, local chat) — codebase-memory-mcp
- [ ] V2-UI-2 Wiki compiler (interlinked wiki pages via LLM) — hive-mind
- [ ] V2-UI-3 Wiki export (agent-crawlable) — graphify
- [ ] V2-UI-4 Export as JSON/Datalog — kraklabs/mie
- [ ] V2-UI-5 MARM Console (distill, terminal, connections) — marm-memory
- [ ] V2-UI-6 Obsidian two-way integration — basic-memory

### Memory model
- [ ] V2-MM-1 Markdown-as-truth two-way editing — basic-memory
- [ ] V2-MM-2 Wikilink/observation/relation grammar — basic-memory
- [ ] V2-MM-3 Schema infer/validate/diff — basic-memory
- [ ] V2-MM-4 Progressive MCP tool discovery with behavior hints (already partial) — basic-memory
- [ ] V2-MM-5 Conflict detection (mie_conflicts) — kraklabs/mie

---

## Cross-Cutting Tasks (all phases)

### Testing infrastructure
- [ ] CC-1 Set up test framework (cargo test + integration tests)
- [ ] CC-2 Golden-file tests for ingest (determinism)
- [ ] CC-3 Benchmark regression gate in CI
- [ ] CC-4 Cross-platform CI (Linux, macOS, Windows)

### Security
- [ ] CC-5 Dependency audit (cargo-deny)
- [ ] CC-6 Secret scanning in CI (gitleaks)
- [ ] CC-7 SBOM generation on release
- [ ] CC-8 Signed releases (cosign)

### Community
- [ ] CC-9 Issue templates
- [ ] CC-10 PR template
- [ ] CC-11 Roadmap published in repo
- [ ] CC-12 Discord/community channel

---

## Summary Counts

| Category | v1 Tasks | v2+ Tasks |
|---|---|---|
| Phase 0 Foundation | 22 | — |
| Phase 1 Core | 38 | — |
| Phase 2 Ingest | 32 | 4 |
| Phase 3 Retrieval | 30 | 6 |
| Phase 4 Lifecycle | 22 | — |
| Phase 5 Code intel | 32 | 6 |
| Phase 6 Surfaces | 40 | — |
| Phase 7 Quality | 30 | — |
| v2+ backlog | — | 38 |
| Cross-cutting | 12 | — |
| **Total** | **~258** | **~54** |

---

*End of tasklist. Update checkboxes as work completes. Phase exit requires all phase tasks checked + acceptance criteria met.*
