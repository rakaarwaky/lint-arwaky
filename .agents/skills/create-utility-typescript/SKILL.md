---
name: create-utility-typescript
description: "Create and validate TypeScript utility layer files following AES rules: stateless standalone functions, no class, no interface impl, pure functions, domain-agnostic, reusable across modules."
metadata:
  tags: [typescript, aes, utility, shared, stateless, pure-function, domain-agnostic]
  triggers:
    - "create utility typescript"
    - "add utility typescript"
    - "extract utility typescript"
    - "create helper function typescript"
    - "check utility typescript"
    - "audit utility typescript"
  dependencies: []
  related:
    - create-taxonomy-typescript
    - create-capabilities-typescript
    - create-agent-typescript
---

# create-utility-typescript

Utility layer = **stateless standalone functions**. No class, no `this`, no domain rules. Pure, domain-agnostic, reusable. File: `utility_<domain>_<role>.ts` (or `taxonomy_<domain>_utility.ts` in shared).

## Role Naming

parser, splitter, trimmer, slugifier, sanitizer, normalizer, extractor, replacer, converter, counter, resolver, detector, builder, joiner, serializer, deserializer, encoder, decoder, hasher, generator, formatter, comparator, differ, matcher, checker, calculator, mapper, merger, grouper, sorter, deduplicator, printer

## Definition of Done

1. Only exported functions — no class definitions.
2. No `this` keyword, no class properties, no instance state.
3. Pure: same input → same output (except I/O utilities).
4. Domain-agnostic: no business rules, no architecture knowledge.
5. Reusable: used by ≥2 modules (otherwise keep as private helper).
6. I/O allowed only if stateless + domain-agnostic + reusable.
7. No import from Capabilities, Agent, Surface, Contract.
8. May import from Taxonomy only.
9. `npx tsc --noEmit` passes.

---

## Stateless Rules

1. **No classes** — no `class`, no `this`, no instance state.
2. **Pure functions** — deterministic: same input → same output. No `Math.random()`, no `Date.now()`, no global mutable state.
3. **Domain-agnostic** — must NOT know about: architecture layer names (agent, capabilities, contract), business domain rules, specific capability logic.
4. **Reusable** — if only one module uses it → keep as private helper in that module.

### I/O Exception

Utility CAN perform I/O if ALL conditions met: stateless (no `this`), domain-agnostic, reusable across multiple modules.

---

## Keep vs Extract Decision

**Keep as private helper** if ANY: accesses `this`/instance state, domain-specific, only one consumer.

**Extract to utility** only if ALL: stateless, pure (I/O allowed), domain-agnostic, ≥2 consumers.

---

## Layer Boundaries

| Allowed | Forbidden |
| --- | --- |
| Stateless exported functions | Class definitions |
| Pure computation (input → output) | `this` keyword / instance state |
| I/O (if domain-agnostic + reusable) | Business rules / domain knowledge |
| Taxonomy imports (`shared/taxonomy_*`) | Import from Capabilities, Agent, Surface |
| File walking, pattern matching, parsing | Protocol/aggregate implementation |
| Environment access (if stateless + reusable) | Magic constants (→ `taxonomy_*_constant.ts`) |

---

## Templates

| File | Purpose |
| --- | --- |
| `templates/utility_name.ts` | Utility function module template |

---

## Workflow

1. **Confirm reusability** — Is this used by ≥2 modules? If no → keep as private helper.
2. **Confirm stateless** — No `this`, no class, no global mutation?
3. **Confirm domain-agnostic** — No business rules, no architecture knowledge?
4. **Create file** → `utility_<domain>_<role>.ts`.
5. **Register** → update `index.ts`.
6. **Verify** → `npx tsc --noEmit`.

---

## Verification Checklist

- [ ] Only exported functions — no class definitions.
- [ ] No `this`, no instance state.
- [ ] Pure / deterministic (or I/O with domain-agnostic + reusable justification).
- [ ] Domain-agnostic — no business rules, no layer-name knowledge.
- [ ] Used by ≥2 modules (otherwise keep as private helper).
- [ ] No import from Capabilities, Agent, Surface, Contract.
- [ ] No magic constants (extracted to `taxonomy_*_constant.ts`).
- [ ] `npx tsc --noEmit` passes.
