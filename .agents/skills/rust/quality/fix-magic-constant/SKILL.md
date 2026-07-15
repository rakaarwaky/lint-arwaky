---
name: fix-magic-constant-rust
version: 1.0.0
category: refactoring
tags: [aes, magic, constant, aes405, rust]
triggers:
  - "fix magic constant rust"
  - "replace hardcoded value rust"
dependencies: []
related:
  - fix-agent-di
---

# fix-magic-constant-rust

## Rules

- NO hardcoded literals in ANY layer
- All domain values MUST be named constants
- Constants MUST live in `taxonomy_*_constant.rs`

## Purpose

Remove hardcoded literals from ALL layers (agent, capabilities, infrastructure) and replace with named constants.

## When to Use

- Agent file has hardcoded literals
- Capabilities file has hardcoded literals
- Infrastructure file has hardcoded literals
- Magic numbers or strings in business logic

## The Fundamental Question

> **"Is there a hardcoded literal?"**

If yes -> **Replace with named constant**

## Workflow

### Step 1: Find Magic Constants

Read code and find hardcoded literals.

### Step 2: Create/Find Constant

Create or find named constant in taxonomy.

### Step 3: Replace

Replace magic with named constant.

## Layer-Specific Examples

### Agent

```rust
// [FORBIDDEN] BEFORE
let result = self.process(fps: 24);

// [OK] AFTER
use crate::taxonomy_animator_constant::FPS_DEFAULT;
let result = self.process(fps: FPS_DEFAULT);
```

### Capabilities

```rust
// [FORBIDDEN] BEFORE
fn calculate_duration(&self) -> f64 {
    0.5  // magic
}

// [OK] AFTER
use crate::taxonomy_animator_constant::MIN_REVEAL_SECONDS;
fn calculate_duration(&self) -> f64 {
    MIN_REVEAL_SECONDS
}
```

### Infrastructure

```rust
// [FORBIDDEN] BEFORE
fn save(&self) {
    let file = std::fs::File::create("manifest.json");  // magic path
}

// [OK] AFTER
use crate::taxonomy_animator_constant::MANIFEST_FILENAME;
fn save(&self) {
    let file = std::fs::File::create(MANIFEST_FILENAME);
}
```
