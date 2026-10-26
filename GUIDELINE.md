# OmniMem Guidelines

## When to remember

- Architectural decisions and their rationale.
- Coding conventions and style rules.
- User preferences (language, framework, tooling).
- Bug root causes and fixes.
- Project-specific terminology.

## When to recall

- At the start of a session: project conventions, recent decisions.
- Before editing: conventions, related prior work.
- When the user references past work.

## When NOT to remember

- Raw code snippets (use the code graph instead).
- Tool call traces.
- Temporary reasoning or drafts.
- Information already in version control.

## Memory hygiene

- One fact per node.
- Use typed edges to connect entities.
- Supersede outdated facts rather than deleting.
- Tombstone when the user says "forget".
- Run `omnimem gc` periodically to decay stale memories.