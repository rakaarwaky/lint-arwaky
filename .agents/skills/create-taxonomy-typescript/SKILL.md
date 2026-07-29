---
name: create-taxonomy-typescript
description: "Create and validate TypeScript taxonomy layer files in shared taxonomy: VOs, entities, errors, events, and constants. Taxonomy is the domain foundation layer — stable language of the domain, free from technical or behavioral concerns."
metadata:
  tags: [typescript, aes, taxonomy, shared, vo, entity, error, event, constant, primitive-to-vo]
  triggers:
    - "create taxonomy typescript"
    - "add taxonomy typescript"
    - "move dataclass to taxonomy typescript"
    - "create vo typescript"
    - "create error taxonomy typescript"
    - "create constant taxonomy typescript"
    - "check taxonomy typescript"
    - "audit taxonomy typescript"
  dependencies: []
  related:
    - create-capabilities-typescript
    - create-agent-typescript
    - create-contract-typescript
---

# create-taxonomy-typescript

Taxonomy = **stable language of the domain**. Single source of truth for VOs, entities, errors, events, constants. Free from technical/behavioral concerns. Location: `packages/shared/src/<domain>/`.

## Taxonomy Types

| File Suffix | Content | Rules |
| --- | --- | --- |
| `_vo.ts` | Value Objects | Validate on construction, `readonly` fields, no I/O |
| `_entity.ts` | Entities with identity | Identity field required (VO), no I/O |
| `_error.ts` | Domain error types | Extend `Error`, `name` set, VO fields only |
| `_event.ts` | Domain event types | Immutable, VO payload fields |
| `_constant.ts` | Compile-time constants | `export const` pure literals only, no functions, no I/O |
| `_utility.ts` | Stateless helper functions | No class, no `this`, pure, domain-agnostic |

## Definition of Done

1. Correct file suffix (`_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`).
2. VOs validate on construction when invariants exist.
3. No raw primitives in fields — use other VOs.
4. No I/O, no side effects, no business logic in VOs/entities/errors/events/constants.
5. Constants: `export const` pure literal values only.
6. Taxonomy imports only other taxonomy types or stdlib (`node:path`, etc.).
7. No import from capabilities, agents, surface, root, contracts.
8. Registered in shared `index.ts`.
9. `npx tsc --noEmit` passes.

---

## Purity and Import Restrictions (AES201/AES401)

| Taxonomy Type | May Import From | Must Not Import From |
| --- | --- | --- |
| `_vo`, `_entity`, `_error`, `_event` | other taxonomy types, stdlib | capabilities, agents, surface, root, contracts, I/O |
| `_constant` | only core/static values | external layer imports, I/O, functions |

**Taxonomy MAY contain:** value validation, domain invariants in constructors, pure transformations between taxonomy types.

**Taxonomy MUST NOT contain:** file I/O (`fs.`), network (`fetch`, `axios`), database, env mutation, side effects, business orchestration.

---

## VO Rules (AES401/AES402)

Domain data MUST use VOs, not raw primitives.

| Primitive | Rule |
| --- | --- |
| `string`, `number` | Forbidden for domain fields. Use VO. |
| `boolean` | Allowed for semantic toggles only. |
| `string[]`, `Record<string, T>` | Forbidden for domain collections/data. Use VO. |

Prefer VOs for: file paths, symbol names, messages, line numbers, severity, counts, thresholds, identifiers, results, policies.

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/taxonomy_name_vo.ts` | Value Object template |
| `templates/taxonomy_name_entity.ts` | Entity template |
| `templates/taxonomy_name_error.ts` | Error type template |
| `templates/taxonomy_name_constant.ts` | Constants template |

---

## Workflow

1. **Determine type** — VO / Entity / Error / Event / Constant / Utility?
2. **Create file** → `taxonomy_<domain>_<type>.ts` in `shared/src/<domain>/`.
3. **VOs**: `readonly` fields, validate in constructor, throw on invalid.
4. **Entities**: add identity VO field.
5. **Errors**: `extends Error`, set `this.name`.
6. **Constants**: `export const NAME = value` only.
7. **Register** → update `index.ts`.
8. **Verify** → `npx tsc --noEmit`.

---

## Verification Checklist

- [ ] Correct file suffix.
- [ ] VOs validate on construction when invariants exist.
- [ ] Single-value VOs expose safe constructors and accessors.
- [ ] Composite VOs use other VOs instead of raw primitives.
- [ ] Error types extend `Error`.
- [ ] Constants are `export const` pure literal values.
- [ ] No import from capabilities, agents, surface, root, contracts.
- [ ] No I/O, no network, no database in taxonomy files.
- [ ] Registered in shared `index.ts`.
- [ ] `npx tsc --noEmit` passes.
