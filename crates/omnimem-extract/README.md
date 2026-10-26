# omnimem-extract

Extraction strategies for omnimem ingest pipelines.

Provides keyword extraction (KeyBERT-style cooccurrence) and LLM-assisted
extraction traits used by `omnimem-ingest`. Trust filters live here too:
each extracted entity carries provenance + a confidence score so the
ingest layer can drop low-trust noise before it reaches the graph.

Part of [omnimem](https://github.com/Uranid/omnimem): versioned, mergeable,
content-addressed knowledge graph for AI agent memory.

## License

Apache-2.0.
