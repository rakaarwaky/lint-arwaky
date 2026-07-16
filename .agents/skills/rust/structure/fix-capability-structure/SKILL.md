---
name: fix-capability-structure-rust
version: 1.0.0
category: refactoring
tags: [aes, capability, protocol, structure, aes403, aes404, rust]
triggers:
  - "fix capability structure rust"
  - "create trait rust"
  - "capability missing trait rust"
dependencies: []
related:
  - clean-bloat
  - module_logic_validator
  - trait-consolidation
---

# fix-capability-structure-rust

## Rules

- Capabilities: ZERO I/O, must implement trait, ALL methods in trait
- Infrastructure: ZERO business logic, must implement port
- One capability struct = one trait file

## Purpose

Fix violations where capability struct doesn't implement trait (AES403) or file contains mixed business logic + I/O (AES404).

## When to Use

- Adding new capability file
- Capability struct has no trait implementation
- File contains both business logic and I/O

## The Fundamental Question

> **"Is this file pure business logic?"**

If yes -> **capabilities\_*.rs + implement trait**
If no (has I/O) -> **split into infrastructure\_*.rs**

## AES Layer Rules

### Capabilities Layer

```
ALLOWED: Computation, Validation, Data Transformation, Business Rules
FORBIDDEN: File I/O, Network calls, Database operations
```

### Infrastructure Layer

```
ALLOWED: File I/O, Network calls, Database operations
FORBIDDEN: Business rules, Domain logic, Computation
```

## Workflow

### Step 1: Analyze File

Read file and check for mixed responsibilities.

### Step 2: Create/Find Trait

If pure business logic -> create/find trait, implement it.

### Step 3: Split if Mixed

If mixed -> split into capabilities + infrastructure.

### Step 4: Ensure All Methods in Trait

Check that ALL methods are in trait.

## File Naming Convention

| Layer          | File Pattern          | Trait Pattern            |
| -------------- | --------------------- | ------------------------ |
| Capabilities   | `capabilities_*.rs`   | `contract_*_protocol.rs` |
| Infrastructure | `infrastructure_*.rs` | `contract_*_port.rs`     |

## Quick Reference

| Layer          | Can Contain                  | Cannot Contain              |
| -------------- | ---------------------------- | --------------------------- |
| capabilities   | Pure computation, validation | I/O, network, database      |
| infrastructure | I/O, network, database       | Business logic, computation |
