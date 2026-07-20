---
name: create-infrastructure-typescript
description: "DEPRECATED: Infrastructure layer has been merged into Capabilities in the new AES architecture. Use create-capabilities-typescript instead. Capabilities now handles both business logic AND external adaptation (I/O, API calls, database access)."
version: 2.0.0
category: refactoring
tags:
  [
    typescript,
    aes,
    infrastructure,
    deprecated,
    capabilities,
  ]
triggers:
  - "create infrastructure typescript"
  - "add infrastructure typescript"
dependencies: []
related:
  - create-capabilities-typescript
---

# create-infrastructure-typescript — DEPRECATED

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

- Use **create-capabilities-typescript** for all new files
- Existing `infrastructure_*.ts` files should be refactored into `capabilities_*.ts`
- Existing `contract_*_port.ts` files should be refactored into `contract_*_protocol.ts`
- Capabilities can now perform I/O directly (file access, network calls, database operations)
- Capabilities must use Utility standalone functions for low-level technical mechanics

## References

See [create-capabilities-typescript](../create-capabilities-typescript/SKILL.md) for the current architecture rules.
