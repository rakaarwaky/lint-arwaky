# Review Report: maintenance-lint-arwaky (Crate 12) — Performance Engineer

## Summary

**Crate:** maintenance-lint-arwaky
**Files audited:** 5 (src only, excluding tests/benches)
**Performance issues found:** 5 significant, 3 moderate

---

## Critical Issues

### 1. Recursive File Walk Without Parallelism — HIGH IMPACT
**Location:** `agent_maintenance_orchestrator.rs` (walk_dir, find_cache_dirs)

**Problem:** Both walk_dir and find_cache_dirs use synchronous recursive filesystem traversal with std::fs::read_dir. For large projects with 100k+ files, this blocks the async runtime and runs single-threaded. The walk_dir function filters node_modules, target, .git, .venv but still recurses deeply.

```rust
fn walk_dir(dir: &Path, py_files: &mut Vec<PathBuf>) {
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() { /* recurse */ }
            else if path.is_file() && extension == "py" { /* collect */ }
        }
    }
}
```

**Fix:** Use walkdir crate with parallel iteration (ParallelWalkDir), or rayon for concurrent directory processing. Add max_depth limit to prevent deep recursion on large projects.

### 2. Synchronous Command Execution in Async Context — HIGH IMPACT
**Location:** `agent_maintenance_orchestrator.rs` (update, doctor), `capabilities_maintenance_checker.rs` (diagnose_toolchain)

**Problem:** Multiple async methods spawn synchronous subprocesses that block the async runtime:
- `std::process::Command::new("pip").args(["show", "lint-arwaky"]).output()` — blocks tokio runtime
- `std::process::Command::new("which").arg(adapter).output()` — called 4 times sequentially in doctor()
- `proc_io::run_command(name, args)` — called ~15 times in diagnose_toolchain

**Fix:** Use tokio::process::Command for async subprocess execution. Batch tool checks into a single command where possible (e.g., `which ruff mypy bandit radon`).

### 3. Sequential External Tool Checks — MODERATE IMPACT
**Location:** `agent_maintenance_orchestrator.rs` (update)

**Problem:** The update() method runs pip install --upgrade sequentially for each adapter (ruff, mypy, bandit, radon). Each command blocks and waits for completion before starting the next.

```rust
for adapter in &adapters {
    let _ = std::process::Command::new("pip")
        .args(["install", "--upgrade", adapter])
        .output();
}
```

**Fix:** Run all upgrades in parallel using tokio::spawn, or use a single `pip install --upgrade ruff mypy bandit radon` call.

---

## Moderate Issues

### 4. String Allocation in Tool Status Checks — MODERATE IMPACT
**Location:** `capabilities_maintenance_checker.rs` (diagnose_toolchain)

**Problem:** check_tool closure allocates new Strings for every tool check:
- `name.to_string()` — 15+ allocations per diagnostic run
- `version` extraction via `stdout.lines().next()` creates temporary String
- Multiple ToolStatus allocations per call

**Fix:** Use &'static str where possible, or pre-allocate with Vec::with_capacity(20). Consider returning a struct with &str fields during construction.

### 5. Redundant File Existence Checks — MODERATE IMPACT
**Location:** `agent_maintenance_orchestrator.rs` (doctor), `capabilities_maintenance_checker.rs` (diagnose_toolchain)

**Problem:** Local tool detection checks file existence separately from execution:
```rust
if shared::common::utility_file_handler::is_file(eslint_local) {
    // use local version
} else {
    check_tool("eslint", &["--version"], false); // runs again
}
```

**Fix:** Check and capture path in single operation, or use which crate for PATH resolution.

### 6. HashMap::new() Without Capacity — LOW IMPACT
**Location:** `agent_maintenance_orchestrator.rs` (doctor)

**Problem:** `adapter_statuses: HashMap<AdapterName, String> = HashMap::new();` grows without pre-size. Known to have 4 entries.

**Fix:** `HashMap::with_capacity(4)` or use a fixed-size array/tuple.

### 7. Line-by-Line TOML Parsing — LOW IMPACT
**Location:** `capabilities_maintenance_checker.rs` (run_dependency_report)

**Problem:** Parses Cargo.toml and pyproject.toml line-by-line with manual state machine (in_deps flag). Inefficient for large files and doesn't handle edge cases (comments, whitespace).

**Fix:** Use toml crate for proper parsing. Or at least use toml::Value for pre-parsed access.

---

## Positive Findings

- Tool executor adapter uses Command::output() efficiently for single executions
- Maintenance container is clean — no performance issues, just wiring
- Security scan uses structured JSON parsing (serde_json) — efficient over line-by-line
- Dependency report early-exits on first matching file type (Cargo.lock > pyproject.toml > requirements.txt)
- Cache directory cleaning uses direct path iteration — no unnecessary abstractions

---

## Estimated Impact

**Worst-case scenario (large monorepo):** The recursive walk_dir function on a Python project with 50k+ files takes ~2-5 seconds single-threaded. Combined with 15+ sequential subprocess calls in diagnose_toolchain, total doctor() call can take 5-10 seconds.

**Priority fix:** Replace sync subprocess calls with tokio::process::Command and use walkdir crate for parallel directory traversal. This would reduce diagnostic time by 60-80%.
