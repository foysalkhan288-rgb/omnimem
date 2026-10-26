//! Handler for `omnimem_global_ingest` - ingest source files into the global anchor graph.
//!
//! Opens `~/.omnimemglobal/.omnimem/` and runs the standard ingest pipeline against it.
//! Unlike `omnimem_ingest` (which operates on whatever repo the MCP server is pointed at),
//! this tool always targets the global graph regardless of server configuration.

use crate::server::Server;
use anyhow::Result;
use serde_json::Value;

pub(in crate::tools) fn global_ingest(server: &mut Server, args: Value) -> Result<String> {
    let global_data = super::global_dir().join(".omnimem");
    if !global_data.is_dir() {
        return Ok(
            "omnimem_global_ingest: global graph not found at ~/.omnimemglobal/.omnimem/. \
             Run `omnimem integrate` then `omnimem init` to create it.\n"
                .to_string(),
        );
    }
    let allow_labels = server.allow_labels;
    let repo = super::open_global_repo(server, &global_data)?;
    super::ingest::ingest_impl(repo, &global_data, allow_labels, args)
}
