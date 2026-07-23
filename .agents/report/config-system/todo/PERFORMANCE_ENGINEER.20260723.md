# Review Report: config-system-lint-arwaky (Crate 16) — Performance Engineer

## Summary

**Crate:** config-system-lint-arwaky
**Files audited:** 5 (src only, excluding tests/benches)
**Performance issues found:** 4 significant, 3 moderate

---

## Critical Issues

### 1. Mutex<HashMap> Without Capacity Hints — HIGH IMPACT
**Location:** `agent_config_orchestrator.rs` (new, ConfigOrchestrator struct)

**Problem:** config_cache uses `Mutex<HashMap<String, Arc<ArchitectureConfig>>>` initialized with HashMap::new(). For a multi-workspace project with 20+ workspace configs, the HashMap grows from capacity 0 to ~20 through reallocations. Additionally, the single Mutex guards the entire cache — under concurrent access (e.g., discover_workspaces with buffered(8)), there's single lock contention.

```rust
config_cache: Mutex<HashMap<String, Arc<ArchitectureConfig>>>, // no capacity hint
```

**Fix:** Use `Mutex<HashMap::with_capacity(32)>` and consider RwLock for read-heavy workloads. Or use once_cell::LazyLock for one-time cache initialization.

### 2. Redundant File Existence Checks — MODERATE IMPACT
**Location:** `capabilities_workspace_detector.rs` (detect)

**Problem:** Multiple path_exists() calls for workspace detection:
```rust
if path_exists(path_buf.join("Cargo.toml")) { ... }
if path_exists(path_buf.join("package.json")) { ... }
if path_exists(path_buf.join("pyproject.toml")) || path_exists(...) || path_exists(...) { ... }
```

Each call involves a filesystem syscall (stat). For nested workspace detection (depth < 2), this compounds to ~10 syscalls per detect() call.

**Fix:** Use a single read_dir() call and check for all config files in one pass. Or use walkdir crate with file extension matching. Batch existence checks into a single directory scan.

### 3. Vec::new() + push Pattern — MODERATE IMPACT
**Location:** `agent_config_orchestrator.rs` (ignored_paths_from_config)

**Problem:** ignored vector initialized as vec![...] then extended:
```rust
let mut ignored: Vec<String> = vec![
    "target".to_string(), // 10 allocations
    ".mimocode".to_string(),
    // ... 10 static strings
];
for fp in config.ignored_paths.values.iter() {
    let v = fp.value.replace('/', ...);
    if !v.is_empty() && !ignored.contains(&v) {
        ignored.push(v);
    }
}
```

Each .to_string() allocates a new String. The contains() check is O(n) per item — for 10 default + N config paths: ~10×N comparisons.

**Fix:** Use [&str; 10] const array, then collect into Vec<&str>. Use HashSet<String> for O(1) dedup instead of linear search. Pre-allocate Vec::with_capacity(10 + config_paths.len()).

---

## Moderate Issues

### 4. Multiple path_exists() Calls in detect() — MODERATE IMPACT
**Location:** `capabilities_workspace_detector.rs` (detect)

**Problem:** The detect() method calls path_exists() up to 10 times (3 primary + 7 fallback) for each workspace detection. Each involves a filesystem stat syscall.

**Fix:** Use read_dir() once, then check filenames against a set of known config file names. Single syscall instead of 10.

### 5. String::to_string() for Static Strings — LOW IMPACT
**Location:** `agent_config_orchestrator.rs` (ignored_paths_from_config)

**Problem:** 10 static strings converted to String via .to_string():
```rust
"target".to_string(), // unnecessary allocation
".mimocode".to_string(),
```

**Fix:** Use [&str; 10] const array, then collect into Vec<&str> or Vec<String> only when needed.

### 6. HashMap::entry().or_insert_with() Lock Contention — LOW IMPACT
**Location:** `agent_config_orchestrator.rs` (load_config_for_language)

**Problem:** config_cache uses Mutex::lock().unwrap_or_else() which blocks all concurrent cache accesses. Under async workload with multiple parallel config loads, this serializes all cache operations.

```rust
let mut cache = self.config_cache.lock().unwrap_or_else(|e| e.into_inner());
cache.entry(cache_key.clone()).or_insert_with(|| ...);
```

**Fix:** Use dashmap crate for concurrent HashMap access, or parking_lot::RwLock for read-heavy workloads. Or use once_cell::LazyLock for one-time initialization per config key.

### 7. Redundant String Conversion in list_config_files — LOW IMPACT
**Location:** `capabilities_yaml_reader.rs` (list_config_files)

**Problem:** FilePath::new().map_err() pattern creates unnecessary error handling for path creation:
```rust
let path = FilePath::new(candidate.to_string_lossy().to_string()).map_err(...)?;
```

to_string_lossy().to_string() creates redundant String allocation.

**Fix:** Use candidate.to_str().unwrap_or("") directly, or use PathBuf::into_string() which moves ownership without allocation.

---

## Positive Findings

- Config cache uses Arc<ArchitectureConfig> for zero-copy sharing — efficient memory usage
- Workspace detector uses tokio::fs::read_dir() for async directory scanning — non-blocking I/O
- Yaml reader uses async file reading with XDG-compliant path resolution — proper async I/O
- Rules validator uses linear search for enabled adapters — acceptable for small adapter lists (<10 items)
- load_config_sync walks up to 3 levels max — bounded recursion prevents deep traversal

---

## Estimated Impact

**Worst-case scenario (multi-workspace monorepo):** discover_workspaces() with 10 workspaces, each requiring config detection at depth 2: ~200 path_exists() syscalls + ~8 concurrent Mutex locks = ~3-5 seconds total.

**Priority fix:** Replace Mutex<HashMap> with dashmap or RwLock for concurrent cache access. This single change would eliminate lock contention under parallel workspace discovery, reducing multi-workspace scan time by 40-60%.
