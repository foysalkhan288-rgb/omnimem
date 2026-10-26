# Quickstart

Get OmniMem running in 5 minutes.

## 1. Install

Pick whichever toolchain you already have:

```bash
# Cargo (Rust) — recommended for dev machines
cargo install --locked omnimem-cli --features bundled-embedder

# pip (Python) — coming in Phase 7
# pip install omnimem

# npm (Node.js) — coming in Phase 7
# npm install -g omnimem-cli
```

Verify:

```bash
omnimem --version
```

## 2. Initialize

```bash
cd your-project
omnimem init          # creates the .omnimem/ directory
```

## 3. Ingest

```bash
omnimem ingest .      # auto-detects file types: MD headings, code AST, PDF sliding window
omnimem ingest --text "Deploy window is Tuesdays 10-11 AM UTC"
```

Ingestion is deterministic and requires no LLM calls. Re-ingesting an unchanged file is a no-op.

## 4. Retrieve

```bash
omnimem retrieve "what's our deploy schedule?"
```

Every query reports `tokens_used`, `candidates_seen`, and `dropped` — no silent truncation.

## 5. Version your knowledge

```bash
omnimem branch create experiment
# ... agent writes findings on the branch ...
omnimem merge experiment
omnimem diff HEAD~1
omnimem revert HEAD~1
```

## 6. Wire into your agent

```bash
omnimem setup --agent claude-code --global   # or cursor, codex, gemini-cli, opencode
```

## Next steps

- Read [Core Concepts](core-concepts.md) to understand nodes, edges, commits, branches.
- See [Configuration](configuration.md) for embedder and store options.
- See [Architecture](architecture.md) for the crate layout.