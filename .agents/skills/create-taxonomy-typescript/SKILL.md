---
name: create-taxonomy-typescript
description: "Create and validate TypeScript taxonomy layer files in shared taxonomy: VOs, entities, errors, events, constants, and pure reusable utilities. Ensures domain data lives only in shared taxonomy and remains pure."
version: 1.3.0
category: refactoring
tags:
  [
    typescript,
    aes,
    taxonomy,
    shared,
    vo,
    entity,
    error,
    event,
    constant,
    utility,
    aes201,
    primitive-to-vo,
  ]
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
  - create-infrastructure-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - interface-consolidation-typescript
  - fix-primitive-to-vo
  - fix-magic-constant
---

# create-taxonomy-typescript

## Purpose

Create and validate TypeScript **taxonomy layer** files inside `packages/shared/src/<domain>/`.

Taxonomy is the single source of truth for:

- value objects, entities, domain errors, domain events,
- constants, pure reusable utility functions.

No domain data structures may be defined in capabilities, infrastructure, agents, surface, or root/container layers.

## Definition of Done

1. Domain data structures live in `shared/taxonomy`.
2. Taxonomy file naming uses allowed strict suffixes.
3. Taxonomy files do not import from capability, infrastructure, agent, surface, or root layers.
4. Taxonomy files contain no I/O and no side effects.
5. Utility functions are stateless, pure, domain-agnostic, and reusable.
6. Value objects validate on construction.
7. Public domain contracts use VOs instead of raw primitives.
8. New taxonomy modules are registered in `index.ts`.
9. `npx tsc --noEmit` passes.

## References

| File | Content |
|------|---------|
| `references/purity-imports.md` | AES201 import restrictions, allowed/forbidden dependencies |
| `references/dataclass-patterns.md` | VOs, entities, errors, events, constants patterns |
| `references/utility-functions.md` | The Ultimate Boundary, good/bad utility examples |
| `references/primitive-vo-rules.md` | Primitive policy table, VO construction rules |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | 20-item verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/taxonomy_name_vo.ts` | New value object file |
| `templates/taxonomy_name_error.ts` | New error type file |
| `templates/taxonomy_name_constant.ts` | New constants file |
| `templates/taxonomy_name_utility.ts` | New utility function file |

## Workflow

### Step 1: Identify the Data Type

When you find an interface/type in a layer file, ask: **"Is this a data type or an implementor?"**

If it carries domain data → move to taxonomy. If it implements an interface and uses DI → keep in layer file.

### Step 2: Determine Taxonomy Domain

Choose the correct domain directory under `packages/shared/src/<domain>/`.

### Step 3: Create or Update Taxonomy File

Use the correct suffix: `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`.

### Step 4: Register Module

Update the domain `index.ts`.

### Step 5: Update Imports in Layer Files

Replace local definitions with imports from taxonomy.

### Step 6: Verify Purity

No imports from layers, no I/O in taxonomy files.

### Step 7: Verify Primitive-to-VO Compliance

No public raw `string` domain fields, VOs validate on construction.

### Step 8: Verify Compilation

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# Find possible data types in layer files
grep -rn "^interface\|^type \|^enum " packages/*/src/ --exclude-dir=shared

# Check forbidden imports in taxonomy files
grep -n "from.*capabilities_\|from.*infrastructure_\|from.*agent_" packages/shared/src/*/taxonomy_*.ts

# Check possible I/O in taxonomy files
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/shared/src/*/taxonomy_*.ts
```

## Common Mistakes

- Defining interfaces/types in layer files.
- Importing non-taxonomy layer types into taxonomy files.
- Importing contract interfaces into taxonomy files.
- Using wrong suffix for taxonomy files.
- Forgetting to register taxonomy modules in `index.ts`.
- Putting domain knowledge into `*_utility.ts`.
- Putting single-consumer helpers into `*_utility.ts`.
- Exposing public raw `string` fields in VOs.
- Creating VOs without validation when domain invariants exist.
- Duplicating taxonomy types across domains.
