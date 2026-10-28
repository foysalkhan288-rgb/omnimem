//! Cross-agent shared memory: project binding, leases, signals.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// A shared store connection for cross-agent memory.
#[derive(Debug, Clone)]
pub struct SharedStore {
    /// Project name bound to this connection.
    pub project: String,
    /// Active leases held by agents.
    pub leases: Arc<Mutex<HashMap<String, Lease>>>,
}

/// A lease held by an agent on a resource.
#[derive(Debug, Clone)]
pub struct Lease {
    /// Agent identifier.
    pub agent: String,
    /// Resource being leased.
    pub resource: String,
    /// Expiry timestamp (unix millis).
    pub expires_at: u64,
}

impl SharedStore {
    /// Create a new shared store bound to `project`.
    pub fn new(project: impl Into<String>) -> Self {
        Self {
            project: project.into(),
            leases: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Acquire a lease on `resource` for `agent`. Returns true if acquired.
    pub fn acquire_lease(&self, agent: &str, resource: &str, ttl_ms: u64) -> bool {
        let mut leases = match self.leases.lock() {
            Ok(g) => g,
            Err(_) => return false,
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        // Remove expired leases.
        leases.retain(|_, l| l.expires_at > now);
        let key = format!("{}:{}", agent, resource);
        if leases.contains_key(&key) {
            return false;
        }
        leases.insert(
            key,
            Lease {
                agent: agent.to_string(),
                resource: resource.to_string(),
                expires_at: now + ttl_ms,
            },
        );
        true
    }

    /// Release a lease.
    pub fn release_lease(&self, agent: &str, resource: &str) -> bool {
        let mut leases = match self.leases.lock() {
            Ok(g) => g,
            Err(_) => return false,
        };
        let key = format!("{}:{}", agent, resource);
        leases.remove(&key).is_some()
    }

    /// List active leases.
    pub fn active_leases(&self) -> Vec<Lease> {
        let leases = match self.leases.lock() {
            Ok(g) => g,
            Err(_) => return Vec::new(),
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        leases.values().filter(|l| l.expires_at > now).cloned().collect()
    }
}