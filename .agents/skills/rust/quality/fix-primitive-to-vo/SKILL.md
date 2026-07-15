---
name: fix-primitive-to-vo-rust
version: 1.0.0
category: refactoring
tags: [aes, primitive, vo, aes401, aes402, rust]
triggers:
  - "fix primitive to vo rust"
  - "replace primitive with vo rust"
dependencies: []
related:
  - fix-cross-import
---

# fix-primitive-to-vo-rust

## Rules

- Entity fields MUST use VOs, not primitives
- Contract signatures MUST use VOs
- VOs MUST validate on construction

## Purpose

Replace `String`, `i32`, `f64`, `bool` in entity fields and contract signatures with validated VOs.

## When to Use

- Method signature has raw primitives
- Entity field uses primitive type

## The Fundamental Question

> **"Is this a raw primitive?"**

If yes -> **Replace with domain VO**

## Workflow

### Step 1: Find Primitives

Read code and find primitives in signatures/entity fields.

### Step 2: Create/Find VO

Create or find existing VO.

### Step 3: Replace

Replace primitive with VO.

## Example

```rust
// BEFORE (primitive)
pub struct LintResult {
    pub file_path: String,
    pub line: u32,
    pub severity: String,
}

// AFTER (VO)
pub struct LintResult {
    pub file_path: FilePath,
    pub line: LineNumber,
    pub severity: Severity,
}
```
