# Review Report: shared-lint-arwaky — Performance Engineer

## Summary

The `shared-lint-arwaky` crate contains shared value objects, contracts, and core utilities. Key performance risks involve synchronous command execution utilities and un-cached regex patterns in signature parsers.

## Performance Profile Analysis

- **Concurrency:** Suboptimal — `run_command` uses `std::process::Command` synchronously.
- **CPU Efficiency:** Moderate regex recompilation overhead in signature utilities.

## Findings by Category

### CPU & Computational Efficiency

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 1 | 🔴 CRITICAL | Dynamic regex recompilation in signature parser | `common/utility_signature_parser.rs` | Cache compiled patterns with `OnceLock` |
| 2 | 🟡 WARNING | Linear search over un-compiled scope rules | `common/utility_scope_matcher.rs` | Use `globset::GlobSet` for O(1) matching |
| 3 | 🟢 INFO | Unconditional string allocation in path normalizer | `common/utility_path_normalization.rs` | Check `contains('\\')` before `replace` |

### Memory Management & Leaks

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 4 | 🔴 CRITICAL | Unbounded memory reading in `read_file_generic` | `common/utility_file_handler.rs:12` | Enforce maximum file size check (e.g. 10MB) |
| 5 | 🟡 WARNING | `FilePath` clones copy string content | `common/taxonomy_path_vo.rs` | Consider `Arc<str>` or `Cow<str>` for VOs |

### I/O & Network Performance

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 6 | 🔴 CRITICAL | Synchronous `run_command` blocks async threads | `common/utility_command_runner.rs:8` | Add `run_command_async` via `tokio::process` |

### Concurrency & Parallelism

*(N/A — Utility library)*

### Database & Query Performance

*(N/A — No database operations in this crate)*

## Violations (if any)

- Synchronous blocking functions exported for use in async contexts.

## Action Items

- [ ] High Priority: Add `run_command_async` in `utility_command_runner.rs`.
- [ ] High Priority: Add file size sanity check in `read_file_generic`.
- [ ] Medium Priority: Cache regexes in `utility_signature_parser.rs`.

## Fixed Code

```rust
// Fixed async command runner utility
pub async fn run_command_async(cmd: &str, args: &[&str]) -> std::io::Result<(String, String, bool)> {
    let output = tokio::process::Command::new(cmd)
        .args(args)
        .output()
        .await?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let success = output.status.success();
    Ok((stdout, stderr, success))
}
```

---

## Detailed Audit Findings

# Performance Audit: shared-lint-arwaky

## Summary

**Crate:** shared-lint-arwaky
**Files audited:** 41 (src only, excluding tests/benches)
**Performance issues found:** 3 high impact, 3 moderate impact

---

## Critical Issues

### 1. Synchronous Blocking Subprocess Command Execution — HIGH IMPACT
**Location:** `common/utility_command_runner.rs` (run_command)

**Problem:** `run_command` executes external commands synchronously using `std::process::Command::new(...)`. Because `shared` is the fundamental utility dependency for all crates, calling `run_command` inside async contexts across `git-hooks`, `external-lint`, or `mcp-server` stalls Tokio async worker threads.

```rust
pub fn run_command(cmd: &str, args: &[&str]) -> (String, String, bool) {
    let output = std::process::Command::new(cmd).args(args).output(); // Synchronous blocking call!
    // ...
}
```

**Fix:** Add `run_command_async` using `tokio::process::Command` alongside `run_command` for use in async contexts.

### 2. Unbounded Memory Loading in Generic File Handler — HIGH IMPACT
**Location:** `common/utility_file_handler.rs` (read_file_generic)

**Problem:** `read_file_generic` reads entire target files into memory as `String` without inspecting file size limits or using memory mapping (`memmap2`). Reading very large files (e.g. 50MB generated data files or minified bundles) causes spike allocations and potential OOM errors.

```rust
pub fn read_file_generic(path: &str) -> std::io::Result<String> {
    std::fs::read_to_string(path) // Unbounded memory load
}
```

**Fix:** Add file size check (e.g. max 10MB) before reading into `String`, or return an error for oversized non-source files.

### 3. Uncached Regular Expressions in Signature Parser — MODERATE/HIGH IMPACT
**Location:** `common/utility_signature_parser.rs`

**Problem:** Function and class signature parsers compile regex patterns dynamically on invocation without caching them across calls. When scanning hundreds of source files during compliance checking, compiling identical regexes repeatedly wastes CPU cycles.

```rust
let re = Regex::new(r"fn\s+([a-zA-Z0-9_]+)").unwrap(); // Recompiled per function call!
```

**Fix:** Cache compiled regexes using `std::sync::OnceLock` or `once_cell::sync::Lazy`.

---

## Moderate Issues

### 4. O(N) Linear Search in Scope Matcher — MODERATE IMPACT
**Location:** `common/utility_scope_matcher.rs` (is_match)

**Problem:** `utility_scope_matcher` evaluates glob/scope rules by linearly searching rule arrays for every single file path checked during scanning.

**Fix:** Pre-compile glob patterns into a unified `globset::GlobSet` for O(1) matching performance.

### 5. Transient String Allocations in Path Normalization — LOW/MODERATE IMPACT
**Location:** `common/utility_path_normalization.rs` (normalize_path)

**Problem:** `normalize_path` replaces `/` and `\` characters by allocating new `String` instances (`path.replace('\\', "/")`) even when paths are already normalized.

**Fix:** Check `if path.contains('\\')` before invoking `path.replace()`.

### 6. Value Object Cloning Overhead — LOW IMPACT
**Location:** `common/taxonomy_path_vo.rs` (FilePath), `common/taxonomy_common_vo.rs`

**Problem:** `FilePath` stores `value: String` and is frequently cloned throughout the pipeline.

**Fix:** Consider using `Arc<str>` or `Cow<'a, str>` for zero-copy file path sharing across lint stages.

---

## Positive Findings

- Domain Value Objects (VOs) encapsulate validation logic cleanly (`FilePath`, `Severity`, `LintResult`, `ScanReport`).
- Layer detector (`utility_layer_detector`) uses efficient slice matching for prefix categorization.

---

## Estimated Impact

**Worst-case scenario:** Calling `run_command` in async contexts stalls worker threads. Parsing signatures in 1,000 files re-compiles regexes 1,000 times, adding 200-400ms CPU latency.

**Priority fix:** Add `run_command_async` using `tokio::process::Command` and cache regular expressions in `utility_signature_parser`.
