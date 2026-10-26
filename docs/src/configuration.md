# Configuration

OmniMem is configured via a TOML file (`omnimem.toml`) and environment variables.

## Config File

Default locations (searched in order):
1. `./omnimem.toml` (project)
2. `~/.omnimem/config.toml` (global)

Example: see `config/omnimem.example.toml`.

Sections: `[storage]`, `[embed]`, `[ingest]`, `[retrieval]`, `[lifecycle]`, `[viewer]`.

## Environment Variables

See [CONFIG_ENV.md](CONFIG_ENV.md) for the full reference. Key variables:

| Variable | Default | Description |
|---|---|---|
| `OMNIMEM_STORE` | `default` | Active named store |
| `OMNIMEM_DATA_DIR` | `~/.omnimem` | Data directory |
| `OMNIMEM_EMBED_PROVIDER` | `onnx` | Embedder: `onnx`, `ollama`, `openai`, `cohere` |
| `OMNIMEM_EMBED_MODEL` | `all-MiniLM-L6-v2` | Embedding model |
| `OMNIMEM_LOG_LEVEL` | `info` | `debug`, `info`, `warn`, `error` |
| `OMNIMEM_SECRET` | (auto-generated) | HTTP server auth token |

## Named Stores

```bash
omnimem store create work
omnimem store set work
OMNIMEM_STORE=work omnimem retrieve "query"
```

## Embedder Providers

- **ONNX (default)**: runs in-process, ~40 MB binary, no API key, fully offline.
- **Ollama**: local HTTP, any model.
- **OpenAI / Cohere**: cloud, requires API key.

Switch providers with one config line: `provider = "ollama"` under `[embed]`.