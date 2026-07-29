---
name: create-root-typescript
description: "Create and validate TypeScript root layer files: composition root that wires Capabilities to Contract interfaces/aggregates and bootstraps the application. Container connects implementations, Entry starts the system."
metadata:
  tags: [typescript, aes, root, container, entry, composition, di, wiring]
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

Root = **composition layer** that assembles the system. Connects concrete implementations to contracts and starts the application. May depend on all layers.

## Two Root Roles

| Role | Suffix | Responsibility |
| --- | --- | --- |
| Container | `_container` | Wire one feature's Capabilities to Contracts |
| Entry | `_entry` | Bootstrap application, compose feature containers |

## Definition of Done

1. Correct suffix: `_container` or `_entry`.
2. Container: wires Capabilities to Contract interfaces/aggregates.
3. Entry: bootstraps application and composes feature containers.
4. May instantiate and wire components.
5. No business logic.
6. No orchestration policy.
7. No technical parsing or UI behavior.
8. `npx tsc --noEmit` passes.

## Workflow

1. **Determine role** — Container (wire one feature) or Entry (bootstrap all)?
2. **Create file** → `root_<concept>_<suffix>.ts`.
3. **Wire deps** → Connect Capabilities to Contract interfaces.
4. **Register** → update `index.ts`.
5. **Verify** → `npx tsc --noEmit`.
