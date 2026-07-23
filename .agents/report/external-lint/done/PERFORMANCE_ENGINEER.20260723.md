# Review Report: external-lint-lint-arwaky — Performance Engineer

## Summary

The `external-lint-lint-arwaky` crate orchestrates 9 external linter adapters (Clippy, Rustfmt, Cargo Audit, Ruff, Mypy, Bandit, ESLint, Prettier, TSC). While `future::join_all` is used for concurrency, all process execution and directory detection use synchronous std API calls inside async functions, causing Tokio worker thread starvation.

## Performance Profile Analysis

- **Concurrency:** Suboptimal. Processes run via `std::process::Command` inside async tasks, occupying worker threads.
- **I/O Overhead:** High during discovery due to synchronous recursive `std::fs::read_dir`.

## Findings by Category

### CPU & Computational Efficiency

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 1 | 🟡 WARNING | Unnecessary `Cow<str>` allocation per directory entry | `agent_external_lint_orchestrator.rs:51` | Use `path.file_name().and_then(|n| n.to_str())` |
| 2 | 🟢 INFO | Un-preallocated result vector extension | `agent_external_lint_orchestrator.rs:136` | Use `Vec::with_capacity` based on result sum |

### Memory Management & Leaks

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 3 | 🟡 WARNING | Transient String allocations during output parsing | `capabilities_*_adapter.rs` | Parse stdout slices (`&str`) directly |

### I/O & Network Performance

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 4 | 🔴 CRITICAL | Synchronous `std::fs::read_dir` in async task | `agent_external_lint_orchestrator.rs:43` | Use `tokio::fs::read_dir` or `spawn_blocking` |
| 5 | 🔴 CRITICAL | Synchronous `std::process::Command` in async adapters | `capabilities_*_adapter.rs` | Use `tokio::process::Command` |

### Concurrency & Parallelism

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 6 | 🟡 WARNING | Tokio worker thread blocking during long-running linters | `agent_external_lint_orchestrator.rs:112` | Async process spawning avoids blocking Tokio runtime |

### Database & Query Performance

*(N/A — No database operations in this crate)*

## Violations (if any)

- Async functions block the Tokio executor thread with synchronous file and process I/O.

## Action Items

- [ ] High Priority: Replace `std::process::Command` with `tokio::process::Command` across all 9 adapters.
- [ ] High Priority: Replace synchronous `detect_languages` with `tokio::fs::read_dir`.
- [ ] Medium Priority: Pre-allocate result vectors in `scan_all`.

## Fixed Code

```rust
// Fixed async process execution in capabilities_rs_clippy_adapter.rs
pub async fn scan(&self, path: &FilePath) -> Result<LintResultList, AdapterError> {
    let output = tokio::process::Command::new("cargo")
        .args(["clippy", "--message-format=json"])
        .current_dir(&path.value)
        .output()
        .await
        .map_err(|e| AdapterError::execution_failed("clippy", e.to_string()))?;
    // ...
}
```

---

## Detailed Audit Findings

# Performance Audit: external-lint-lint-arwaky

## Summary

**Crate:** external-lint-lint-arwaky
**Files audited:** 17 (src only, excluding tests/benches)
**Performance issues found:** 3 critical/high impact, 2 moderate impact

---

## Critical Issues

### 1. Synchronous Blocking I/O in Async Context — HIGH IMPACT
**Location:** `agent_external_lint_orchestrator.rs` (detect_languages)

**Problem:** `detect_languages()` performs a recursive filesystem traversal using `std::fs::read_dir` synchronously inside `scan_all()`, which is an `async fn` executed on the Tokio runtime. In large monorepos with tens of thousands of files, this synchronous I/O blocks Tokio worker threads, stalling concurrent tasks.

```rust
fn detect_languages(dir: &std::path::Path, ...) -> std::io::Result<()> {
    let entries = match std::fs::read_dir(dir) { ... };
    // synchronous recursion
}
```

**Fix:** Use `tokio::fs::read_dir` for async directory scanning, or wrap the detection call in `tokio::task::spawn_blocking`.

### 2. Synchronous Process Command Execution in Adapters — HIGH IMPACT
**Location:** `capabilities_rs_fmt_adapter.rs`, `capabilities_py_ruff_adapter.rs`, `capabilities_js_eslint_adapter.rs`, `capabilities_rs_clippy_adapter.rs`, `capabilities_rs_audit_adapter.rs`, etc.

**Problem:** Adapters invoke external tool commands using `std::process::Command::new(...)` synchronously inside `async fn scan()`. Even though adapters run concurrently via `future::join_all`, each spawned process blocks its underlying Tokio executor thread during linter execution (which can take several seconds for tools like `mypy` or `cargo audit`).

```rust
let output = std::process::Command::new("cargo")
    .args(["clippy", "--message-format=json"])
    .output()?; // Synchronous blocking call
```

**Fix:** Replace `std::process::Command` with `tokio::process::Command` so that subprocess execution yields the Tokio thread while waiting for linter output.

### 3. Redundant String Allocations in Output Parsing — MODERATE IMPACT
**Location:** `capabilities_py_mypy_adapter.rs`, `capabilities_js_eslint_adapter.rs`, `capabilities_rs_clippy_adapter.rs`

**Problem:** Linter stdout output parsing splits output lines and calls `.to_string()`, `format!`, or `PathBuf::from()` per line without pre-allocating result vectors. For standard lint reports with hundreds of messages, this results in hundreds of transient heap allocations.

**Fix:** Pre-allocate `Vec::with_capacity(estimated_lines)` and borrow string slices (`&str`) during regex/JSON parsing before creating final `LintResult` VOs.

---

## Moderate Issues

### 4. Unbounded Cow Allocation in Directory Filtering — LOW/MODERATE IMPACT
**Location:** `agent_external_lint_orchestrator.rs` (detect_languages)

**Problem:** `path.file_name().unwrap().to_string_lossy()` creates a `Cow<str>` heap allocation for every directory inspected during traversal.

**Fix:** Compare file name using `path.file_name().and_then(|n| n.to_str())` directly without lossy String conversion.

### 5. Un-preallocated Vector Extension — LOW IMPACT
**Location:** `agent_external_lint_orchestrator.rs` (scan_all)

**Problem:** `for values in results.into_iter().flatten() { all.extend(values); }` extends `all` vector without setting capacity based on total results.

**Fix:** Calculate total capacity hint `let total: usize = results.iter().flatten().map(|v| v.len()).sum(); let mut all = Vec::with_capacity(total);`.

---

## Positive Findings

- Runs detected linter adapters concurrently using `futures::future::join_all`.
- Bailout mechanisms handle missing binaries gracefully with warning logs instead of crashing.
- Language detection stops scanning early once Rust, Python, and JavaScript/TypeScript markers are all found (`if *has_rs && *has_py && *has_js { break; }`).

---

## Estimated Impact

**Worst-case scenario:** Monorepo with Rust, Python, and Node.js codebases. Running 9 external linters with synchronous `std::process::Command` blocks 9 Tokio thread pool slots simultaneously and stalls background tasks for 5-15 seconds.

**Priority fix:** Convert all adapters to `tokio::process::Command`. This eliminates Tokio thread starvation during external linter execution.
