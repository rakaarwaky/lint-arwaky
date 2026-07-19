---
name: create-infrastructure-typescript
description: "Create and validate TypeScript infrastructure layer files following AES rules: I/O and external integration only, zero business logic, 3-block structure, one class per file, port interface contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    typescript,
    aes,
    infrastructure,
    port,
    structure,
    aes404,
    3-block-structure,
    di,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create infrastructure typescript"
  - "add infrastructure typescript"
  - "fix infrastructure structure typescript"
  - "create port typescript"
  - "infrastructure missing port typescript"
  - "check infrastructure typescript"
  - "audit infrastructure typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - trait-consolidation-typescript
  - module_logic_validator-typescript
  - fix-infrastructure-structure-typescript
  - create-missing-ports-typescript
---

# create-infrastructure-typescript

## Purpose

Create and validate TypeScript **infrastructure layer** files following clean architecture / AES rules.

An infrastructure file must contain **I/O and external system integration only**:

- file system access, network calls, database access, external API calls,
- environment/system integration, technical mapping, serialization/deserialization,
- error mapping, adapter implementation for port interfaces.

Infrastructure MUST NOT contain business logic.

## Definition of Done

1. ONE implementation class per file.
2. Class implements ONE domain port interface.
3. Block 2 contains ONLY port interface method implementations.
4. Utility methods, static factories, private helpers in Block 3.
5. Zero business logic.
6. No locally defined domain data structures.
7. Service dependencies use DI via port interfaces.
8. Value/configuration fields use shared VOs.
9. I/O errors are propagated explicitly.
10. `npx tsc --noEmit` passes.

## References

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, method placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/primitive-vo-policy.md` | Primitive policy table, VO rules |
| `references/error-handling.md` | 4 error handling rules |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | 22-item verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/infrastructure.ts` | New infrastructure implementation file |
| `templates/port.ts` | New port interface definition |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Is this pure I/O or external system integration?"**

If yes → keep as infrastructure. If it contains business logic → move to capabilities.

### Step 2: Check for Missing Port

Does the infrastructure class implement a port interface? If no → create one.

### Step 3: Create Port File if Missing

Create `contract_<name>_port.ts` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: class definition + `constructor` → port methods → utility/factories/helpers.

### Step 5: Verify Class Discipline

One class, no local data interfaces, DI via port interfaces, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no business logic, no local domain data definitions.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# List port interface implementations
grep -n "implements I[A-Za-z0-9_]*Port" packages/*/src/infrastructure_*.ts

# Check business logic keywords
grep -n "is_orphan\|analyze\|validate\|calculate\|compute\|business" packages/*/src/infrastructure_*.ts

# Check forbidden imports
grep -n "^\s*from\s+.*capabilities_\|^\s*from\s+.*agent_" packages/*/src/infrastructure_*.ts
```

## Common Mistakes

- Putting business logic in infrastructure.
- Defining domain data interfaces in infrastructure files.
- Using concrete service types as constructor fields.
- Putting private helpers in the port interface.
- Putting constructors in the port interface.
- Placing utility methods before the port interface methods.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Silent error swallowing with `?? ''` or `|| 0`.
- Magic constants in infrastructure logic.
- Infrastructure returning lint results directly.
