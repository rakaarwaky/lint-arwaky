# Review Report: project-setup-lint-arwaky — Performance Engineer

## Summary

The `project-setup-lint-arwaky` crate handles project initialization, environment generation, and MCP server configuration. Main bottlenecks involve synchronous subprocess execution for binary discovery and deep directory scans.

## Performance Profile Analysis

- **Startup Latency:** Moderate delay when probing binary paths via `which` process execution.
- **I/O Efficiency:** Fast for Phase 1 (marker files), slower for Phase 2 fallback extension scans.

## Findings by Category

### CPU & Computational Efficiency

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 1 | 🟡 WARNING | `to_string_lossy()` allocations per directory entry | `capabilities_setup_processor.rs:276` | Use `path.file_name().and_then(|n| n.to_str())` |
| 2 | 🟢 INFO | Intermediate String array allocations in `generate_env` | `capabilities_setup_processor.rs:47` | Build string directly with `format!` |

### Memory Management & Leaks

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 3 | 🟡 WARNING | Repeated JSON HashMap cloning in MCP config generators | `capabilities_setup_processor.rs:73` | Reuse reference structure |

### I/O & Network Performance

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 4 | 🔴 CRITICAL | Synchronous `run_command("which", ...)` process spawn | `capabilities_setup_processor.rs:116` | Use `which` Rust crate directly |
| 5 | 🔴 CRITICAL | Recursive `std::fs::read_dir` up to depth 4 in Phase 2 | `capabilities_setup_processor.rs:258` | Restrict fallback scan to standard `src/` dirs |

### Concurrency & Parallelism

*(N/A — Linear setup processor)*

### Database & Query Performance

*(N/A — No database operations in this crate)*

## Violations (if any)

- Spawning external process binaries (`which`) for simple filesystem PATH searches.

## Action Items

- [ ] High Priority: Replace `proc_io::run_command("which")` with `which::which("lint-arwaky-mcp")`.
- [ ] High Priority: Restrict fallback extension scan to `src/`, `lib/`, `crates/`, `packages/`.
- [ ] Medium Priority: Avoid `to_string_lossy()` allocations in `scan_source_extensions`.

## Fixed Code

```rust
// Fixed binary lookup using Rust `which` crate (zero subprocess overhead)
fn which_mcp_binary_fast(&self) -> McpBinaryNameVO {
    if let Ok(path) = which::which("lint-arwaky-mcp") {
        return McpBinaryNameVO::new(path.to_string_lossy().to_string());
    }
    McpBinaryNameVO::new("lint-arwaky-mcp".to_string())
}
```

---

## Detailed Audit Findings

# Performance Audit: project-setup-lint-arwaky

## Summary

**Crate:** project-setup-lint-arwaky
**Files audited:** 5 (src only, excluding tests/benches)
**Performance issues found:** 2 high impact, 2 moderate impact

---

## Critical Issues

### 1. Unbounded Synchronous Directory Traversal for Language Detection — HIGH IMPACT
**Location:** `capabilities_setup_processor.rs` (scan_source_extensions)

**Problem:** If top-level project markers (`Cargo.toml`, `pyproject.toml`, `package.json`) are absent, `detect_languages()` falls back to `scan_source_extensions()`. This recursively walks directory trees up to depth 4 using synchronous `setup_io::read_dir_entries(dir)` calls. In monorepos or repositories with large source trees, synchronous file traversal blocks execution during `lint-arwaky init`.

```rust
fn scan_source_extensions(&self, dir: &std::path::Path, depth: usize, max_depth: usize, ...) {
    let entries = match setup_io::read_dir_entries(dir) { ... };
    for path in entries {
        // synchronous recursion up to depth 4
    }
}
```

**Fix:** Restrict detection scan to standard source directories (`src/`, `lib/`, `crates/`, `packages/`, `apps/`) instead of checking root `.`, or use async directory scanning.

### 2. Synchronous External Command Execution for Binary Lookup — HIGH IMPACT
**Location:** `capabilities_setup_processor.rs` (which_mcp_binary)

**Problem:** `which_mcp_binary()` executes `proc_io::run_command("which", &["lint-arwaky-mcp"])` synchronously using `std::process::Command`. This blocks the thread while searching PATH entries via shell subprocess.

```rust
let (stdout, _, success) = proc_io::run_command("which", &["lint-arwaky-mcp"]);
```

**Fix:** Use `which` crate directly in Rust for zero-subprocess PATH resolution, or execute asynchronously.

---

## Moderate Issues

### 3. Intermediate String Allocations in MCP Config Generators — MODERATE IMPACT
**Location:** `capabilities_setup_processor.rs` (generate_env, mcp_config_claude, mcp_config_vscode)

**Problem:** `generate_env()` constructs `[String; 5]` array, converts items via `to_string()`, then joins with `\n` creating intermediate strings. `mcp_config_claude()` and `mcp_config_vscode()` clone JSON HashMaps repeatedly.

**Fix:** Construct env strings directly with `format!` and reuse single `McpConfigVO` instances.

### 4. Unnecessary Lossy String Conversions — LOW IMPACT
**Location:** `capabilities_setup_processor.rs` (which_mcp_binary, scan_source_extensions)

**Problem:** `path.file_name().unwrap_or_default().to_string_lossy()` creates a new heap `Cow<str>` allocation for every directory entry evaluated during language detection scan.

**Fix:** Compare file name using `path.file_name().and_then(|n| n.to_str())`.

---

## Positive Findings

- Uses `include_str!` compile-time embedded YAML templates for config generation (`lint_arwaky.config.rust.yaml`, etc.), eliminating template file I/O at runtime.
- Directory exclusion filter skips build artifacts (`target`, `node_modules`, `vendor`, `dist`, `build`, `__pycache__`).
- Phase 1 marker-based language detection avoids directory scanning when standard config files are present.

---

## Estimated Impact

**Worst-case scenario:** Running `lint-arwaky init` in a directory without standard root files triggers recursive directory scanning of thousands of files across depth 4, adding 200-800ms startup latency.

**Priority fix:** Replace shell `which` command with `which` Rust crate for instant binary lookup and scope `scan_source_extensions` to known source folders.
