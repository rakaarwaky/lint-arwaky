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

Contract = pure interface definitions. No implementations. File: `contract_<concept>_<suffix>.ts`.

**Allowed imports:** taxonomy types, other contract types.
**Forbidden:** capabilities, agents, surface, root.

## Contract Roles

| Suffix | Implemented By | Used By |
| --- | --- | --- |
| `_protocol` | Capabilities | Agent |
| `_aggregate` | Agent | Surface |

Naming: `I<Name>Protocol`, `I<Name>Aggregate`.

## Rules

- `export interface` only — no class implementations.
- No private helper signatures.
- All methods type-annotated.
- Signatures use shared VOs — no `string`/`number`/`string[]`/`Record<string,T>` for domain values.
- `boolean` allowed for semantic toggles only.
- Register in shared `index.ts`.

## Templates

### Protocol interface

```typescript
import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Protocol {
    methodName(param: <VO>): void;
}
```

### Aggregate interface

```typescript
import { <VO> } from '../shared/<domain>/taxonomy_<name>_vo';

export interface I<Name>Aggregate {
    execute(request: ScanRequest): LintResult[];
}
```

## Workflow

1. Which layer implements this? Capabilities → `_protocol`. Agent → `_aggregate`.
2. Golden Rule: only methods called by outer layers go in the interface.
3. Create `contract_<concept>_<suffix>.ts` in shared domain.
4. Register in `index.ts`.
5. `npx tsc --noEmit`.

## Checklist

- [ ] Correct suffix `_protocol` or `_aggregate`.
- [ ] `export interface` only — no class implementations.
- [ ] All methods type-annotated.
- [ ] No imports from capabilities, agents, surface.
- [ ] Signatures use shared VOs.
- [ ] Registered in shared `index.ts`.
- [ ] `npx tsc --noEmit` passes.
