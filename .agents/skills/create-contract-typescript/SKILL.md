---
name: create-contract-typescript
description: "Create and validate TypeScript contract layer files in shared domain: pure interface definitions for protocols and aggregates. Contracts define public promises only, with no implementation, no layer imports, and domain-safe VO-based signatures."
metadata:
  tags: [typescript, aes, contract, protocol, aggregate, interface, vo]
  triggers:
    - "create contract typescript"
    - "add contract typescript"
    - "create protocol typescript"
    - "create aggregate typescript"
    - "contract missing typescript"
    - "validate contract typescript"
    - "check contract typescript"
  dependencies: []
  related:
    - create-capabilities-typescript
    - create-agent-typescript
    - create-taxonomy-typescript
---

# create-contract-typescript

Contract layer = **pure interface definitions** for the shared domain. No implementation. No layer imports. File: `contract_<concept>_<suffix>.ts`.

## Contract Roles

| Suffix | Implemented By | Used By | Example |
| --- | --- | --- | --- |
| `_protocol` | Capabilities | Agent | `contract_import_forbidden_protocol.ts` |
| `_aggregate` | Agent | Surface | `contract_import_runner_aggregate.ts` |

Interface naming: `I<Name>Protocol`, `I<Name>Aggregate`.

## Definition of Done

1. Contract file uses correct suffix: `_protocol` or `_aggregate`.
2. Contains only interface definitions — no class implementations.
3. No private helper method signatures.
4. All methods have proper TypeScript type annotations.
5. Interfaces exported with `export interface`.
6. Imports only taxonomy and other contract types.
7. Signatures use shared VOs for domain data.
8. Error types from shared taxonomy.
9. Module registered in shared `index.ts`.
10. `npx tsc --noEmit` passes.

---

## Purity and Import Restrictions (AES201)

| Contract File | May Import From | Must Not Import From |
| --- | --- | --- |
| `contract_*_protocol.ts` | taxonomy types, other contract types | capabilities, agents, surface, root |
| `contract_*_aggregate.ts` | taxonomy types, other contract types | capabilities, agents, surface, root |

---

## Interface Structure Rules

1. Contracts contain interface definitions only.
2. No method implementations.
3. No private helper signatures.
4. All methods MUST have proper TypeScript type annotations.
5. Interfaces MUST be exported with `export interface`.
6. Error types from shared taxonomy.
7. Naming: `I<Name>Protocol`, `I<Name>Aggregate`.

---

## VO Rules

Contract signatures must use shared VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `string`, `number` | Forbidden for domain fields/contract values. Use VO. |
| `boolean` | Allowed for semantic toggles only. |
| `string[]`, `Record<string, T>` | Forbidden for domain collections/data. Use VO. |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/contract_name_protocol.ts` | Protocol interface definition |
| `templates/contract_name_aggregate.ts` | Aggregate interface definition |

---

## Workflow

1. **Determine role** — Which layer implements this? Capabilities → `_protocol`. Agent → `_aggregate`.
2. **Identify public methods** — Golden Rule: called by outer layers? YES → keep. NO → private helper (not in interface).
3. **Create file** → `contract_<concept>_<suffix>.ts` in shared domain.
4. **Register** → update `index.ts`.
5. **Verify** → `npx tsc --noEmit`.

---

## Verification Checklist

- [ ] Correct suffix: `_protocol` or `_aggregate`.
- [ ] Only interface definitions — no implementations.
- [ ] No private helper signatures.
- [ ] All methods type-annotated.
- [ ] Interfaces exported with `export interface`.
- [ ] Imports only taxonomy and contract types.
- [ ] No import from capabilities, agents, surface.
- [ ] Signatures use shared VOs.
- [ ] Error types from shared taxonomy.
- [ ] Registered in shared `index.ts`.
- [ ] `npx tsc --noEmit` passes.
