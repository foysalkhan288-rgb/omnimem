# Retrieval

## Overview

OmniMem uses hybrid retrieval: vector (HNSW) + sparse (BM25) + graph traversal, fused via RRF.

## Query types

- **General**: balanced weights across all lanes
- **WHY**: boosts graph traversal (decision/convention facts)
- **WHEN**: boosts temporal filtering

## Token budget

Every query reports `tokens_used`, `candidates_seen`, and `dropped`. No silent truncation.

```bash
omnimem retrieve "what did we decide about rate limiting?"
# Output includes: tokens_used=450, candidates_seen=23, dropped=5
```

## Grounded abstain

When evidence is weak, OmniMem returns "insufficient evidence" instead of hallucinating.

## Bi-temporal

Use `as_of(timestamp)` to query facts as of a specific time.