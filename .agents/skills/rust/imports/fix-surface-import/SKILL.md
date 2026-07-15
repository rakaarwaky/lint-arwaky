---
name: fix-surface-import-rust
version: 1.0.0
category: refactoring
tags: [aes, surface, import, aes201, rust]
triggers:
  - "fix surface import rust"
  - "surface import violation rust"
dependencies: []
related:
  - module_logic_validator
---

# fix-surface-import-rust

## Rules

- Surfaces MUST use aggregate traits via DI
- Surfaces MUST NOT import capabilities or infrastructure directly

## Purpose

Prevent CLI/web controllers from importing capabilities/infrastructure directly.

## When to Use

- Surface file imports `capabilities_*` or `infrastructure_*`

## The Fundamental Question

> **"Does this surface import concrete structs?"**

If yes -> **Use aggregate trait via DI**

## Workflow

### Step 1: Find Forbidden Imports

Read surface file, ask: Does it import capabilities_ or infrastructure_?

### Step 2: Replace with Aggregate Trait

Change import to aggregate trait.

### Step 3: Wire via DI Container

Wire dependencies through container.

## Example

```rust
// WRONG:
// surface_cli_command.rs
use crate::capabilities_lint_executor::LintExecutor;  // FORBIDDEN

// CORRECT:
// surface_cli_command.rs
use crate::contract_lint_aggregate::ILintAggregate;

pub struct CliCommand {
    lint: Arc<dyn ILintAggregate>,
}

impl CliCommand {
    pub fn new(lint: Arc<dyn ILintAggregate>) -> Self {
        Self { lint }
    }
}
```
