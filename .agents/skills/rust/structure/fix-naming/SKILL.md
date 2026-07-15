---
name: fix-naming-rust
version: 1.0.0
category: refactoring
tags: [aes, naming, convention, rust]
triggers:
  - "fix naming rust"
  - "rename file rust"
  - "naming convention rust"
dependencies: []
related:
  - module_logic_validator
---

# fix-naming-rust

## Rules

| Layer | Pattern | Suffix |
|-------|---------|--------|
| root | `root_*_container.rs` | `_container` |
| taxonomy | `taxonomy_*_vo.rs` | `_vo`, `_constant` |
| contract | `contract_*_protocol.rs` | `_protocol`, `_port`, `_aggregate` |
| capabilities | `capabilities_*.rs` | flexible |
| infrastructure | `infrastructure_*.rs` | flexible |
| agent | `agent_*.rs` | `_orchestrator` |
| surface | `surface_*.rs` | `_command`, `_controller` |

## Purpose

Rename files to follow `prefix_concept_suffix` pattern with correct layer suffix.

## When to Use

- File name doesn't follow convention
- Wrong suffix for layer type

## The Fundamental Question

> **"Does this file name follow the pattern?"**

Pattern: `prefix_concept_suffix.rs`

## Workflow

### Step 1: Check Current Name

Compare file name against naming rules.

### Step 2: Rename File

Use `git mv` to rename file.

### Step 3: Update All Imports

Update all files that import from this file.

## Common Violations

| Violation | Fix |
|-----------|-----|
| `checker.rs` | Rename to `capabilities_checker.rs` |
| `parser.rs` | Rename to `infrastructure_parser.rs` |
| `my_struct.rs` | Rename to `taxonomy_my_struct_vo.rs` |
| `contract.rs` | Rename to `contract_*_protocol.rs` |
