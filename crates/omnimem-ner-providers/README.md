# omnimem-ner-providers

NER provider adapters for omnimem. Ships `RuleNer` (heuristic, zero-dependency) and `NullNer`.

Provides implementations of the `Ner` trait used by `omnimem-ingest` during entity extraction.
`RuleNer` applies a fast regex-based rule set to tag person, organization, and location spans.
`NullNer` is a no-op pass-through used when entity extraction is disabled.

Part of the [omnimem](https://github.com/Uranid/omnimem) workspace: Git for AI Agent Knowledge.
