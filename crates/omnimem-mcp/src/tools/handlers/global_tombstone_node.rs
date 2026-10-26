//! Handler for the `omnimem_global_tombstone_node` MCP tool.
//!
//! Parallel to `omnimem_tombstone_node` but targets the global graph
//! (~/.omnimemglobal/.omnimem/) instead of the local repo. Lets agents
//! honour "forget X" requests for facts stored in the global graph.

use crate::server::Server;
use anyhow::{Context, Result, anyhow, bail};
use omnimem_core::id::NodeId;
use serde_json::Value;

// ============================================================
// omnimem_global_tombstone_node
// ============================================================

pub(in crate::tools) fn global_tombstone_node(server: &mut Server, args: Value) -> Result<String> {
    let global_data = super::global_dir().join(".omnimem");
    if !global_data.is_dir() {
        return Ok(
            "omnimem_global_tombstone_node: global graph not found at ~/.omnimemglobal/.omnimem/. \
             Run `omnimem integrate` then `omnimem init` to create it.\n"
                .to_string(),
        );
    }

    let id_str = args
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("missing 'id'"))?;
    let id = NodeId::parse_uuid(id_str).context("invalid node UUID")?;
    let reason = args
        .get("reason")
        .and_then(Value::as_str)
        .unwrap_or("")
        .to_string();
    let agent_id = args
        .get("agent_id")
        .and_then(Value::as_str)
        .ok_or_else(|| anyhow!("missing 'agent_id'"))?
        .to_string();
    let message = args
        .get("message")
        .and_then(Value::as_str)
        .unwrap_or("omnimem_mcp global tombstone")
        .to_string();

    let repo = super::open_global_repo(server, &global_data)?;

    // 404-equivalent: node must exist in the global graph.
    if repo.lookup_node(&id)?.is_none() {
        bail!("no node with id={id_str} in global graph");
    }
    // 409-equivalent: refuse to re-tombstone.
    if repo.is_tombstoned(&id) {
        bail!("node {id_str} is already tombstoned in global graph");
    }

    let mut tx = repo.start_transaction();
    tx.tombstone_node(id, reason.clone())?;
    let opts = omnimem_core::repo::CommitOptions::new(agent_id.as_str(), message.as_str());
    let new_repo = tx.commit_opts(opts)?;

    let mut out = String::new();
    out.push_str("omnimem_global_tombstone_node: ok\n");
    out.push_str(&format!("  id:      {id_str}\n"));
    if !reason.is_empty() {
        out.push_str(&format!("  reason:  {reason}\n"));
    }
    out.push_str(&format!("  op_id:   {}\n", new_repo.op_id()));
    Ok(out)
}
