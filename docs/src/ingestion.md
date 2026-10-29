# Ingestion

## Overview

OmniMem ingests files deterministically with $0 LLM cost. The same input always produces the same knowledge graph.

## Supported formats

- **Markdown** (.md, .mdx): heading-aware chunking, preserves hierarchy
- **Code** (.rs, .py, .ts, .go, .java, ...): tree-sitter AST parsing, extracts functions/classes/methods, CALLS edges
- **PDF**: sliding window with configurable max_tokens and overlap
- **JSONL transcripts**: Claude Code session import
- **Plain text**: paragraph splitting

## Commands

```bash
omnimem ingest .              # auto-detect all files in directory
omnimem ingest file.md        # single file
omnimem ingest --text "..."   # inline text
```

## Determinism

- Content hashing (BLAKE3) ensures re-ingesting unchanged files is a no-op.
- No LLM calls during ingestion.
- Reproducible across machines.