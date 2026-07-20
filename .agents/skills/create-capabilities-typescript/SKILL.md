---
name: create-capabilities-typescript
description: "Create and validate TypeScript capabilities layer files following AES rules: concrete implementation of behavior (business logic + external adaptation), 3-block structure, one class per file, protocol interface contracts, DI for service dependencies, and shared VOs for domain data."
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
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - trait-consolidation-typescript
  - module_logic_validator-typescript
  - fix-capability-structure-typescript
  - create-missing-protocols-typescript
---

# create-capabilities-typescript

## Purpose

Create and validate TypeScript **capabilities layer** files following AES rules.

A capabilities file contains the **concrete implementation** of the system's behavior. This layer encapsulates both:

- **Business logic**: computations, validations, transformations, assessments
- **External adaptation**: database access, third-party API calls, file system access, infrastructure mechanics

Capabilities hide these implementations behind Contracts, keeping behavior modular, swappable, and fully isolated from orchestration.

A capabilities file must:

- implement one domain protocol interface,
- follow strict 3-block structure,
- use dependency injection for service collaborators,
- use shared VOs for domain data,
- use Utility standalone functions for low-level technical operations.

## Definition of Done

1. ONE implementation class per file.
2. Class implements ONE domain protocol interface.
3. Block 2 contains ONLY domain protocol method implementations.
4. Utility methods, static factories, private helpers in Block 3.
5. No locally defined domain data structures.
6. Service dependencies use DI via protocol interfaces.
7. Value/configuration fields use shared VOs.
8. No inter-capability dependencies (capabilities must not import other capabilities).
9. Low-level technical operations delegate to Utility standalone functions.
10. `npx tsc --noEmit` passes.

## References

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, method placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/primitive-vo-policy.md` | Primitive policy table, VO construction rules |
| `references/error-handling.md` | Error handling rules with examples |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | Verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/capabilities_name.ts` | New capabilities implementation file |
| `templates/contract_name_protocol.ts` | New protocol interface definition |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Does this implement protocol behavior?"**

If yes → keep as capabilities. If no → check if it's orchestration (→ agent), domain data (→ taxonomy), or pure technical mechanics (→ utility).

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

No forbidden imports, no inter-capability dependencies, no business logic leakage.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# Check forbidden imports (no infrastructure, no agent, no other capabilities)
grep -n "^\s*from\s+.*infrastructure_\|^\s*from\s+.*agent_\|^\s*from\s+.*capabilities_" packages/*/src/capabilities_*.ts

# List protocol interface implementations
grep -n "implements I[A-Za-z0-9_]*Protocol" packages/*/src/capabilities_*.ts
```

## Common Mistakes

- Importing other capabilities directly.
- Defining domain data interfaces in capabilities files.
- Using concrete service types as constructor fields.
- Putting private helpers in the protocol interface.
- Putting constructors in the protocol interface.
- Placing utility methods before the domain protocol methods.
- Mixing Block 2 and Block 3 responsibilities.
- Silent error swallowing with `?? ''` or `|| 0`.
- Magic constants in capabilities logic.
- Not delegating low-level technical operations to Utility.
