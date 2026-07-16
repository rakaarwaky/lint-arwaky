---
name: fix-error-handling-rust
version: 1.0.0
category: refactoring
tags: [aes, error-handling, result, unwrap, propagation, rust]
triggers:
  - "fix error handling rust"
  - "replace unwrap default rust"
  - "proper error propagation rust"
  - "error propagation rust"
dependencies: []
related:
  - clean-bloat
  - fix-bypass-comments
---

# fix-error-handling-rust

## Rules

- Never silently discard errors with `unwrap_or_default()` in capabilities or agent layers
- Infrastructure layer may use `unwrap_or_default()` ONLY when the error is genuinely unrecoverable (e.g., missing optional config)
- All public methods MUST return `Result<T, E>` where `E` is descriptive
- Violations go through `LintResult::new_arch()` — distinguish "file read failed" from "import check failed"

## Purpose

Replace silent error swallowing (`unwrap_or_default()`, `.map_err(|e| format!("{:?}", e))` + unwrap) with proper error propagation or explicit `LintResult` failure records.

## When to Use

- Capabilities or agent files use `unwrap_or_default()` to discard errors
- Infrastructure returns `Result` but capabilities unwrap it silently
- No custom error types exist for domain-specific failures
- Error messages are generic (e.g., `{:?}` formatting discards detail)

## The Fundamental Question

> **"Where did the error go?"**

If the answer is "silently discarded" -> **Fix error handling**

## Detection Patterns

### Silent Swallowing (Fix)

```rust
// [FORBIDDEN] Error silently discarded
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()  // Error thrown away
}

// [FORBIDDEN] Error detail lost
.cycle_check()
.map_err(|e| format!("{:?}", e))  // Debug formatting loses context
.unwrap_or_default()               // Silently discarded
```

### Silent Fallback (Fix)

```rust
// [FORBIDDEN] Empty string fallback hides failure
fn str_or(val: Option<String>) -> String {
    val.unwrap_or_else(String::new)  // No indication of failure
}
```

### Proper Patterns (Use)

```rust
// [OK] Explicit error propagation
fn parse_file(path: &FilePath) -> Result<Content, ParseError> {
    std::fs::read_to_string(path).map_err(ParseError::Io)
}

// [OK] LintResult for check failures (not IO failures)
fn check_imports(...) -> Vec<LintResult> {
    // File read failure -> propagate Result
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return vec![LintResult::new_arch(
            "PARSE_ERROR", &format!("Cannot read: {}", e), path.clone()
        )],
    };
    // Import check failure -> LintResult (expected outcome)
    // ...
}
```

## Layer-Specific Rules

### Capabilities

```rust
// [FORBIDDEN] BEFORE
fn analyze(&self, path: &FilePath) -> Analysis {
    let content = match read_file(path) {
        Ok(c) => c,
        Err(_) => return Analysis::empty(),  // Swallowed error
    };
    // ...
}

// [OK] AFTER
fn analyze(&self, path: &FilePath) -> Result<Analysis, FileReadError> {
    let content = read_file(path)?;  // Propagate
    // ...
    Ok(analysis)
}
```

### Agent

```rust
// [FORBIDDEN] BEFORE
let result = checker.check().unwrap_or_default();

// [OK] AFTER
let result = match checker.check() {
    Ok(r) => r,
    Err(e) => {
        violations.push(LintResult::new_arch(
            "CHECK_ERROR", &e.to_string(), file_path.clone()
        ));
        continue;
    }
};
```

### Infrastructure

Infrastructure may use `unwrap_or_default()` for genuinely optional values:

```rust
// [OK] Optional config with sensible default
fn get_timeout(config: &Option<Config>) -> u32 {
    config.as_ref().and_then(|c| c.timeout).unwrap_or(30)
}
```

## Workflow

### Step 1: Find Silent Error Discards

Search for `unwrap_or_default()`, `.map_err.*unwrap_or_default`, and silent fallbacks.

### Step 2: Categorize Errors

- **IO errors** (file read, network) -> propagate with `Result` or return `LintResult::new_arch()`
- **Logic errors** (validation, parsing) -> propagate with `Result` + custom error type
- **Optional values** (config, defaults) -> keep `unwrap_or_default()` if intentional

### Step 3: Define Error Types

Create descriptive error types (e.g., `ParseError`, `FileReadError`, `CheckError`).

### Step 4: Replace Silent Discards

- IO failures -> propagate or explicit `LintResult` record
- Logic failures -> `Result<T, E>` with descriptive `E`
- Optional values -> keep if intentional, document why

### Step 5: Verify

Run clippy and tests. No silent error discards should remain in capabilities or agent layers.

## Quick Commands

```bash
# Find unwrap_or_default() calls
rg "unwrap_or_default\(\)" crates/import-rules/src/

# Find map_err + unwrap patterns
rg "\.map_err.*unwrap" crates/import-rules/src/

# Run clippy after fixes
cargo clippy -p import_rules -- -D warnings
```
