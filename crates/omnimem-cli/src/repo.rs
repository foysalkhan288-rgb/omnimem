//! Repo location + open helpers shared across CLI commands.

use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result, anyhow, bail};
use omnimem_backend_redb::open_or_init;
use omnimem_core::repo::ReadonlyRepo;
use omnimem_core::store::{Blockstore, OpHeadsStore};

/// Directory name containing the omnimem repo artefacts inside a
/// user-visible folder.
pub(crate) const omnimem_DIR: &str = ".omnimem";

/// Resolve the repository root. If `override_path` is supplied it is
/// taken as the directory *containing* `.omnimem/`. Otherwise walk up
/// from `cwd` looking for a `.omnimem` subdir, like `git` does.
///
/// Returns the path to the `.omnimem` directory itself (the data root),
/// not the containing folder.
pub(crate) fn locate_data_dir(override_path: Option<&Path>) -> Result<PathBuf> {
    if let Some(p) = override_path {
        let data = if p.ends_with(omnimem_DIR) {
            p.to_path_buf()
        } else {
            p.join(omnimem_DIR)
        };
        if !data.is_dir() {
            bail!("no omnimem repository at {}", data.display());
        }
        return Ok(data);
    }
    let cwd = std::env::current_dir().context("cwd unreadable")?;
    let mut cursor: &Path = &cwd;
    loop {
        let candidate = cursor.join(omnimem_DIR);
        if candidate.is_dir() {
            return Ok(candidate);
        }
        match cursor.parent() {
            Some(parent) => cursor = parent,
            None => bail!(
                "no omnimem repository found in {} or any parent. Run `omnimem init` first.",
                cwd.display()
            ),
        }
    }
}

/// redb-backed data file inside the data dir.
pub(crate) fn db_path(data_dir: &Path) -> PathBuf {
    data_dir.join("repo.redb")
}

/// Open the store handles for an existing repository. Errors if the
/// dir / redb file does not exist.
pub(crate) fn open_stores(data_dir: &Path) -> Result<(Arc<dyn Blockstore>, Arc<dyn OpHeadsStore>)> {
    let db = db_path(data_dir);
    if !db.exists() {
        bail!(
            "redb file missing at {}; the repo exists but its storage is gone",
            db.display()
        );
    }
    let (bs, ohs, _) = open_or_init(&db)?;
    Ok((bs, ohs))
}

/// Create (or re-open) a repo at `data_dir`. Used by `omnimem init` and
/// as the transparent fallback in commands that accept an empty repo.
pub(crate) fn create_or_open_stores(
    data_dir: &Path,
) -> Result<(Arc<dyn Blockstore>, Arc<dyn OpHeadsStore>)> {
    std::fs::create_dir_all(data_dir)
        .with_context(|| format!("creating {}", data_dir.display()))?;
    let db = db_path(data_dir);
    let (bs, ohs, _) = open_or_init(&db)?;
    Ok((bs, ohs))
}

/// Load the current `ReadonlyRepo` for the located data dir.
pub(crate) fn open_repo(override_path: Option<&Path>) -> Result<ReadonlyRepo> {
    let (_dir, repo, _bs, _ohs) = open_all(override_path)?;
    Ok(repo)
}

/// Open-everything-in-one-shot. redb holds an exclusive file lock, so
/// opening the redb twice in the same process (e.g. `open_repo` +
/// `open_stores`) deadlocks. Commands that need both the repo view
/// AND direct store access (to decode an `IndexSet`, walk the op-log,
/// etc.) use this function to pay for the lock exactly once.
pub(crate) fn open_all(
    override_path: Option<&Path>,
) -> Result<(
    PathBuf,
    ReadonlyRepo,
    Arc<dyn Blockstore>,
    Arc<dyn OpHeadsStore>,
)> {
    let data_dir = locate_data_dir(override_path)?;
    let (bs, ohs) = open_stores(&data_dir)?;
    let repo = ReadonlyRepo::open(bs.clone(), ohs.clone()).or_else(|e| {
        // Structural match, not string-contains: an edit to the error
        // message must not silently change auto-init behaviour, and
        // genuine store corruption must not be masked by auto-init.
        if e.is_uninitialized() {
            ReadonlyRepo::init(bs.clone(), ohs.clone()).map_err(anyhow::Error::from)
        } else {
            Err(anyhow!(e))
        }
    })?;
    Ok((data_dir, repo, bs, ohs))
}
