# Benchmark Methodology

## Reproduction

```bash
# LongMemEval-S (500 questions)
omnimem bench run --dataset longmemeval --split S

# LoCoMo (10 conversations)
omnimem bench run --dataset locomo

# coding-agent-life-v1 (15 sessions)
omnimem bench run --dataset coding-agent-life
```

## Metrics

- **R@5**: Recall at 5 — fraction of queries where the correct answer is in the top-5 retrieved items.
- **Tokens/query**: Average tokens consumed per query.
- **Ingest cost**: Total LLM tokens spent during ingestion ($0 for OmniMem deterministic).

## Targets

| Benchmark | Target R@5 |
|---|---|
| LongMemEval-S | >= 95% |
| coding-agent-life-v1 | 100% |
| LoCoMo | track |

## Methodology

1. Ingest all sessions into a fresh OmniMem store.
2. For each question, retrieve top-5 and check if the answer-containing passage is present.
3. Report aggregate R@5 and per-category breakdown.
4. Reproduce with `--seed` for deterministic results.