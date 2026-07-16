---
name: clean-bloat-rust
version: 1.0.0
category: validation
tags: [aes, cleanup, bloat, stubs, thin-wrappers, mvp, boilerplate, rust]
triggers:
  - "clean bloat rust"
  - "remove stubs rust"
  - "remove thin wrappers rust"
  - "clean capabilities rust"
dependencies: []
related:
  - module_logic_validator
---

# clean-bloat-rust

## Rules

- Never remove real logic
- Always update trait (remove methods)
- Always run lint after changes

## Purpose

Scan capability files for code that is NOT relevant to the MVP scope.

## When to Use

- After refactoring capability modules
- Before committing capability changes
- When user asks to clean bloat from a module

## The Fundamental Question

Before keeping any function, ask:

> **"Why does this function need to exist?"**

If the answer is:

- "Because it was always there" -> **REMOVE**
- "Because it might be useful someday" -> **REMOVE**
- "Because it handles edge cases we don't have" -> **REMOVE**
- "Because it's required by MVP" -> **KEEP**
- "Because it's called by a method that's required by MVP" -> **KEEP**

## Detection Patterns

### Thin Wrappers (Remove)

```rust
// Simple attribute return
fn get_something(&self, obj: &Obj) -> f64 {
    obj.attribute
}
// WHY: Direct attribute access is simpler. No logic added.

// Simple enum comparison
fn should_force_x(&self, hint: &ActionHint) -> bool {
    *hint == ActionHint::X
}
// WHY: Comparison is already simple. Wrapper adds no value.
```

### Stubs (Remove)

```rust
fn method(&self) -> Option<()> { None }
fn method(&self) -> String { String::new() }
// WHY: Empty implementations provide no value.
```

### Duplicate Functions (Remove)

Same function in multiple capability files -- keep in the file that owns the logic.
WHY: Duplicates create maintenance burden. Single source of truth.

### Overengineered Patterns (Remove)

```rust
// Temporal enforcer, circular dependency detection, etc.
// if NOT in MVP -> REMOVE
// WHY: Complexity without clear MVP requirement is waste.
```

## Workflow

### Step 1: Read Requirements

Read the requirements to understand MVP scope.

### Step 2: List Files

List all capability/trait files.

### Step 3: Analyze Each File

For each file, ask "Why does each function need to exist?"

### Step 4: Mark for Removal

If answer is not "required by MVP" -> mark for removal.

### Step 5: Report

Report per file -- show what to keep/remove.

### Step 6: Get Approval

Get approval per file.

### Step 7: Execute Cleanup

Remove bloat, update trait.
