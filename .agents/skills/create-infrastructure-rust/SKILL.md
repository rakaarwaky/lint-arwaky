---
name: create-infrastructure-rust
description: "DEPRECATED: Infrastructure layer has been merged into Capabilities in the new AES architecture. Use create-capabilities-rust instead. Capabilities now handles both business logic AND external adaptation (I/O, API calls, database access)."
version: 2.0.0
category: refactoring
tags:
  [
    rust,
    aes,
    infrastructure,
    deprecated,
    capabilities,
  ]
triggers:
  - "create infrastructure rust"
  - "add infrastructure rust"
dependencies: []
related:
  - create-capabilities-rust
---

# create-infrastructure-rust — DEPRECATED

## ⚠️ This Skill Is Deprecated

The Infrastructure layer has been **removed** in the new AES architecture (v2.0).

Infrastructure responsibilities (I/O and external system integration) have been **merged into the Capabilities layer**.

## What Changed

| Before (v1.x) | After (v2.0) |
|----------------|--------------|
| Infrastructure layer: I/O only | **Removed** |
| Capabilities layer: business logic only | Now includes **business logic + external adaptation** |
| `_port` contract suffix | **Removed** — use `_protocol` instead |

## Migration

- Use **create-capabilities-rust** for all new files
- Existing `infrastructure_*.rs` files should be refactored into `capabilities_*.rs`
- Existing `contract_*_port.rs` files should be refactored into `contract_*_protocol.rs`
- Capabilities can now perform I/O directly (file access, network calls, database operations)
- Capabilities must use Utility standalone functions for low-level technical mechanics

## References

See [create-capabilities-rust](../create-capabilities-rust/SKILL.md) for the current architecture rules.
