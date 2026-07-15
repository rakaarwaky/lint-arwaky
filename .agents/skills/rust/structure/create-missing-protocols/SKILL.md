---
name: create-missing-protocols-rust
version: 1.0.0
category: refactoring
tags: [aes, protocol, contract, capability, aes403, rust]
triggers:
  - "create trait rust"
  - "add trait rust"
  - "capability missing trait rust"
dependencies: []
related:
  - fix-capability-structure
  - module_logic_validator
  - trait-consolidation
---

# create-missing-protocols-rust

## Rules

- Every capability struct MUST implement a trait
- Trait MUST define methods for all public methods
- Trait lives in `crates/shared/src/<domain>/contract_*_protocol.rs`
- 1 capability struct = 1 trait file

## Purpose

Create missing trait files for capabilities that don't implement any trait (AES403 fix).

## When to Use

- Capability struct has no trait implementation
- Lint reports AES403 violations
- New capability file added without trait

## The Fundamental Question

> **"Does this capability have a trait?"**

If no → **Create trait and make capability implement it**

## Detection Pattern

```rust
// BAD: No trait
struct FrameComposer;
impl FrameComposer {
    fn compose_frame(&self) { ... }
}

// GOOD: Implements trait
struct FrameComposer;
impl IFrameComposerProtocol for FrameComposer {
    fn compose_frame(&self) { ... }  // implements trait method
}
```

## Trait Location

| Crate | Trait Path |
|-------|------------|
| import-rules | `crates/shared/src/import_rules/contract_*_protocol.rs` |
| code-analysis | `crates/shared/src/code_analysis/contract_*_protocol.rs` |

## Workflow

### Step 1: Find Capabilities Without Traits

Check each capability file for trait implementation.

### Step 2: List Public Methods

Identify all public methods that should be in the trait.

### Step 3: Create Trait File

Create `contract_*_protocol.rs` in shared crate with trait definition.

### Step 4: Update Capability

Make capability struct implement the new trait.

### Step 5: Verify

Run `cargo check` to confirm AES403 is resolved.
