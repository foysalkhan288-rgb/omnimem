# omnimem-cli

Command-line interface for omnimem - Git for AI Agent Knowledge.

Ships the `omnimem` binary: the Git-shaped porcelain that most users reach for
first. The binary walks up to the nearest `.omnimem/` like `git` does, and
persists defaults in `config.toml` so common retrieve knobs can be set once
and forgotten.

```bash
omnimem init
omnimem add node --summary "Alice lives in Berlin and works at Globex"
omnimem retrieve "Alice Berlin" --limit 10
```

Workspace top and full quickstart: [`../../README.md`](../../README.md),
[`../../docs/src/quickstart.md`](../../docs/src/quickstart.md).

Licensed under Apache-2.0.
