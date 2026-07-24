# Review Report: lint-arwaky — Performance Engineer

## Summary

Lint Arwaky is a Rust-based architecture compliance scanner with 17 workspace crates, 3 binary targets (CLI, MCP server, TUI), and a 7-layer AES architecture. The codebase demonstrates solid architectural discipline with proper DI wiring, contract-based layering, and subprocess isolation for linter calls. However, several performance opportunities exist: the hot scanning path uses fully synchronous I/O with no parallelization, the global file cache has unbounded growth risk, and config caching serializes all reads through a single `Mutex`. The project already depends on `rayon = "1.10"` but does not use it. Overall performance health is **MODERATE** — functional correctness is strong, but throughput and latency can improve significantly with async I/O, rayon-based parallelism, and cache optimization.

---

## Performance Profile Analysis

### Build & Runtime Characteristics

| Metric | Value | Notes |
|--------|-------|-------|
| Workspace crates | 17 | Deep dependency tree |
| Binary targets | 3 | CLI, MCP server, TUI |
| External deps | ~25 crates | serde, tokio, regex, clap, rayon (unused) |
| Release profile | `opt-level = "z"`, lto=false, codegen-units=16 | Size-optimized, not speed-optimized |
| Rayon dependency | Present but unused | Parallelism available but not leveraged |

The release profile favors binary size (`opt-level = "z"`) over execution speed. For a CLI tool where latency matters more than download size, `opt-level = "2"` or `"3"` would yield faster scans. The `codegen-units = 16` setting avoids the SIGSEGV bug but limits compiler optimization compared to single-unit builds.

---

## Findings by Category

### CPU & Computational Efficiency

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | Sequential file scanning in hot path — `run_all_checks` reads files one-by-one with no parallelization | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:139-180` | Replace sequential file reads with `rayon::parallel_iter` for O(N) → O(N/P) throughput |
| 2 | 🟡 WARNING | Fork+exec overhead per linter — `run_all_linters_json` spawns 6 separate `lint-arwaky-cli` subprocesses, each bootstraps full DI container | `crates/cli-commands/src/surface_check_command.rs:109-138` | Consider in-process linter calls via shared aggregates for same-project scans; reserve subprocesses for cross-language scans |
| 3 | 🟡 WARNING | No rayon parallelism despite dependency — `rayon = "1.10"` is in Cargo.toml but never used | `Cargo.toml` | Use `rayon::scope` or `par_bridge` for parallel file scanning and duplicate detection |
| 4 | 🟢 INFO | Config cache lock contention — single `Mutex<HashMap>` serializes ALL config reads across the entire application | `crates/config-system/src/agent_config_orchestrator.rs:31` | Switch to `RwLock` or per-language caches to allow concurrent reads |
| 5 | 🟢 INFO | String cloning in hot path — repeated `.clone()` on FilePath values and `to_string()` calls create unnecessary allocations | Throughout CLI commands | Use `&str` references where possible; batch string conversions |

### Memory Management & Leaks

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | Global file cache has unbounded growth risk — `MAX_CACHE_ENTRIES = 20_000` only prevents insertion past limit, no eviction of existing entries | `crates/shared/src/orphan-detector/utility_file_cache.rs:9-10` | Implement LRU eviction (e.g., `lru::LruCache`) or periodic cache rotation |
| 2 | 🟢 INFO | No explicit memory pooling — each lint result creates new `LintResult`, `ErrorCode`, `FilePath` structs with owned Strings | `crates/shared/src/cli-commands/` | Consider `Cow<'_, str>` for paths that may already be `String` or borrowed |
| 3 | 🟢 INFO | ScanReport duplication — violations are collected as `Vec<ViolationItem>`, then passed to formatters that clone them again for JSON/SARIF rendering | `crates/cli-commands/src/surface_output_component.rs:80-120` | Pass `&[ViolationItem]` by reference; avoid cloning in JSON serialization |

### I/O & Network Performance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | Synchronous file reads in hot path — `std::fs::read_to_string` blocks the tokio runtime during scan operations | `crates/code-analysis/src/agent_code_analysis_orchestrator.rs:155-170` | Use `tokio::fs::read_to_string` or spawn_blocking for non-blocking I/O |
| 2 | 🟢 INFO | No I/O batching — each file is opened, read, and closed individually; no read-ahead or batch reads | `crates/shared/src/orphan-detector/utility_file_cache.rs:30-45` | Batch directory reads; consider mmap for large files |
| 3 | 🟢 INFO | Canonicalize path in loop — `std::fs::canonicalize` called per-violation file path during filtering | `crates/cli-commands/src/surface_check_command.rs:145-150` | Cache canonicalized paths; batch canonicalize calls |

### Concurrency & Parallelism

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟡 WARNING | Single-threaded runtime in CLI — `new_current_thread()` used for all async operations, preventing true parallelism | `crates/cli-commands/src/surface_common_action.rs:15-24` | Use `tokio::runtime::Runtime::new()` (multi-thread) for scan operations; reserve current-thread for interactive commands |
| 2 | 🟢 INFO | Config discover_workspaces uses `buffered(8)` — good parallelism but limited by I/O-bound workspace detection | `crates/config-system/src/agent_config_orchestrator.rs:104-132` | Add rayon parallelism for file collection phase within each workspace |
| 3 | 🟢 INFO | Watch mode event loop uses single broadcast channel — all file-change events serialized through one receiver | `crates/file-watch/src/agent_watch_orchestrator.rs:85-105` | Consider per-extension channels or batched event processing for high-churn directories |

### Database & Query Performance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| N/A | — | No database queries in this project | — | — |

---

## Violations (if any)

### AES Convention Deviations

| Code | Severity | Issue | Location |
|------|----------|-------|----------|
| Performance | — | `rayon` dependency declared but unused — dead dependency increases build time and binary size | `Cargo.toml` |
| Performance | — | Release profile uses `opt-level = "z"` (size) instead of `"2"` or `"3"` (speed) for a latency-critical CLI tool | `Cargo.toml` |
| Performance | — | `codegen-units = 16` with `lto = false` — neither size-optimized nor speed-optimized; consider `lto = true` for release | `Cargo.toml` |

---

## Action Items

- [ ] **HIGH** Replace sequential file scanning in `run_all_checks` with `rayon::parallel_iter` — expect 3-5x throughput improvement on large projects
- [ ] **HIGH** Switch config cache from `Mutex<HashMap>` to `RwLock` or per-language caches to eliminate read contention
- [ ] **MEDIUM** Implement LRU eviction for global file cache (`FILE_CACHE`) to prevent unbounded memory growth
- [ ] **MEDIUM** Change release profile from `opt-level = "z"` to `"2"` and enable `lto = true` for faster execution
- [ ] **LOW** Replace subprocess spawning in `run_all_linters_json` with in-process aggregate calls for same-language scans
- [ ] **LOW** Use `Cow<'_, str>` for FilePath values to avoid unnecessary String allocations in hot paths
- [ ] **LOW** Batch canonicalize path calls during violation filtering instead of per-file canonicalization

---

## Fixed Code

### Fix 1: Parallel file scanning with rayon (HIGH priority)

**Before — sequential scan:**
```rust
// agent_code_analysis_orchestrator.rs — run_all_checks
for file in files {
    let c = match shared::code_analysis::utility_file_reader::read_lintable_file(file) {
        Ok(Some(content)) => content,
        // ... error handling
    };
    entries.push((file.clone(), c.clone()));
    // Run checks on each file sequentially
}
```

**After — parallel scan with rayon:**
```rust
use rayon::prelude::*;

// Parallel file reads and check execution
let results: Vec<Vec<LintResult>> = files.par_iter().map(|file| {
    let filename = Path::new(file)
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    
    let c = match shared::code_analysis::utility_file_reader::read_lintable_file(file) {
        Ok(Some(content)) => content,
        Ok(None) => return vec![/* skipped violation */],
        Err(e) => return vec![/* IO error */],
    };
    
    // Run checks on this file
    let mut file_violations = Vec::new();
    self.deps.bypass_checker.check_bypass_comments(file, &c, &mut file_violations);
    self.deps.dead_inheritance_checker.check_dead_inheritance(file, &c, &mut file_violations);
    // ... other checks
    file_violations
}).collect();

// Flatten results from all parallel tasks
let violations: Vec<LintResult> = results.into_iter().flatten().collect();
```

**Expected improvement:** 3-5x throughput on projects with 100+ source files (assuming 8 cores).

### Fix 2: Config cache with RwLock (HIGH priority)

**Before — Mutex serializes all access:**
```rust
pub struct ConfigOrchestrator {
    config_cache: Mutex<HashMap<String, Arc<ArchitectureConfig>>>,
}

// Single lock for both reads AND writes
let mut cache = self.config_cache.lock().unwrap();
cache.entry(cache_key).or_insert_with(|| /* parse */);
```

**After — RwLock allows concurrent reads:**
```rust
use std::sync::RwLock;

pub struct ConfigOrchestrator {
    config_cache: RwLock<HashMap<String, Arc<ArchitectureConfig>>>,
}

// Multiple readers can hold the read lock simultaneously
let config = {
    let cache = self.config_cache.read().unwrap();
    cache.get(&cache_key).cloned()
};

if config.is_none() {
    let mut cache = self.config_cache.write().unwrap();
    cache.entry(cache_key.clone()).or_insert_with(|| /* parse */);
}
```

**Expected improvement:** Eliminated lock contention during concurrent config reads in multi-project scans.

### Fix 3: LRU file cache (MEDIUM priority)

**Before — unbounded growth after limit:**
```rust
const MAX_CACHE_ENTRIES: usize = 20_000;

if cache.len() < MAX_CACHE_ENTRIES {
    cache.insert(path.value().to_string(), content.clone());
}
// Entries never evicted once inserted
```

**After — bounded LRU cache:**
```rust
use lru::LruCache;

static FILE_CACHE: OnceLock<LruCache<String, String>> = OnceLock::new();

fn cache() -> &'static LruCache<String, String> {
    FILE_CACHE.get_or_init(|| LruCache::<String, String>::new(20_000.try_into().unwrap()))
}

pub fn read_cached(path: &FilePath) -> ContentString {
    let mut cache = cache();
    
    if let Some(content) = cache.get(path.value()) {
        return ContentString::new(content.clone());
    }
    
    let content = fs::read_to_string(path.value()).unwrap_or_default();
    cache.put(path.value().to_string(), content);
    
    ContentString::new(content)
}
```

**Expected improvement:** Memory usage bounded to ~20,000 entries × average file size; old entries automatically evicted.

### Fix 4: Release profile optimization (LOW priority)

**Before — size-optimized:**
```toml
[profile.release]
opt-level = "z"      # Size optimization
lto = false           # No LTO
codegen-units = 16    # Parallel compilation
strip = "symbols"
```

**After — speed-optimized:**
```toml
[profile.release]
opt-level = "2"       # Good speed/size balance
lto = true            # Link-time optimization for best code quality
codegen-units = 1     # Required when LTO is enabled
strip = "symbols"
debug = false
incremental = false
```

**Expected improvement:** 10-25% faster execution due to better compiler optimizations and LTO inlining.

---

## Appendix: Performance Hot Path Summary

### `lint-arwaky scan` — Full execution timeline

```
1. Parse CLI args          (clap)           ~1-5ms
2. Build DI container      (new_default)    ~50-200ms  ← config reads, crate initialization
3. Spawn 6 subprocesses    (Command::new)   ~100-500ms  ← fork+exec overhead per linter
4. Each subprocess bootstraps DI  ~50-200ms  ← repeated 6x = 300-1200ms
5. Collect JSON results    (serde_json)     ~10-50ms
6. Filter violations       (canonicalize)   ~5-20ms/file
7. Format output           (render_*)       ~1-5ms
```

**Total estimated time for 500-file project: ~3-8 seconds**

### Bottleneck analysis

| Phase | Time | Dominant operation | Parallelizable? |
|-------|------|-------------------|-----------------|
| DI container bootstrap | 50-200ms | Config file reads, crate init | Partial (rayon for config parsing) |
| Subprocess spawning | 100-500ms | Fork+exec of 6 binaries | Already parallel (tokio::join!) |
| Per-subprocess DI | 300-1200ms | Repeated config reads, crate init | **YES — major optimization target** |
| File scanning (inside each linter) | Varies | Sequential file I/O | **YES — rayon parallelism** |
| JSON parsing | 10-50ms | serde_json deserialization | No (sequential per subprocess) |

The biggest win: **in-process linter calls** instead of subprocess spawning. This eliminates 6x DI bootstrap overhead (~300-1200ms) and allows shared config cache across all linters.
