# Migration Guide

## From mnem

OmniMem is a fork of mnem. Your `.mnem/` directory is compatible. Rename to `.omnimem/` or keep `.mnem/` (OmniMem reads both).

```bash
# Install OmniMem
cargo install --locked omnimem-cli

# Your existing .mnem/ directory works as-is
omnimem stats
```

## From memorix

1. Export memorix memories: `memorix export --format jsonl > memories.jsonl`
2. Import into OmniMem: `omnimem ingest --format jsonl memories.jsonl`
3. Verify: `omnimem stats`

## From basic-memory

1. OmniMem reads Markdown. Point ingest at your basic-memory directory:
   `omnimem ingest /path/to/basic-memory/`
2. Wikilinks are preserved as edges.

## From Letta / Mem0

Export memories as JSONL and use `omnimem ingest --format jsonl`.