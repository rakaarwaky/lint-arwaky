---
name: create-capabilities-typescript
description: "Create and validate TypeScript capabilities layer files following AES rules: pure domain behavior, zero I/O, 3-block structure, one class per file, protocol interface contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    typescript,
    aes,
    capability,
    protocol,
    structure,
    aes402,
    aes403,
    aes404,
    3-block-structure,
    di,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create capability typescript"
  - "add capability typescript"
  - "fix capability structure typescript"
  - "create protocol typescript"
  - "capability missing protocol typescript"
  - "check capabilities typescript"
  - "audit capabilities typescript"
dependencies: []
related:
  - create-infrastructure-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - trait-consolidation-typescript
  - module_logic_validator-typescript
  - fix-capability-structure-typescript
  - create-missing-protocols-typescript
---

# create-capabilities-typescript

## Purpose

Create and validate TypeScript **capabilities layer** files following clean architecture / AES rules.

A capabilities file must contain **pure domain behavior**:

- no I/O, no infrastructure detail, no agent detail,
- no locally defined domain data structures,
- one implementation class per file,
- one domain protocol interface as the public contract,
- strict 3-block structure,
- dependency injection for service collaborators,
- shared VOs for domain data.

## Definition of Done

1. ONE implementation class per file.
2. Class implements ONE domain protocol interface.
3. Block 2 contains ONLY domain protocol method implementations.
4. Utility methods, static factories, private helpers in Block 3.
5. Zero I/O, zero side-effecting infrastructure calls.
6. No locally defined domain data structures.
7. Service dependencies use DI via protocol interfaces.
8. Value/configuration fields use shared VOs.
9. Reusable, stateless, domain-agnostic functions extracted to `*_utility.ts`.
10. `npx tsc --noEmit` passes.

## References

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, method placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/primitive-vo-policy.md` | Primitive policy table, VO construction rules |
| `references/error-handling.md` | 4 error handling rules with examples |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | 23-item verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/capabilities.ts` | New capabilities implementation file |
| `templates/protocol.ts` | New protocol interface definition |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Is this pure domain behavior?"**

If yes → keep as capabilities. If no → move I/O to infrastructure.

### Step 2: Check Missing Interface (AES403)

Does the capability class implement a protocol interface? If no → create one.

### Step 3: Create Interface File if Missing

Create `contract_<name>_protocol.ts` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: class definition + `constructor` → protocol methods → utility/factories/helpers.

### Step 5: Verify Class Discipline

One class, no local data interfaces, DI via protocol interfaces, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no I/O, no business logic leakage.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# Check I/O in capabilities (AES404)
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/capabilities_*.ts

# Check forbidden imports
grep -n "^\s*from\s+.*infrastructure_\|^\s*from\s+.*agent_" packages/*/src/capabilities_*.ts

# List protocol interface implementations
grep -n "implements I[A-Za-z0-9_]*Protocol" packages/*/src/capabilities_*.ts
```

## Common Mistakes

- Putting I/O in capabilities.
- Defining domain data interfaces in capabilities files.
- Using concrete service types as constructor fields.
- Putting private helpers in the protocol interface.
- Putting constructors in the protocol interface.
- Placing utility methods before the domain protocol methods.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Silent error swallowing with `?? ''` or `|| 0`.
- Magic constants in capabilities logic.
