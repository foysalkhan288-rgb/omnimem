# omnimem-bench

Benchmark harness for omnimem. Ships the runner, dataset cache,
LongMemEval / LoCoMo / ConvoMem / MemBench / hybrid-v4 scorers, the
cpu-local omnimem adapter, the bundled ONNX MiniLM-L6-v2 embedder, and
the `omnimem bench` interactive TUI.

## Status

- 6 benches: LongMemEval, LoCoMo, ConvoMem, MemBench (simple-roles +
  highlevel-movie), LongMemEval-hybrid-v4. All shipped, end-to-end.
- Adapter: in-process `omnimem` (`Repo::open_in_memory`).
- Run mode: cpu-local (single-threaded, in-process).
- Embedders: ONNX MiniLM (default, bundled, matches headline numbers)
  and bag-of-tokens (offline / CI fallback).

See ``
for the design rationale.

## Quickstart

```bash
omnimem bench               # interactive TUI
omnimem bench list          # JSON of available benches
omnimem bench fetch longmemeval    # cache LongMemEval (~264 MB, HF)
omnimem bench fetch locomo         # cache LoCoMo (~3 MB, GitHub raw)
omnimem bench fetch                # fetch every bench
omnimem bench run --benches longmemeval --with omnimem --mode cpu-local --out ./out
omnimem bench results ./out        # re-render RESULTS.md
```

## Smoke test

```bash
cargo run --example smoke -p omnimem-bench
```

A 5-question LongMemEval canary against a synthetic in-process
omnimem repo. Exits non-zero if `recall@5 == 0`.
