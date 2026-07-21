# Review Report: bypass-checker — Backend Developer

## Summary

The `BypassChecker` capability (AES304) had two critical bugs: (1) `has_safe_unwrap_variant` failed to detect `.unwrap()` calls when the receiver was a normal identifier (e.g., `some_option.unwrap()`), causing false negatives; (2) the quick-scan optimization at the top of `check_bypass_comments` did not account for `#[allow]` attributes, `assert false`, or language-specific patterns, causing the main detection loop to be skipped entirely for those cases. Both bugs have been fixed. The test suite was expanded from 18 to 46 tests covering all detection paths, edge cases, and skip logic.

## Findings by Category

### Architecture & Layer Compliance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🟢 INFO | `b_is_ident` was a private helper only used by `has_safe_unwrap_variant`; after the fix it became dead code | `capabilities_check_bypass_checker.rs:350` | Removed (done) |

### Security

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | No security issues found | — | — |

### Performance

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| 1 | 🔴 CRITICAL | Quick-scan optimization skipped `#[allow]` attributes, `assert false`, and language-specific patterns — the main loop was never reached for these violations | `capabilities_check_bypass_checker.rs:71-84` | Extended quick-scan to check `starts_with_allow_attr()` and language-specific phrases (done) |

### Error Handling

| # | Severity | Issue | Location | Recommendation |
|---|----------|-------|----------|----------------|
| — | — | No error handling issues found | — | — |

## Violations (if any)

No AES layer violations introduced. Pre-existing AES402 violations in `shared/` contract files are unrelated to this change.

## Action Items

- [x] 🔴 Fix `has_safe_unwrap_variant` — remove `!b_is_ident(bytes[i - 1])` guard that prevented detection of normal method calls like `some_option.unwrap()`
- [x] 🔴 Fix quick-scan optimization — add `#[allow]` attribute check and language-specific pattern checks (`assert false`, `raise `, `throw new`) so the main detection loop is reached
- [x] 🟡 Remove dead `b_is_ident` function after fix
- [x] 🟡 Expand test suite from 18 to 46 tests covering all detection paths
- [x] 🟡 Fix test names that had incorrect expectations (e.g., `test_python_raise_not_detected` → `test_python_raise_notimplementederror`)
- [ ] 🟢 Consider removing `FIXME`/`HACK`/`XXX` from default forbidden patterns — these are code quality markers in comments, not runtime bypass annotations, and are correctly skipped by the comment filter

## Fixed Code

### Fix 1: `has_safe_unwrap_variant` — removed incorrect identifier boundary check

**Before:**
```rust
if bytes[i..].starts_with(b".unwrap") && (i == 0 || !b_is_ident(bytes[i - 1])) {
```

**After:**
```rust
if bytes[i..].starts_with(b".unwrap") {
```

**Why:** The `.` prefix in `.unwrap()` is sufficient to identify a method call. The `!b_is_ident(bytes[i - 1])` check required the character before `.` to be a non-identifier, which is almost never true for normal method calls (e.g., `some_option.unwrap()` has `_` before `.`). This caused the function to return `true` (all safe) even when unsafe `.unwrap()` calls were present.

### Fix 2: Quick-scan optimization — added missing pattern checks

**Before:**
```rust
let has_bypass_token = content.lines().any(|line| {
    let trimmed = line.trim();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }
    let lc = line.to_lowercase();
    effective_patterns.iter().any(|p| lc.contains(p.as_str()))
        || lc.contains("raise ")
        || lc.contains("throw new")
});
```

**After:**
```rust
let language = code_analysis_language_from_file(file);
let has_bypass_token = content.lines().any(|line| {
    let trimmed = line.trim();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") || trimmed.starts_with('*') {
        return false;
    }
    let lc = line.to_lowercase();
    if effective_patterns.iter().any(|p| lc.contains(p.as_str())) {
        return true;
    }
    if starts_with_allow_attr(trimmed) {
        return true;
    }
    match language {
        Language::Python => lc.contains("raise ") || lc.contains("assert false"),
        Language::JavaScript | Language::TypeScript => lc.contains("throw new"),
        _ => false,
    }
});
```

**Why:** The original quick-scan only checked config patterns and two hardcoded phrases (`raise `, `throw new`). It missed `#[allow]` attributes (checked via `starts_with_allow_attr`), `assert false` (Python), and the `language` variable was computed later in the function. This caused the main detection loop to be skipped for files containing only `#[allow]` or `assert false` violations.

### Fix 3: Removed dead `b_is_ident` function

The `b_is_ident` helper was only used by the removed `!b_is_ident(bytes[i - 1])` check and became dead code after Fix 1.
