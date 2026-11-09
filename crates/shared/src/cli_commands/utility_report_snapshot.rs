// PURPOSE: utility_report_snapshot — read and write one scan's counts so the
// next `report` can say what changed. I/O only; the types live in
// taxonomy_report_snapshot_vo because a `utility_` file holds no types (AES404).
use shared_common::ReportSnapshot;
use shared_common::SnapshotStore;
use shared_filesystem::utility_filesystem_io::write_string;
use std::path::{Path, PathBuf};

/// A stable, collision-free key for a target path.
///
/// The readable part keeps `snapshots.json` legible; the FNV-1a suffix is what
/// makes it a key. Collapsing every non-alphanumeric run to `-` mapped
/// `/repo/a-b` and `/repo/a/b` onto the same slug, so reporting on one path
/// overwrote the other's baseline and the delta was computed against a
/// different tree.
fn slug_for(target: &str) -> String {
    let readable: String = target
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .trim_matches('-')
        .chars()
        .rev()
        .take(48)
        .collect::<String>()
        .chars()
        .rev()
        .collect();
    let readable = if readable.is_empty() {
        "workspace".to_string()
    } else {
        readable
    };
    format!("{readable}-{:016x}", fnv1a_64(target.as_bytes()))
}

/// FNV-1a. Inline rather than `DefaultHasher` because the key is persisted:
/// `DefaultHasher`'s algorithm is explicitly allowed to change between Rust
/// releases, which would orphan every stored baseline on a toolchain bump.
fn fnv1a_64(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in bytes {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// The key a target's snapshot is filed under.
///
/// Public because the reader has to derive the same key the writer used; the
/// slug is meaningless on its own and nothing else needs to know about it.
pub fn target_key(target: &str) -> String {
    slug_for(target)
}

/// Where snapshots live: `$XDG_DATA_HOME/lint-arwaky/snapshots.json`, falling
/// back to `~/.local/share` when `XDG_DATA_HOME` is unset.
///
/// The XDG data dir rather than the repository, so a scan does not leave a file
/// behind that a contributor has to decide whether to commit.
fn snapshot_path() -> PathBuf {
    let base = std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or_else(|| {
            let home = std::env::var_os("HOME")
                .map(PathBuf::from)
                .unwrap_or_else(|| PathBuf::from("."));
            home.join(".local").join("share")
        });
    base.join("lint-arwaky").join("snapshots.json")
}

pub fn read_store() -> SnapshotStore {
    read_store_at(&snapshot_path())
}

/// Read the store from an explicit path.
///
/// The path is a parameter so the round trip can be tested: the only other way
/// to point it at a scratch directory is the environment, and setting that
/// needs `unsafe` in a workspace that forbids it.
pub fn read_store_at(path: &Path) -> SnapshotStore {
    let Ok(text) = std::fs::read_to_string(path) else {
        return SnapshotStore::default();
    };
    match serde_json::from_str(&text) {
        Ok(store) => store,
        Err(e) => {
            // Keep the unreadable file instead of letting the next write
            // replace it: a truncated or half-written store is the one case
            // where the baseline is still partly recoverable by hand.
            let quarantine = path.with_extension("corrupt.json");
            let preserved = std::fs::rename(path, &quarantine).is_ok();
            // stderr, so piping the report's stdout to a file still shows this.
            eprintln!(
                "lint-arwaky: the saved report snapshot at {} could not be read ({e}); \
                 treating this run as the first{}",
                path.display(),
                if preserved {
                    format!(" (unreadable file kept at {})", quarantine.display())
                } else {
                    String::new()
                }
            );
            SnapshotStore::default()
        }
    }
}

/// Write the store for `target`, keeping every other target's entry.
pub fn write_store(target: &str, snapshot: ReportSnapshot) -> std::io::Result<()> {
    write_store_at(&snapshot_path(), target, snapshot)
}

/// Write the store to an explicit path. See [`read_store_at`] for why.
///
/// A failure is returned rather than swallowed: the report still prints, but a
/// reader should know the next run has no baseline to compare.
pub fn write_store_at(path: &Path, target: &str, snapshot: ReportSnapshot) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut store = read_store_at(path);
    store.entries.insert(slug_for(target), snapshot);
    // Serialize the whole store, not its map. Writing `store.entries` produced
    // `{"key": {...}}` while `read_store` parses `{"entries": {"key": {...}}}`, so
    // the read failed and every report said it had no baseline.
    let json = serde_json::to_string_pretty(&store)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    // Write through a synced sibling + rename. A plain `fs::write` truncated
    // first, so a crash or a full disk mid-write destroyed every target's
    // baseline, not just this one.
    write_string(path, &json)
}
