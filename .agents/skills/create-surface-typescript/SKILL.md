---
name: create-surface-typescript
description: "Create and validate TypeScript surface layer files following AES406: smart/utility/passive surfaces, strict import rules, delegate to aggregates, zero direct lower-layer imports, zero business logic, VO-based state, and explicit error handling."
version: 1.3.0
category: refactoring
tags:
  [
    typescript,
    aes,
    surface,
    command,
    controller,
    component,
    hook,
    store,
    action,
    view,
    entry,
    structure,
    aes406,
    vo,
    di,
  ]
triggers:
  - "create surface typescript"
  - "add surface typescript"
  - "fix surface typescript"
  - "create command typescript"
  - "create component typescript"
  - "create hook typescript"
  - "create store typescript"
  - "surface role violation typescript"
  - "audit surface typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-infrastructure-typescript
  - create-agent-typescript
  - create-contract-typescript
  - create-taxonomy-typescript
  - enforce-1-class-per-file-typescript
  - module_logic_validator-typescript
---

# create-surface-typescript

## Purpose

Create and validate TypeScript **surface layer** files in feature packages.

The surface layer is the outermost boundary of the application.

It is responsible for:

- receiving user input,
- mapping input events to shared action/event VOs,
- delegating execution to aggregates,
- rendering/displaying state from shared VOs.

The surface layer MUST NOT:

- import capabilities directly,
- import infrastructure directly,
- import concrete agent classes directly,
- contain business logic,
- contain domain computation,
- perform I/O directly.

## Definition of Done

1. Surface file uses a valid suffix.
2. Surface role is clear: smart, utility, or passive.
3. Smart surface imports only taxonomy and aggregate contracts.
4. Utility surface imports only taxonomy and passive surfaces.
5. Passive surface imports only taxonomy.
6. No surface file imports capabilities, infrastructure, or concrete agents.
7. Smart surface delegates to aggregates via `I<Name>Aggregate`.
8. Utility surface does not import concrete smart surfaces.
9. Passive surface contains only rendering/display logic.
10. Surface state fields use shared VOs.
11. Service dependencies use port interfaces via DI.
12. Errors are handled explicitly.
13. `npx tsc --noEmit` passes.

## References

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports for each surface type |
| `references/surface-types.md` | Smart/Utility/Passive surface definitions |
| `references/helper-vs-utility.md` | Helper vs utility decision |
| `references/error-handling.md` | Error handling rules |
| `references/primitive-vo-policy.md` | Primitive policy table |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | 20-item verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/surface_name.ts` | New surface implementation file |

## Workflow

### Step 1: Determine Surface Type

Ask: **"What role does this surface serve?"**

| Role | Suffixes |
|------|----------|
| Entry point / command / controller | `_command`, `_controller`, `_page`, `_entry` |
| Event/action/store/screen adapter | `_hook`, `_store`, `_action`, `_screen` |
| Rendering component/view/layout | `_component`, `_view`, `_layout` |

### Step 2: Check Import Rules

Verify imports follow the correct pattern for the surface type.

### Step 3: Create Surface File

Create `surface_<concept>_<suffix>.ts` in the appropriate feature package.

### Step 4: Verify Role Compliance

No capabilities imports, no infrastructure imports, no concrete agent imports, no business logic, no domain computation, no I/O.

### Step 5: Verify DI and VO Usage

Service fields use port interfaces, state fields use shared VOs.

### Step 6: Verify Error Handling

No silent error swallowing.

### Step 7: Verify Compilation

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# Check forbidden lower-layer imports
grep -n "^\s*from\s+.*capabilities_\|^\s*from\s+.*infrastructure_\|^\s*from\s+.*agent_" packages/*/src/surface_*.ts

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes

- Importing capabilities directly in surface files.
- Importing infrastructure directly in surface files.
- Importing concrete agent classes in surface files.
- Smart surface calling capabilities or infrastructure directly.
- Utility surface importing concrete smart surface.
- Passive surface containing business logic.
- Defining domain data interfaces in surface files.
- Using concrete service types as smart surface fields.
- Silently discarding errors.
