---
name: omnimem
description: Use OmniMem to remember, recall, and link facts across sessions. Triggers when the user asks to remember, recall, or when prior context would help.
---

# OmniMem Skill

## When to use

- The user says "remember", "note", "save", "don't forget".
- The user asks "what did we decide", "do you remember", "last time".
- Starting a new session where prior context (decisions, conventions, architecture) matters.
- Before editing code, recall relevant conventions and decisions.

## Commands

- `omnimem remember "<fact>"` — store a fact.
- `omnimem link <src> <dst> <predicate>` — connect two entities.
- `omnimem recall "<query>"` — retrieve relevant facts (budget-packed).
- `omnimem branch create <name>` / `merge <name>` — experiment with knowledge.
- `omnimem tombstone <id> "<reason>"` — forget a fact.

## Judgment

- Store decisions, conventions, architecture facts, and user preferences.
- Do NOT store raw code, tool traces, or temporary reasoning.
- Prefer one fact per node; link entities with typed edges.
- When a fact changes, supersede the old one (revoked_by) rather than deleting.

## Lifecycle phases

1. **Prime** — at session start, recall project conventions and recent decisions.
2. **Remind** — before answering, recall facts relevant to the user's question.
3. **Nudge** — after a decision is made, offer to remember it.
4. **Compact** — at session end, consolidate and forget stale facts.