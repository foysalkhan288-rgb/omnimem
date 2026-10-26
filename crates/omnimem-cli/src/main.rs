#![forbid(unsafe_code)]
#![deny(missing_docs)]
//! `omnimem` - the Git-shaped porcelain for omnimem.
//!
//! Minimum-viable CLI for the adoption surface: init / status / log /
//! show / add / query / diff / ref / config / stats. The Git-verb
//! spine ships incrementally: `branch`, `blame`, `cat-file`, `remote`,
//! `clone file://`, `fetch`, `push`, and `pull` are wired (with
//! wire verbs over HTTP). `merge` / `revert` / `fsck` / `gc` remain
//! advertised in `omnimem --help` but fail with EX_CONFIG until a later
//! revision.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod commands;
mod config;
mod doctor;
mod global;
mod http;
mod integrate;
mod mcp;
mod repo;
mod unintegrate;
mod wizard;

#[derive(Parser)]
#[command(
    name = "omnimem",
    version,
    about = "Git for AI Agent Knowledge. A persistent, versioned knowledge layer for AI agents.",
    long_about = None,
    propagate_version = true
)]
pub(crate) struct Cli {
    /// Path to the repository directory (the directory that contains
    /// `.omnimem/`). Defaults to walking up from the current directory,
    /// like `git` does.
    #[arg(long, short = 'R', global = true)]
    repo: Option<PathBuf>,

    /// Optional so bare `omnimem` drops into the first-run wizard (or
    /// `omnimem status` for returning users) instead of printing help.
    /// `omnimem --help` still prints help because clap captures that
    /// before we reach `main`.
    #[command(subcommand)]
    cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    /// Initialize a new omnimem repository (creates `.omnimem/repo.redb`).
    Init(commands::init::Args),
    /// Print current op-head, head commit, ref summary, and label counts.
    #[command(after_long_help = "\
Examples:
  omnimem status                    # current op + head commit + ref count
  omnimem -R ~/notes status         # explicit repo path
  omnimem status && omnimem retrieve \"query\"
")]
    Status,
    /// Walk the op-log backwards from the current head.
    Log(commands::log::Args),
    /// Show the full detail of one operation.
    Show(commands::show::Args),
    /// Add a node or edge (commits immediately).
    #[command(
        subcommand,
        after_long_help = "\
Examples:
  omnimem add node -s \"Alice lives in Berlin\"
  omnimem add node --label Person --prop name=Alice --prop city=Berlin \\
    -s \"Alice is a climber\"
  omnimem add edge --from <src-uuid> --to <dst-uuid> --label knows

See `omnimem add node --help` / `omnimem add edge --help` for full options.
"
    )]
    Add(commands::add::AddCmd),
    /// Query the graph via property filters + optional traversal.
    Query(commands::query::Args),
    /// Agent-facing retrieval: property filters + cosine vector
    /// (requires an embedder when a text query is set). When a
    /// learned-sparse provider is configured (`[sparse]` in
    /// config.toml), results are fused with the sparse lane via
    /// min-max convex combination (or RRF via `--fusion rrf`). There
    /// is no separate `--sparse` flag: sparse participation is
    /// driven by config.
    Retrieve(commands::retrieve::Args),
    /// Backfill embeddings for nodes that don't have one. Needs
    /// `embed.provider` + `embed.model` configured (see `omnimem config`).
    Embed(commands::embed_cmd::Args),
    /// Read embedding vectors from the sidecar.
    ///
    /// ```bash
    /// omnimem embedding get <node-uuid> <model>   # print vector as space-separated floats
    /// omnimem embedding ls  <node-uuid>           # list all stored model identifiers
    /// ```
    #[command(
        subcommand,
        after_long_help = "\
Examples:
  omnimem embedding get <uuid> onnx:all-MiniLM-L6-v2   # print embedding vector
  omnimem embedding ls  <uuid>                          # list all models for a node
"
    )]
    Embedding(commands::embedding_cmd::EmbeddingCmd),
    /// Retro-embed nodes that lack a vector. Like `omnimem embed` but
    /// adds `--since <commit>` and a `label + props` fallback for
    /// nodes without a `--summary`. Promoted as the recovery path
    /// when `omnimem add node` warned that the embedder was unreachable
    /// (audit-2026-04-25 C7-5).
    #[command(after_long_help = "\
Examples:
  omnimem reindex                       # embed every node missing a vector
  omnimem reindex --label Person        # only nodes of this label
  omnimem reindex --since <commit>      # only nodes added since <commit>
  omnimem reindex --force               # re-embed even already-embedded nodes
  omnimem reindex --dry-run             # report count without changing anything
")]
    Reindex(commands::reindex::Args),
    /// Embedder-manifest tooling. Currently exposes `audit`, the
    /// CI-enforceable check that every provider declares a noise
    /// floor (Gap 15).
    #[command(subcommand)]
    Embedder(commands::embed_cmd::EmbedderCmd),
    /// Report which refs and how many nodes/edges differ between two ops.
    Diff(commands::diff::Args),
    /// Manage named refs (branches, tags, arbitrary labels).
    #[command(
        subcommand,
        after_long_help = "\
Examples:
  omnimem ref list                                 # every ref in the current view
  omnimem ref set refs/heads/main <commit-cid>     # point `main` at a commit
  omnimem ref delete refs/heads/scratch            # remove a ref

Refs are just named pointers to commit CIDs. omnimem does not distinguish
branches from tags at the data layer; the convention `refs/heads/*`
for branches and `refs/tags/*` for tags follows git.
"
    )]
    Ref(commands::refs::RefCmd),
    /// Get or set repository configuration (`user.name`, `user.email`, ...).
    Config(commands::cfg_cmd::Args),
    /// Short one-line stats useful in prompts and shell scripts.
    #[command(after_long_help = "\
Examples:
  omnimem stats                     # one-line summary (nodes/edges/refs)
  omnimem stats | tee prompt.txt    # append to an LLM system prompt
  watch -n 5 omnimem stats          # live tail while ingesting
")]
    Stats,
    /// Print index schema: node labels, indexed props, and adjacency
    /// index presence. Mirrors the `omnimem_schema` MCP tool.
    #[command(after_long_help = "\
Examples:
  omnimem schema                    # human-readable label + prop index summary
  omnimem schema --json             # machine-readable JSON
  omnimem -R ~/notes schema         # explicit repo path
")]
    Schema(commands::schema::Args),
    /// Export the subtree reachable from a ref / CID to a CAR v1
    /// archive. The file can be shipped over any channel (email, USB,
    /// SSH, S3) and imported on the other side with `omnimem import`.
    Export(commands::export::Args),
    /// Import a CAR v1 archive into the current repository. Every
    /// block is CID-verified before being stored.
    Import(commands::import::Args),
    /// Ingest external source files (Markdown / text / PDF / chat JSON)
    /// into the current repository as a Doc + Chunk + Entity subgraph.
    #[command(after_long_help = "\
Examples:
  omnimem ingest notes.md                           # single-file markdown
  omnimem ingest --text \"The quick brown fox\"       # inline text (no file needed)
  omnimem ingest --chunker recursive --max-tokens 1024 book.pdf
  omnimem ingest --recursive docs/                  # walk a directory
")]
    Ingest(commands::ingest::Args),
    /// Manage `refs/heads/<name>` pointers (git analog: `git branch`).
    #[command(
        subcommand,
        after_long_help = "\
Examples:
  omnimem branch list                          # every refs/heads/*
  omnimem branch create feature/oauth          # at the current head
  omnimem branch create hotfix --from <cid>    # at a specific commit
  omnimem branch delete old-experiment
"
    )]
    Branch(commands::branch::BranchCmd),
    /// Manage `refs/tags/<name>` pointers (git analog: `git tag`).
    #[command(
        subcommand,
        after_long_help = "\
Examples:
  omnimem tag list                             # every refs/tags/*
  omnimem tag create v1.0                      # at the current head
  omnimem tag create v1.0 --from <cid>         # at a specific commit
  omnimem tag delete v0.9
"
    )]
    Tag(commands::tag::TagCmd),
    /// Walk the incoming-edge index for a node and report who points
    /// at it (git analog: `git blame`, but coarser - ).
    #[command(after_long_help = "\
Examples:
  omnimem blame <node-uuid>
  omnimem blame <node-uuid> --etype authored
")]
    Blame(commands::blame::Args),
    /// Graph traversal from a node. Lists outgoing edges, optionally
    /// filtered by --edge-label and bounded by --limit.
    #[command(after_long_help = "\
Examples:
  omnimem traverse <node-uuid>
  omnimem traverse <node-uuid> --edge-label knows
  omnimem traverse <node-uuid> -e knows -e authored --limit 10
  omnimem traverse <node-uuid> --json
")]
    Traverse(commands::traverse::Args),
    /// Fetch a single node by UUID and print its ntype, summary, and props.
    /// Use `omnimem cat-file --json <cid>` when you have a block CID instead.
    #[command(after_long_help = "\
Examples:
  omnimem get <uuid>
  omnimem get <uuid> --content                 # also print the content body
  omnimem global get <uuid>
")]
    Get(commands::get_node::Args),
    /// Soft-delete a node: records a tombstone with an optional reason.
    /// Tombstoned nodes are excluded from `omnimem retrieve` and `omnimem query`
    /// but remain in the blockstore and op-log for auditing.
    /// Prefer this over `omnimem delete` when an audit trail matters.
    #[command(after_long_help = "\
Examples:
  omnimem tombstone <uuid>
  omnimem tombstone <uuid> --reason \"superseded by newer decision\"
  omnimem global tombstone <uuid>
")]
    Tombstone(commands::tombstone::Args),
    /// Hard-delete a node from the current view. No audit trail.
    /// The block remains in the blockstore until `omnimem gc` runs.
    /// Use `omnimem tombstone` when an audit trail matters.
    #[command(after_long_help = "\
Examples:
  omnimem delete <uuid>
  omnimem global delete <uuid>
")]
    Delete(commands::delete_node::Args),
    /// Emit the raw bytes of a CID (binary-safe) or a decoded JSON
    /// preview (`--json`). Git analog: `git cat-file`.
    #[command(
        name = "cat-file",
        after_long_help = "\
Examples:
  omnimem cat-file <cid>                       # raw DAG-CBOR bytes
  omnimem cat-file <cid> --json | jq .         # pretty JSON
"
    )]
    CatFile(commands::cat_file::Args),
    /// Manage the global repo registry (~/.omnimemglobal/repos.toml).
    /// Lists, sets the default, or prunes stale entries.
    #[command(subcommand)]
    Repos(commands::repos::ReposCmd),
    /// Read and write the global anchor graph (~/.omnimemglobal/.omnimem/) directly.
    /// All subcommands operate exclusively on the global graph.
    #[command(subcommand)]
    Global(commands::global_cmd::GlobalCmd),
    /// Manage `[remote.<name>]` entries in `.omnimem/config.toml`. Pure
    /// local config-file ops; no network in 0.3.
    #[command(
        subcommand,
        after_long_help = "\
Examples:
  omnimem remote add origin file:///tmp/alice.car
  omnimem remote list
  omnimem remote show origin
  omnimem remote remove origin
"
    )]
    Remote(commands::remote::RemoteCmd),
    /// Clone a omnimem repo from an archive. 0.3 supports `file://` URLs
    /// and bare `.car` paths; remote schemes land in PR 3.
    #[command(after_long_help = "\
Examples:
  omnimem clone file:///tmp/alice.car /tmp/mirror
  omnimem clone ./alice.car
")]
    Clone(commands::clone::Args),
    /// Fetch new blocks from a remote (`refs/remotes/<name>/*` tracking refs).
    #[command(after_long_help = "\
Examples:
  omnimem fetch                         # fetch from `origin`
  omnimem fetch upstream                # explicit remote name

Authentication (when the remote requires it):
  omnimem_REMOTE_ORIGIN_TOKEN=... omnimem fetch origin
  omnimem_HTTP_PUSH_TOKEN=...    omnimem fetch origin  # fallback
")]
    Fetch(commands::fetch::Args),
    /// Push commits + blocks to a remote and advance its named ref.
    #[command(after_long_help = "\
Examples:
  omnimem push                          # push HEAD to origin/main
  omnimem push origin main              # explicit remote + branch

Authentication:
  omnimem_REMOTE_ORIGIN_TOKEN=... omnimem push origin main
")]
    Push(commands::push::Args),
    /// Fast-forward pull from a remote into the local branch.
    #[command(after_long_help = "\
Examples:
  omnimem pull                          # ff origin/main into local main
  omnimem pull origin main

Fast-forward only. Use `omnimem merge <remote>/<branch>` (B4) for 3-way merges.
")]
    Pull(commands::pull::Args),
    /// Switch HEAD to an existing branch (git analog: `git switch` /
    /// `git checkout <branch>`). Updates the active working position to
    /// the tip of the named branch without touching the working tree
    /// (omnimem has none).
    #[command(after_long_help = "\
Examples:
  omnimem switch main
  omnimem switch feature/oauth
  omnimem switch hotfix

Use 'omnimem branch list' to see available branches.
")]
    Switch(commands::switch::SwitchArgs),
    /// Alias for `omnimem switch`. Switch HEAD to an existing branch.
    #[command(after_long_help = "\
Examples:
  omnimem checkout main
  omnimem checkout feature/oauth

Use 'omnimem branch list' to see available branches.
")]
    Checkout(commands::switch::SwitchArgs),
    /// 3-way merge between branches. LCA + structured conflict
    /// detector + executor with `ours`/`theirs`/`manual` strategies.
    #[command(after_long_help = "\
Examples:
  omnimem merge feature                    # 3-way merge `refs/heads/feature` into HEAD
  omnimem merge feature --strategy=ours    # pick left / current side on conflict
  omnimem merge feature --strategy=theirs  # pick right / incoming side on conflict
  omnimem merge feature --dry-run          # preview outcome, persist nothing
  omnimem merge --continue                 # finish an in-progress merge after manual edits
  omnimem merge --abort                    # cancel an in-progress merge
")]
    Merge(commands::merge::Args),
    /// Revert an op's changes by creating a new inverse commit.
    Revert(commands::deferred::RevertArgs),
    /// Check the object DAG for corruption (reachability-only).
    Fsck(commands::fsck::Args),
    /// Garbage-collect unreferenced blocks.
    Gc(commands::gc::Args),
    /// Wire omnimem into every detected MCP agent host (Claude Desktop,
    /// Cursor, Continue, Zed) with atomic backups of the originals.
    Integrate(integrate::Args),
    /// Remove omnimem wiring from one or more agent hosts. Reads
    /// `~/.omnimemglobal/integrations.toml` to show what is currently
    /// integrated; interactive if no host names are given.
    #[command(after_long_help = "\
Examples:
  omnimem unintegrate                  # interactive: pick hosts to remove
  omnimem unintegrate claude-code      # remove a specific host
  omnimem unintegrate --all            # remove everything
  omnimem unintegrate --dry-run --all  # preview without changes
")]
    Unintegrate(unintegrate::UnintegrateArgs),
    /// Non-mutating health check: binaries, repo, config, embedder,
    /// wired hosts. Exits 1 if any check fails.
    Doctor(doctor::Args),
    /// Emit a shell completion script for bash / zsh / fish /
    /// powershell / elvish. Pipe the output into your shell's
    /// completion directory.
    #[command(after_long_help = "\
Examples:
  omnimem completions bash > ~/.local/share/bash-completion/completions/omnimem
  omnimem completions zsh > ~/.zsh/completions/_omnimem
  omnimem completions fish > ~/.config/fish/completions/omnimem.fish
  omnimem completions powershell >> $PROFILE
  omnimem completions elvish > ~/.config/elvish/lib/omnimem.elv
")]
    Completions(commands::completions::Args),
    /// Run omnimem-bench: download datasets, run scorers, emit
    /// RESULTS.md. 0.1.0 ships LongMemEval, LoCoMo, ConvoMem,
    /// MemBench (simple-roles + highlevel-movie), and
    /// LongMemEval-hybrid-v4 against the in-process omnimem adapter.
    /// mem0 / MemPalace adapters, CPU-parallel mode, and
    /// Docker-compose mode are stubbed and print a "coming 0.2.0"
    /// message at runtime.
    #[command(after_long_help = "\
Examples:
  omnimem bench                                          # interactive setup
  omnimem bench list --pretty                            # list all benches as JSON
  omnimem bench fetch longmemeval                        # download a dataset
  omnimem bench run --benches longmemeval --with omnimem \\
    --mode cpu-local --out ./bench-out --top-k 10 --limit 5 --non-interactive
  omnimem bench results ./bench-out                      # re-render RESULTS.md
")]
    Bench(commands::bench::BenchArgs),
    /// Model Context Protocol server (JSON-RPC over stdio). Exposes
    /// `omnimem_retrieve`, `omnimem_commit`, `omnimem_global_retrieve`, and 12+
    /// more tools to any MCP-aware host (Claude Desktop, Cursor, Claude
    /// Code, Continue, Zed, ...).
    ///
    /// ```bash
    /// omnimem mcp                        # auto-detect repo via walk-up
    /// omnimem mcp --repo ./my-repo       # explicit repo path
    /// ```
    ///
    /// Use `omnimem integrate` to auto-wire this into Claude Desktop etc.;
    /// or configure your client's `mcpServers` entry manually.
    Mcp(mcp::serve::ServeArgs),
    /// Serve the HTTP JSON API. Binds to loopback by default; set
    /// `--bind 0.0.0.0:9876` to expose (requires opt-in, see help).
    ///
    /// ```bash
    /// omnimem http --bind 127.0.0.1:9876 --repo .  # HTTP API server
    /// ```
    Http(http::serve::ServeArgs),
}

fn main() {
    let cli = Cli::parse();
    // Track whether the failure was an deferred-stub so the
    // caller gets the BSD-sysexits EX_CONFIG exit code instead of the
    // generic 1. We distinguish by pattern-matching the verb BEFORE
    // dispatch, not by parsing the error string, so future refactors
    // of `deferred::ex_config` can't drift the exit code.
    let result = match cli.cmd {
        None => wizard::run(cli.repo.as_deref()),
        Some(Cmd::Init(args)) => commands::init::run(cli.repo.as_deref(), args),
        Some(Cmd::Status) => commands::status::run(cli.repo.as_deref()),
        Some(Cmd::Log(args)) => commands::log::run(cli.repo.as_deref(), args),
        Some(Cmd::Show(args)) => commands::show::run(cli.repo.as_deref(), args),
        Some(Cmd::Add(sub)) => commands::add::run(cli.repo.as_deref(), sub),
        Some(Cmd::Query(args)) => commands::query::run(cli.repo.as_deref(), args),
        Some(Cmd::Retrieve(args)) => commands::retrieve::run(cli.repo.as_deref(), args),
        Some(Cmd::Embed(args)) => commands::embed_cmd::run(cli.repo.as_deref(), args),
        Some(Cmd::Embedding(sub)) => commands::embedding_cmd::run(
            cli.repo.as_deref(),
            commands::embedding_cmd::EmbeddingArgs { cmd: sub },
        ),
        Some(Cmd::Reindex(args)) => commands::reindex::run(cli.repo.as_deref(), args),
        Some(Cmd::Embedder(sub)) => {
            commands::embed_cmd::run_embedder(commands::embed_cmd::EmbedderArgs { cmd: sub })
        }
        Some(Cmd::Diff(args)) => commands::diff::run(cli.repo.as_deref(), args),
        Some(Cmd::Ref(sub)) => commands::refs::run(cli.repo.as_deref(), sub),
        Some(Cmd::Config(args)) => commands::cfg_cmd::run(cli.repo.as_deref(), args),
        Some(Cmd::Stats) => commands::stats::run(cli.repo.as_deref()),
        Some(Cmd::Schema(args)) => commands::schema::run(cli.repo.as_deref(), args),
        Some(Cmd::Export(args)) => commands::export::run(cli.repo.as_deref(), args),
        Some(Cmd::Import(args)) => commands::import::run(cli.repo.as_deref(), args),
        Some(Cmd::Ingest(args)) => commands::ingest::run(cli.repo.as_deref(), args),
        Some(Cmd::Branch(sub)) => commands::branch::run(cli.repo.as_deref(), sub),
        Some(Cmd::Tag(sub)) => commands::tag::run(cli.repo.as_deref(), sub),
        Some(Cmd::Blame(args)) => commands::blame::run(cli.repo.as_deref(), args),
        Some(Cmd::Traverse(args)) => commands::traverse::run(cli.repo.as_deref(), args),
        Some(Cmd::Get(args)) => commands::get_node::run(cli.repo.as_deref(), args),
        Some(Cmd::Tombstone(args)) => commands::tombstone::run(cli.repo.as_deref(), args),
        Some(Cmd::Delete(args)) => commands::delete_node::run(cli.repo.as_deref(), args),
        Some(Cmd::CatFile(args)) => commands::cat_file::run(cli.repo.as_deref(), args),
        Some(Cmd::Repos(sub)) => commands::repos::run(cli.repo.as_deref(), sub),
        Some(Cmd::Global(sub)) => commands::global_cmd::run(cli.repo.as_deref(), sub),
        Some(Cmd::Remote(sub)) => commands::remote::run(cli.repo.as_deref(), sub),
        Some(Cmd::Clone(args)) => commands::clone::run(cli.repo.as_deref(), args),
        Some(Cmd::Fetch(a)) => commands::fetch::run(cli.repo.as_deref(), a),
        Some(Cmd::Push(a)) => commands::push::run(cli.repo.as_deref(), a),
        Some(Cmd::Pull(a)) => commands::pull::run(cli.repo.as_deref(), a),
        Some(Cmd::Switch(a)) => commands::switch::run(cli.repo.as_deref(), a),
        Some(Cmd::Checkout(a)) => commands::switch::run(cli.repo.as_deref(), a),
        Some(Cmd::Merge(a)) => commands::merge::run(cli.repo.as_deref(), a),
        Some(Cmd::Revert(a)) => commands::deferred::run_revert(cli.repo.as_deref(), a),
        Some(Cmd::Fsck(a)) => commands::fsck::run(cli.repo.as_deref(), a),
        Some(Cmd::Gc(a)) => commands::gc::run(cli.repo.as_deref(), a),
        Some(Cmd::Integrate(args)) => integrate::run(args),
        Some(Cmd::Unintegrate(args)) => unintegrate::run(args),
        Some(Cmd::Doctor(args)) => doctor::run(cli.repo.as_deref(), args),
        Some(Cmd::Completions(args)) => commands::completions::run(args),
        Some(Cmd::Bench(args)) => commands::bench::run(args),
        Some(Cmd::Mcp(args)) => mcp::serve::run(args),
        Some(Cmd::Http(args)) => http::serve::run(args),
    };
    if let Err(e) = result {
        // audit-2026-04-25 R4 (Stage E re-fix): print the anyhow
        // error chain manually with consecutive-duplicate elision.
        // The previous `{e:#}` form printed each segment twice for
        // omnimem-core errors that wrap their inner Display in the
        // outer variant's `#[error("repo: {0}")]` while ALSO
        // exposing the inner as `#[source]` via `#[from]` -- the
        // chain walker then visited both. Surfaced as e.g.
        // `error: repo: retrieve: ...: retrieve: ...`.
        let mut last: Option<String> = None;
        let mut parts: Vec<String> = Vec::new();
        for cause in e.chain() {
            let s = format!("{cause}");
            // Skip a segment if it is identical to, OR a strict
            // suffix of, the previous segment. Strict-suffix
            // catches `repo: retrieve: ...` followed by
            // `retrieve: ...`.
            if let Some(prev) = &last
                && (prev == &s || prev.ends_with(&s))
            {
                continue;
            }
            last = Some(s.clone());
            parts.push(s);
        }
        eprintln!("error: {}", parts.join(": "));
        std::process::exit(1);
    }
}
