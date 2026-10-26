"""pyomnimem - git for knowledge graphs, with retrieval under a token budget.

The public Python API is re-exported from the compiled Rust extension
(`pyomnimem._omnimem`) so callers write:

    from pyomnimem import Repo, RetrievalResult

`pyomnimem.langchain.omnimemRetriever` wraps a `Repo` as a LangChain retriever
for drop-in use with LangChain / LangGraph / LlamaIndex pipelines that
accept any `BaseRetriever`. Install extras to pull the dependency:

    pip install omnimem-py[langchain]
"""

from __future__ import annotations

# Re-export the compiled Rust classes so `from pyomnimem import X` keeps
# working. `_omnimem` is deliberately private; its symbols have no
# backward-compat guarantee when accessed directly.
from pyomnimem._omnimem import (
    omnimemError,
    Repo,
    RetrievalResult,
    RetrievedItem,
    Transaction,
)

# NOTE: deliberately NOT imported here:
#   - pyomnimem.langchain  -- pulls langchain-core at import time; only
#                          imported when the user explicitly does
#                          `from pyomnimem.langchain import omnimemRetriever`.

__all__ = [
    "omnimemError",
    "Repo",
    "RetrievalResult",
    "RetrievedItem",
    "Transaction",
]

# Version is sourced from the Rust package via pyproject.toml; expose
# it here for parity with typical Python packages.
try:
    from importlib.metadata import version as _pkg_version

    __version__ = _pkg_version("omnimem-py")
except Exception:
    # Happens during editable / maturin develop on systems that can't
    # resolve importlib metadata. Non-fatal.
    __version__ = "0.0.0+unknown"
