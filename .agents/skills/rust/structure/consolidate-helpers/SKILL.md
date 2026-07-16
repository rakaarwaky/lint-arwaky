---
name: consolidate-helpers-rust
version: 1.0.0
category: refactoring
tags: [aes, helpers, deduplication, shared, utilities, rust]
triggers:
  - "consolidate helpers rust"
  - "deduplicate helpers rust"
  - "extract shared utility rust"
  - "shared utilities rust"
dependencies: []
related:
  - fix-magic-constant
  - clean-bloat
---

# consolidate-helpers-rust

## Rules

- Extract duplicated helper functions into a single shared module (`src/common/` or `src/utilities/`)
- Each helper function MUST have exactly one definition
- Shared helpers must be `pub(crate)` — not public API, crate-internal only
- Never create a shared module for a function that exists only once

## Purpose

Extract duplicated helper functions from multiple files into a single shared utilities module. Eliminates copy-paste bugs and ensures a single source of truth.

## When to Use

- Same helper function appears in 2+ files (e.g., `filepath_or_default` in both container and orchestrator)
- Helper functions are trivially identical (same signature, same body)
- User asks to deduplicate or consolidate helpers

## The Fundamental Question

> **"Is this helper function defined in more than one file?"**

If yes -> **Extract into shared module**

## Detection Patterns

### Duplicated Free Functions (Consolidate)

```rust
// In root_import_rules_container.rs
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

// In agent_import_orchestrator.rs (DUPLICATE)
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}
```

### Duplicated String Helpers (Consolidate)

```rust
// In agent_import_orchestrator.rs
fn str_or(val: Option<String>) -> String {
    val.unwrap_or_else(String::new)
}
// If this also exists elsewhere -> consolidate
```

### Hardcoded Layer Prefixes in Multiple Files (Consolidate)

```rust
// In LayerDetectionAnalyzer (line ~50)
let prefixes = vec!["taxonomy_", "contract_", "service_", ...];

// In ImportParserAdapter (line ~30) — SAME vec! -> consolidate into shared constant
```

## Shared Module Structure

```rust
// src/common/mod.rs
pub(crate) fn filepath_or_default<T>(result: Result<FilePath, T>) -> FilePath {
    result.unwrap_or_default()
}

pub(crate) fn str_or(val: Option<String>) -> String {
    val.unwrap_or_else(String::new)
}

pub(crate) const LAYER_PREFIXES: &[&str] = &[
    "taxonomy_",
    "contract_",
    "service_",
    // ... all prefixes from single source of truth
];
```

## Workflow

### Step 1: Find Duplicated Helpers

Search for identical free functions across files. Look for:

- Same function name in different files
- Same function body (even if names differ slightly)

### Step 2: Verify Identity

Compare function signatures and bodies. Only consolidate if IDENTICAL.

### Step 3: Create Shared Module

Create `src/common/mod.rs` (or use existing utilities module). Add `pub(crate)` helper functions.

### Step 4: Replace Duplicates

- In each file, replace the local helper with `use crate::common::*;` or specific import
- Keep the function call signature identical (no callers need to change)

### Step 5: Verify

Run clippy and tests. No duplicated helpers should remain.

## What NOT to Consolidate

- Functions that exist only in ONE file -> leave as-is
- Functions with different semantics but same name -> rename one, don't merge
- Large functions (>20 lines) -> consider if they belong in a module or are just poorly named

## Quick Commands

```bash
# Find duplicate function names across files
rg "^fn " crates/import-rules/src/ | cut -d: -f2 | sort | uniq -d

# Find identical function bodies
rg "^fn [a-z_]+\(" crates/import-rules/src/ -A 3
```
