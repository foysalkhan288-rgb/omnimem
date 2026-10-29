# OmniMem Integration Snippets

## AGENTS.md

```markdown
## Memory

This project uses OmniMem for persistent agent memory. Before starting work:

1. Run `omnimem retrieve "<topic>"` to recall relevant decisions and conventions.
2. After making a decision, run `omnimem remember "<decision>"`.
3. Link related entities with `omnimem link <src> <dst> <predicate>`.

Memory is stored in `.omnimem/` and committed alongside code.
```

## CLAUDE.md

```markdown
# OmniMem

This project uses OmniMem. Use the MCP tools (`omnimem_retrieve`, `omnimem_remember`, `omnimem_link`) to access persistent memory across sessions.

- At session start: recall project conventions and recent decisions.
- Before editing: recall relevant conventions.
- After decisions: remember them.
```

## Cursor (.cursorrules)

```
Use OmniMem MCP tools to recall and store project knowledge. Always check memory before making architectural decisions.
```