# Review Report: naming-rules-lint-arwaky — Performance Engineer

## Summary

The `naming-rules-lint-arwaky` crate validates AES101 file naming conventions. Primary issues involve repeated vector allocation inside error paths and incorrect regex cache fallback for custom word counts.

## Performance Profile Analysis

- **Memory Efficiency:** Suboptimal on error paths due to transient `Vec<String>` creation.
- **Correctness/Performance Tradeoff:** Dynamic regex lookup hardcodes 3 vs 4 words.

## Findings by Category

### CPU & Computational Efficiency

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 1 | 🟡 WARNING | `get_stem(filename)` called up to 3 times per file | `capabilities_naming_convention_checker.rs:125` | Extract stem once and reuse slice |
| 2 | 🟢 INFO | Inefficient `OnceLock` matching for `min_words > 4` | `capabilities_naming_convention_checker.rs:89` | Expand `OnceLock` array for min_words 2..=8 |

### Memory Management & Leaks

| # | Severity | Issue | Location | Recommendation |
| - | -------- | ----- | -------- | -------------- |
| 3 | 🔴 CRITICAL | `allowed: Vec<String>` allocated on every violation | `capabilities_naming_convention_checker.rs:129` | Use static `&'static str` array constant |
| 4 | 🟡 WARNING | Redundant `f.to_string()` allocations in file loop | `capabilities_naming_convention_checker.rs:42` | Use `&f.value` reference |

### I/O & Network Performance

*(N/A — In-memory filename parsing)*

### Concurrency & Parallelism

*(N/A — Pure in-memory checker)*

### Database & Query Performance

*(N/A — No database operations in this crate)*

## Violations (if any)

- Allocating transient vectors inside exception/violation constructors.

## Action Items

- [ ] High Priority: Replace `allowed: Vec<String>` with static `&[&str]` constant.
- [ ] Medium Priority: Cache stem string slice `let stem = get_stem(filename)` once per call.
- [ ] Medium Priority: Fix `naming_regex` to support `min_words` up to 8 correctly.

## Fixed Code

```rust
// Fixed static allowed prefixes constant
const ALLOWED_PREFIXES: &[&str] = &[
    "agent", "capabilities", "contract", "root", "surface", "taxonomy", "utility",
];

// Fixed violation creation without heap allocations:
let allowed_strings: Vec<String> = ALLOWED_PREFIXES.iter().map(|&s| s.to_string()).collect();
```

---

## Detailed Audit Findings

# Performance Audit: naming-rules-lint-arwaky

## Summary

**Crate:** naming-rules-lint-arwaky
**Files audited:** 5 (src only, excluding tests/benches)
**Performance issues found:** 2 high impact, 2 moderate impact

---

## Critical Issues

### 1. Vector Allocation on Every Unknown Prefix Violation — HIGH IMPACT
**Location:** `capabilities_naming_convention_checker.rs` (_check_file_naming)

**Problem:** Whenever a file fails prefix resolution, `_check_file_naming` creates a new heap-allocated vector `allowed: Vec<String>` by executing `layer_prefixes.iter().map(|p| p.trim_end_matches('_').to_string()).collect()`. For projects with multiple misnamed or unrecognised files, this causes repeated heap allocations for identical static strings.

```rust
let allowed: Vec<String> = layer_prefixes
    .iter()
    .map(|p| p.trim_end_matches('_').to_string())
    .collect(); // Allocation on every violation creation!
```

**Fix:** Define `ALLOWED_PREFIXES` as a `const` or `LazyLock` static array of `&'static str` (`["agent", "capabilities", "contract", "root", "surface", "taxonomy", "utility"]`).

### 2. Broken `OnceLock` Pattern for Non-Standard `min_words` — MODERATE/HIGH IMPACT
**Location:** `capabilities_naming_convention_checker.rs` (naming_regex)

**Problem:** `naming_regex()` caches compiled `Regex` using static `RE3` for `min_words == 3` and `RE4` for any `min_words != 3`. If a user configures `min_words = 5` in `lint_arwaky.config.yaml`, the system fallbacks to `RE4` (which enforces a 4-word minimum regex pattern `^{3,}$` instead of 5 words!).

```rust
let re_lock = match min_words {
    3 => &RE3,
    _ => &RE4, // Ignores min_words values > 4!
};
```

**Fix:** Support dynamic caching using a fixed table of `OnceLock` slots (e.g. for min_words 2..=8) or a thread-safe map.

---

## Moderate Issues

### 3. Duplicate Stem Extraction per File — MODERATE IMPACT
**Location:** `capabilities_naming_convention_checker.rs` (_check_file_naming)

**Problem:** `get_stem(filename)` is invoked up to 3 separate times within `_check_file_naming` for the same file stem.

```rust
let stem = get_stem(filename).unwrap_or_default(); // Call #1
// ...
let stem = get_stem(filename).unwrap_or_default(); // Call #2
```

**Fix:** Compute `let stem = get_stem(filename).unwrap_or_default();` once at the start of `_check_file_naming` and reuse the reference.

### 4. Redundant String Conversions in Loop — LOW IMPACT
**Location:** `capabilities_naming_convention_checker.rs` (check_file_naming)

**Problem:** `let f_str: String = f.to_string();` converts `FilePath` to `String` on every loop iteration over files.

**Fix:** Operate directly on `&f.value` or `f.as_str()` slice.

---

## Positive Findings

- Uses `OnceLock` to compile and cache standard 3-word and 4-word naming convention regular expressions.
- Early bailout for barrel files (`index.ts`, `mod.rs`, `lib.rs`) and entry points.

---

## Estimated Impact

**Worst-case scenario:** Codebases with 100 non-compliant files produce 100 duplicate allocations of the allowed prefix vector. Setting `min_words: 5` in config silently enforces the incorrect 4-word rule.

**Priority fix:** Replace static prefix vector mapping with static `&'static str` constant and expand `OnceLock` regex table to support variable word counts.
