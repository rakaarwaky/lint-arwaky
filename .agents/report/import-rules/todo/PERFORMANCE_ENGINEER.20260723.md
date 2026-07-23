# Review Report: import-rules-lint-arwaky — Performance Engineer

## Summary

The `import-rules-lint-arwaky` crate enforces AES201-AES205 import boundary and cycle constraints. Performance issues stem from inner-loop string formatting in layer checking and redundant file reading across checkers.

## Performance Profile Analysis

- **Memory Allocations:** High — `format!("{}_", k)` called inside nested loops for every import statement.
- **I/O Overhead:** High — 4x redundant disk reads across separate checker implementations.

## Findings by Category

### CPU & Computational Efficiency

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 1 | 🔴 CRITICAL | `format!("{}_", k)` inside nested import loop | `capabilities_cycle_import_analyzer.rs:126` | Pre-compute `layer_prefixes` array once |
| 2 | 🟡 WARNING | Repeated string splitting for specialized layer names | `capabilities_cycle_import_analyzer.rs:105` | Use `&str` slicing without allocation |

### Memory Management & Leaks

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 3 | 🟡 WARNING | `Vec<String>` path conversions in trait bridge | `capabilities_cycle_import_analyzer.rs:37` | Pass `&[FilePath]` directly to `_scan` |
| 4 | 🟢 INFO | Dynamic string allocation in cycle violation messages | `capabilities_cycle_import_analyzer.rs:184` | Use static string formatting |

### I/O & Network Performance

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 5 | 🔴 CRITICAL | 4x redundant file reading across capability checkers | `capabilities_*_checker.rs` | Share `SourceContentVO` across checkers |

### Concurrency & Parallelism

*(N/A — Pure computational and I/O checkers)*

### Database & Query Performance

*(N/A — No database operations in this crate)*

## Violations (if any)

- Duplicate file reading violates I/O minimization guidelines.

## Action Items

- [ ] High Priority: Pre-compute `layer_prefixes` once outside file loop.
- [ ] High Priority: Refactor orchestrator to read files once and pass `SourceContentVO`.
- [ ] Medium Priority: Accept `&[FilePath]` in `_scan` to remove `Vec<String>` conversions.

## Fixed Code

```rust
// Fixed pre-computation of layer prefixes
let layer_prefixes: Vec<String> = layer_keys.iter().map(|k| format!("{}_", k)).collect();

// Inside import module loop:
let is_cross_layer_crate = if is_crate_import {
    let stripped = module_value
        .strip_prefix("crate::")
        .or_else(|| module_value.strip_prefix("lint_arwaky::"))
        .unwrap_or("");
    let first_segment = stripped.split("::").next().unwrap_or("");
    layer_prefixes.iter().any(|prefix| stripped.starts_with(prefix))
        || layer_keys.iter().any(|k| k == first_segment)
} else {
    false
};
```

---

## Detailed Audit Findings

# Performance Audit: import-rules-lint-arwaky

## Summary

**Crate:** import-rules-lint-arwaky
**Files audited:** 8 (src only, excluding tests/benches)
**Performance issues found:** 3 high impact, 2 moderate impact

---

## Critical Issues

### 1. High Heap Allocation in Cross-Layer Import Loop — HIGH IMPACT
**Location:** `capabilities_cycle_import_analyzer.rs` (_scan)

**Problem:** For every import module line processed across all files, `_scan()` evaluates `layer_keys.iter().any(|k| { let prefix = format!("{}_", k); stripped.starts_with(&prefix) })`. Calling `format!("{}_", k)` inside an inner loop over all imports creates a heap String allocation for *every layer* for *every import line*. In a project with 1000 files and 10 imports each, across 7 layer keys, this produces 70,000 temporary heap allocations per scan pass.

```rust
layer_keys.iter().any(|k| {
    let prefix = format!("{}_", k); // Allocation inside inner loop per import!
    stripped.starts_with(&prefix)
})
```

**Fix:** Pre-compute layer prefix strings once before scanning files:
`let layer_prefixes: Vec<String> = layer_map.values.keys().map(|k| format!("{}_", k)).collect();`

### 2. Duplicate Disk I/O Across Independent Checkers — HIGH IMPACT
**Location:** `capabilities_cycle_import_analyzer.rs`, `capabilities_import_forbidden_checker.rs`, `capabilities_dummy_import_checker.rs`, `capabilities_import_mandatory_checker.rs`

**Problem:** Each capability checker (`DependencyCycleAnalyzer`, `ImportForbiddenChecker`, `DummyImportChecker`, `ImportMandatoryChecker`) independently reads file content from disk using `read_file_generic(file)`. Scanning a 1000-file project results in 4,000 separate disk read calls instead of 1,000.

**Fix:** Pass pre-read `SourceContentVO` or AST structs down from the orchestrator so each file is read from disk exactly once.

### 3. Redundant `String` Vector Conversion in Trait Bridge — MODERATE IMPACT
**Location:** `capabilities_cycle_import_analyzer.rs` (scan, check_cycles)

**Problem:** `scan` and `check_cycles` convert `&[FilePath]` or `&FilePathList` into `Vec<String>` via `.map(|f| f.to_string()).collect()` before calling `_scan`. This allocates a new `String` for every file path twice.

```rust
let file_strs: Vec<String> = files.iter().map(|f| f.to_string()).collect();
```

**Fix:** Refactor `_scan` to accept `&[FilePath]` directly, avoiding intermediate `String` vector conversions.

---

## Moderate Issues

### 4. Uncached Layer Resolution Slices — MODERATE IMPACT
**Location:** `capabilities_cycle_import_analyzer.rs` (_scan)

**Problem:** `specialized.split('(').next().unwrap_or(...)` and `target_layer.split('(').next().unwrap_or(...)` perform string splitting and String allocation (`to_string()`) repeatedly for every layer detected.

**Fix:** Use `&str` slicing (`&specialized[..specialized.find('(').unwrap_or(specialized.len())]`) without heap allocation.

### 5. String Formatting for Violation Messages in Loop — LOW IMPACT
**Location:** `capabilities_cycle_import_analyzer.rs` (_scan)

**Problem:** `format!("Circular dependency between layers '{}' and '{}'...")` allocates violation reason strings dynamically for cycle edges without using static message templates.

**Fix:** Pre-format or use template messages where possible.

---

## Positive Findings

- Uses Tarjan's / DFS cycle detection algorithm in `utility_cycle_detector` for efficient $O(V + E)$ graph cycle analysis.
- Bails out early when rule is disabled (`if !config.enabled.value`).
- Checks exception lists (`rule.exceptions`) before running heavy regex/AST extraction.

---

## Estimated Impact

**Worst-case scenario:** Scanning a 1,000-file codebase generates >70,000 transient string allocations in `_scan` and performs 4,000 disk I/O calls. Total scan time increases by 300-600ms due to allocation and disk churn.

**Priority fix:** Pre-compute `layer_prefixes` once and pass pre-read `SourceContentVO` across checkers.
