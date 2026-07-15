---
name: method_classifier-rust
version: 1.0.0
category: validation
tags: [aes, classification, public, helper, utility, capabilities, infrastructure, rust]
triggers:
  - "classify methods rust"
  - "check method order rust"
  - "validate public helper utility rust"
  - "align capabilities rust"
dependencies: []
related:
  - clean-bloat
  - module_logic_validator
---

# method_classifier-rust

## Rules

- Public methods MUST be above Helper methods
- Helper methods MUST be above Utility methods

## Purpose

Determine which methods SHOULD be called by the agent orchestrator, and ensure proper ordering and classification.

## When to Use

- After adding new methods to capabilities/infrastructure
- Before committing capability changes
- When user asks to classify methods

## The Fundamental Question

Before classifying any method, ask:

> **"Should this method be called by the agent?"**

If the answer is:

- "Yes, it's part of core workflow" → **Public**
- "It supports a Public method" → **Helper**
- "It's a stateless domain-agnostic tool" → **Utility**
- "No, it's not needed" → **Remove** (use clean-bloat)

## Classification Rules

- **[Public]**: Method that SHOULD be called by the agent orchestrator
- **[Helper]**: Method that supports a Public method
- **[Utility]**: Stateless, domain-agnostic mathematical or formatting tool

## Workflow

### Step 1: List All Methods

Read file and list all methods.

### Step 2: Classify Each Method

For each method, ask "Should this be called by the agent?"

### Step 3: Check Ordering

Verify Public above Helper above Utility.

### Step 4: Add Comments

Add classification comments.

## Ordering Rule

**Public** methods MUST be above **Helper** methods, which MUST be above **Utility** methods.

```rust
impl MyCapability {
    // [Public] Core workflow
    pub fn public_method(&self) -> Result { ... }

    // [Helper] Supports public
    fn helper_method(&self) -> Data { ... }

    // [Utility] Stateless tool
    fn utility_method(&self) -> String { ... }
}
```
