//! Auto-sync watcher: re-index changed files incrementally.

use crate::graph::CodeGraph;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;

static AUTO_WATCH_ENABLED: AtomicBool = AtomicBool::new(true);

/// Enable or disable the global auto-watch toggle.
pub fn set_auto_watch(enabled: bool) {
    AUTO_WATCH_ENABLED.store(enabled, Ordering::Relaxed);
}

/// Check if auto-watch is enabled globally.
pub fn auto_watch_enabled() -> bool {
    AUTO_WATCH_ENABLED.load(Ordering::Relaxed)
}

/// A file-watcher that rebuilds the code graph on changes.
pub struct Watcher {
    root: PathBuf,
    _watcher: notify::RecommendedWatcher,
    rx: mpsc::Receiver<notify::Result<notify::Event>>,
}

impl Watcher {
    /// Start watching `root` for changes.
    pub fn start(root: impl Into<PathBuf>) -> crate::error::Result<Self> {
        let root = root.into();
        let (tx, rx) = mpsc::channel();
        let mut watcher = notify::recommended_watcher(move |res| {
            let _ = tx.send(res);
        })
        .map_err(|e| crate::error::CodeIntelError::Parse(e.to_string()))?;
        use notify::Watcher as _;
        watcher
            .watch(&root, notify::RecursiveMode::Recursive)
            .map_err(|e| crate::error::CodeIntelError::Parse(e.to_string()))?;
        Ok(Self {
            root,
            _watcher: watcher,
            rx,
        })
    }

    /// Return the watched root.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Drain pending events and rebuild the graph if any file changed.
    /// Returns `Some(graph)` if a rebuild occurred.
    pub fn poll(&self) -> Option<CodeGraph> {
        let mut changed = false;
        while let Ok(res) = self.rx.try_recv() {
            if let Ok(ev) = res {
                if !ev.paths.is_empty() {
                    changed = true;
                }
            }
        }
        if changed {
            CodeGraph::build(&self.root).ok()
        } else {
            None
        }
    }
}