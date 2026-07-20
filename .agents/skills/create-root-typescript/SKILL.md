---
name: create-root-typescript
description: "Create and validate TypeScript root layer files: composition root that wires Capabilities to Contract interfaces/aggregates and bootstraps the application. Container connects implementations, Entry starts the system."
version: 2.0.0
category: refactoring
tags:
  [
    typescript,
    aes,
    root,
    container,
    entry,
    composition,
    di,
    wiring,
  ]
triggers:
  - "create root typescript"
  - "add root typescript"
  - "create container typescript"
  - "create entry typescript"
  - "wire dependencies typescript"
  - "check root typescript"
  - "audit root typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-agent-typescript
  - create-contract-typescript
  - create-taxonomy-typescript
---

# create-root-typescript

## Purpose

Create and validate TypeScript **root layer** files — the composition layer that assembles the system.

Root connects concrete implementations to contracts and starts the application. It may depend on all layers.

Two root roles exist:

- `container` → Wires one feature by connecting Capabilities to Contract interfaces and aggregates
- `entry` → Bootstraps the application and composes feature containers

## Definition of Done

1. Root file uses correct suffix: `_container` or `_entry`.
2. Container wires Capabilities to Contract interfaces/aggregates.
3. Entry bootstraps the application and composes feature containers.
4. Root may instantiate and wire components.
5. Root must not contain business logic.
6. Root must not contain orchestration policy.
7. Root must not contain technical parsing or UI behavior.
8. `npx tsc --noEmit` passes.

## Workflow

### Step 1: Determine Root Role

Ask: **"What does this file do?"**

| Role | Suffix | Responsibility |
|------|--------|----------------|
| Container | `_container` | Wire one feature's Capabilities to Contracts |
| Entry | `_entry` | Bootstrap application, compose feature containers |

### Step 2: Create Root File

Create `root_<concept>_<suffix>.ts` in the appropriate location.

### Step 3: Wire Dependencies

Connect Capabilities implementations to their Contract interfaces/aggregates.

### Step 4: Verify

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# Find root files
grep -rn "root_\|container\|entry" packages/*/src/

# Check for business logic in root
grep -n "function calculate\|function validate\|function analyze" packages/*/src/root_*.ts
```

## Common Mistakes

- Putting business logic in root files.
- Putting orchestration policy in root files.
- Root containing UI behavior.
- Forgetting to wire Capabilities to Contracts.
- Creating circular dependencies between containers.
