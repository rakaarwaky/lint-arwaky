# Contract — TypeScript

File: `contract_<concept>_<suffix>.ts`. Contract = pure interface definitions. No implementations.

## Import rules

**Allowed imports:** taxonomy types, other contract types.
**Forbidden:** capabilities, agents, surface, root.

## Rules (TypeScript)

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
