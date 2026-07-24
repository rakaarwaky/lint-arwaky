---
name: create-utility-typescript
description: "Create and validate TypeScript utility layer files following AES rules: stateless standalone functions, no class, no interface impl, pure functions, domain-agnostic, reusable across modules."
metadata:
  tags:
    [
      typescript,
      aes,
      utility,
      shared,
      stateless,
      pure-function,
      domain-agnostic,
    ]
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

## Purpose

Create and validate TypeScript **utility layer** files inside `packages/shared/src/<domain>/`.

A utility file contains **stateless standalone functions**. It exists so that Capabilities, Agents, and Surfaces can remain clean and expressive by delegating low-level technical mechanics to reusable helpers.

A utility file must:

- contain ONLY exported functions (no class, no interface),
- be completely stateless (no instance state, no class properties),
- be pure (input A always produces output B),
- be domain-agnostic (no business rules, no domain knowledge),
- be reusable across multiple modules.

## Role Naming (ARCHITECTURE §7)

Utility role suffixes describe the technical responsibility:

parser, splitter, trimmer, slugifier, sanitizer, normalizer, extractor, replacer, converter, counter, resolver, detector, builder, joiner, serializer, deserializer, encoder, decoder, hasher, generator, formatter, comparator, differ, matcher, checker, calculator, mapper, merger, grouper, sorter, deduplicator, printer

File: `utility_<domain>_<role>.ts`

## Dependencies (ARCHITECTURE §7)

- **May depend on:** Taxonomy only.
- **Must NOT depend on / import:** Capabilities, Agent, Surface, Contract, other Utility (except shared taxonomy utilities).

## Special Rules (ARCHITECTURE §7)

- **Stateless Only:** no class definitions, no `this`, no instance state.
- **Pure Functions:** input A always produces output B. No randomness, no global state mutation, no I/O side effects (unless domain-agnostic + reusable).
- **No Business Decisions:** utility does not know business rules, domain constraints, or architecture policies.
- **No Interface Implementation:** utility never implements a protocol or aggregate interface.
- **I/O Allowed:** stateless + I/O + domain-agnostic + reusable = valid utility (e.g., `walkSourceFiles`, `readFileContent`).
- **Standalone Functions Only:** no classes, no methods. Just `export function` declarations.
- **No Magic Constants:** extract reusable constants into `taxonomy_<domain>_constant.ts` in shared.

## Definition of Done

1. NO class definition — only exported functions.
2. All functions are stateless (no `this`, no instance state).
3. Functions are pure: input A always produces output B.
4. Functions are domain-agnostic: no business rules, no architecture knowledge.
5. Functions are reusable across multiple modules.
6. No magic constants — use shared taxonomy constants.
7. Only depends on Taxonomy layer.
8. `npx tsc --noEmit` passes.

## References

Read these files for detailed rules:

| File                             | Content                                         |
| -------------------------------- | ----------------------------------------------- |
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies      |
| `references/stateless-rules.md`  | Stateless, pure, domain-agnostic decision rules |
| `references/examples.md`         | All BAD/GOOD code examples                      |
| `references/commands.md`         | Quick heuristic check commands                  |

## Templates

Use these templates when creating new files:

| File                          | Purpose                         |
| ----------------------------- | ------------------------------- |
| `templates/utility_name.ts`   | New utility implementation file |

## Workflow

### Step 1: Analyze Code Responsibility

Read the code and ask: **"Is this a stateless, pure, domain-agnostic function?"**

If yes → extract to utility. If no → check if it's business logic (→ capabilities), orchestration (→ agent), or domain data (→ taxonomy).

### Step 2: Check Reusability

Is the function used by multiple modules? Or will it be useful in the future?

- **Single-use + domain-specific** → keep as private helper in the layer file
- **Reusable + domain-agnostic** → extract to utility

### Step 3: Verify Stateless Purity

Does the function have ANY of these?

- `this` keyword
- Access to class properties
- Random number generation
- System clock access
- Global state mutation
- Business rule knowledge

If YES → NOT a utility. Keep as private helper.

### Step 4: Verify Domain Agnosticism

Does the function know about:

- Architecture layer names?
- Business domain rules?
- Specific capability logic?

If YES → NOT a utility. Domain-specific code belongs in capabilities.

### Step 5: Create Utility File

Write the exported functions following the template. No classes.

### Step 6: Update Module Registration

Add export to the appropriate shared domain `index.ts`.

### Step 7: Verify Compilation

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# Check for forbidden patterns (class, this)
grep -rn "^class \|this\." packages/shared/src/<domain>/utility_*.ts

# List all utility functions
grep -rn "^export function" packages/shared/src/<domain>/utility_*.ts

# Check imports in utilities (should only use taxonomy)
grep -rn "^import" packages/shared/src/<domain>/utility_*.ts
```

## Common Mistakes

- Adding class definitions to utility files.
- Implementing interface contracts in utility files.
- Using `this` or accessing class properties.
- Including business logic or domain rules.
- Using magic constants instead of shared taxonomy constants.
- Importing Capabilities, Agent, or Surface modules.
- Creating functions that are only used by one module (keep as private helper).
- Mixing pure functions with stateful operations.
- Adding I/O to domain-specific functions (must be domain-agnostic + reusable).
