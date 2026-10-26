# Core Concepts

OmniMem stores knowledge as a **versioned, content-addressed knowledge graph**. This page explains the core primitives.

## Nodes and Edges

- **Node**: a fact, decision, convention, entity, document chunk, or code symbol. Carries a cryptographic identity (CID) derived from its content.
- **Edge**: a typed relationship between two nodes (e.g. `CALLS`, `works_at`, `revoked_by`, `CONTAINS`).

Every node and edge is content-addressed: identical content produces the same CID on any machine.

## Commits and Branches

Every write is a **commit** in an append-only DAG (like Git). Commits can be:
- **branched** (`branch create`) — independent lines of knowledge
- **diffed** (`diff`) — what changed between commits
- **merged** (`merge`) — 3-way graph + embedding merge
- **reverted** (`revert`) — roll back a bad batch of facts

## Content Addressing (BLAKE3)

Each node, edge, tree, and commit is identified by a BLAKE3 hash of its canonical DAG-CBOR encoding. This gives:
- Determinism: same input → same ID on any machine
- Deduplication: storing the same fact twice collapses to one node
- Replay: any prior commit is addressable forever

## Tombstones (Forgetting)

Forgetting is first-class. `tombstone(id, reason)` logically removes a node from all retrieval paths without deleting its content or breaking the audit trail. Supersession (`revoked_by` edges) lets a new fact replace an old one while preserving history.

## Bi-Temporal Facts

Facts carry two time dimensions:
- **Transaction time**: when OmniMem learned it
- **Valid time**: when it was true in the world

This enables `as_of(timestamp)` queries: "what did we know on date X?"

## Named Stores

Isolate memory by project or agent using named stores (`omnimem store create work`). The `OMNIMEM_STORE` env var selects the active store per process.