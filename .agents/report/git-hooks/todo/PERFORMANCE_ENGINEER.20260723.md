# Review Report: git-hooks-lint-arwaky — Performance Engineer

## Summary

 around git diff inspection and pre-commit hook installation. The crate suffers from synchronous process spawning in async trait implementations and multi-subprocess trial loops during git diff detection.

## Performance Profile Analysis

- **Process Overhead:** Up to 6 sub-process spawns per diff check due to fallback variants.
- **Async Safety:** Low — synchronous process execution blocks Tokio worker threads.

## Findings by Category

### CPU & Computational Efficiency

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 1 | 🟡 WARNING | Heap string allocations for variant list | `capabilities_diff_checker.rs:109` | Lazily format variant strings |
| 2 | 🟢 INFO | Un-preallocated `HashSet` to `Vec` conversion | `capabilities_diff_checker.rs:126` | Use `Vec::with_capacity` |

### Memory Management & Leaks

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 3 | 🟡 WARNING | Redundant `changed_files` clones in `GitDiffResultVO` | `capabilities_diff_checker.rs:55` | Move vector ownership or use `Arc` |

### I/O & Network Performance

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 4 | 🔴 CRITICAL | Synchronous `git` command execution in `async_trait` | `capabilities_diff_checker.rs:22` | Use `tokio::process::Command` |
| 5 | 🔴 CRITICAL | Sequential probing of 4-6 git variants | `capabilities_diff_checker.rs:115` | Query `git merge-base` once |

### Concurrency & Parallelism

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 6 | 🟡 WARNING | Synchronous git child processes block Tokio threads | `utility_git_io.rs` | Offload git I/O to async subprocesses |

### Database & Query Performance

*(N/A — No database operations in this crate)*

## Violations (if any)

- Blocking I/O inside `async_trait` implementations (`IDiffProtocol`).

## Action Items

- [ ] High Priority: Refactor `utility_git_io` to use `tokio::process::Command`.
- [ ] High Priority: Replace variant loop with `git merge-base` or single `rev-parse`.
- [ ] Medium Priority: Avoid redundant cloning of `changed_files` in `GitDiffResultVO`.

## Fixed Code

```rust
// Fixed async git diff query
pub async fn collect_changed_files_async(project_path: &FilePath, default_branch: &str) -> FilePathList {
    let output = tokio::process::Command::new("git")
        .args(["diff", "--name-only", &format!("origin/{}...HEAD", default_branch)])
        .current_dir(&project_path.value)
        .output()
        .await;

    let mut changed_set = HashSet::new();
    if let Ok(out) = output {
        let stdout = String::from_utf8_lossy(&out.stdout);
        for line in stdout.lines() {
            if let Ok(fp) = FilePath::new(line) {
                changed_set.insert(fp);
            }
        }
    }
    let mut vec = Vec::with_capacity(changed_set.len());
    vec.extend(changed_set);
    FilePathList::new(vec)
}
```

---

## Detailed Audit Findings

# Performance Audit: git-hooks-lint-arwaky

## Summary

**Crate:** git-hooks-lint-arwaky
**Files audited:** 6 (src only, excluding tests/benches)
**Performance issues found:** 2 critical/high impact, 3 moderate impact

---

## Critical Issues

### 1. Synchronous Git Command Execution in Async Trait — HIGH IMPACT
**Location:** `capabilities_diff_checker.rs` (run_git_diff_check, get_diff, get_changed_files, get_default_branch)

**Problem:** `DiffChecker` implements `IDiffProtocol` (an `async_trait`). However, all methods delegate git operations to `git_io::run_git_command`, which spawns `std::process::Command` synchronously. When running diff checks in async contexts (e.g. MCP server or pre-commit pipeline), this blocks Tokio executor threads for each git invocation.

```rust
#[async_trait::async_trait]
impl IDiffProtocol for DiffChecker {
    async fn get_diff(&self, path: &FilePath) -> GitDiffResultVO {
        let default_branch = self.get_default_branch(path); // Synchronous git execution
        let changed_files = self.collect_changed_files(path, &default_branch); // Multiple sync git subprocesses
        // ...
    }
}
```

**Fix:** Use `tokio::process::Command` or wrap synchronous git execution in `tokio::task::spawn_blocking`.

### 2. Sequential Multi-Variant Git Command Probing — HIGH IMPACT
**Location:** `capabilities_diff_checker.rs` (collect_changed_files)

**Problem:** `collect_changed_files()` probes up to 4 git diff range variants (`origin/main...HEAD`, `HEAD...origin/main`, `main...HEAD`, `master...HEAD`) plus fallback commands (`diff HEAD`, `ls-files`). If early variants fail, this executes up to 6 separate `git` subprocess spawns sequentially.

```rust
let variants = [
    format!("origin/{}...HEAD", default_branch),
    format!("HEAD...origin/{}", default_branch),
    format!("{}...HEAD", default_branch),
    "master...HEAD".to_string(),
];
for variant in &variants {
    if self.try_variant(&mut changed_set, variant, project_path) {
        break;
    }
}
```

**Fix:** Verify if `origin/{default_branch}` exists using a single `git rev-parse` command or query `git merge-base` once before running diff.

---

## Moderate Issues

### 3. Repeated Heap String Formatting — MODERATE IMPACT
**Location:** `capabilities_diff_checker.rs` (collect_changed_files)

**Problem:** `variants` array constructs heap-allocated `format!("origin/{}...HEAD", default_branch)` strings on every single call to `collect_changed_files()`.

**Fix:** Format strings lazily inside `try_variant` loop only when attempting that specific variant, or use string formatting directly into buffer.

### 4. Duplicate Cloning of GitDiffResultVO File Vectors — MODERATE IMPACT
**Location:** `capabilities_diff_checker.rs` (get_diff)

**Problem:** `changed_files` is cloned twice into `GitDiffResultVO` (`modified: changed_files.clone()` and `all_files: changed_files.clone()`). Each clone copies all `FilePath` instances in the vector.

**Fix:** Share underlying `Arc<Vec<FilePath>>` or move vector ownership for the last consumer field.

### 5. Un-preallocated HashSet to Vec Conversion — LOW IMPACT
**Location:** `capabilities_diff_checker.rs` (collect_changed_files)

**Problem:** `FilePathList::new(changed_set.into_iter().collect())` collects from `HashSet` into `Vec` without specifying capacity hint `Vec::with_capacity(changed_set.len())`.

**Fix:** Use `let mut vec = Vec::with_capacity(changed_set.len()); vec.extend(changed_set); FilePathList::new(vec);`.

---

## Positive Findings

- Deduplicates changed files across git diff output using `HashSet<FilePath>`.
- Extension filtering in `get_diff()` skips non-lintable files early.
- Handles empty/detached HEAD gracefully with fallback `ls-files` check.

---

## Estimated Impact

**Worst-case scenario:** Running git diff checks on non-main branches with missing origin remote causes 6 sequential synchronous `git` process spawns, stalling the async executor for 200-500ms.

**Priority fix:** Replace sequential variant trial-and-error with a single `git merge-base` query, and make process spawning asynchronous.
