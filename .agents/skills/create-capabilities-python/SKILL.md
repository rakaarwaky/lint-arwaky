---
name: create-capabilities-python
description: "Create and validate Python capabilities layer files following AES rules: pure domain behavior, zero I/O, 3-block structure, one class per file, protocol ABC contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    python,
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
  - "create capability python"
  - "add capability python"
  - "fix capability structure python"
  - "create protocol python"
  - "capability missing protocol python"
  - "check capabilities python"
  - "audit capabilities python"
dependencies: []
related:
  - create-infrastructure-python
  - create-agent-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - module_logic_validator-python
  - fix-capability-structure-python
  - create-missing-protocols-python
---

# create-capabilities-python

## Purpose

Create and validate Python **capabilities layer** files following clean architecture / AES rules.

A capabilities file must contain **pure domain behavior**:

- no I/O, no infrastructure detail, no agent detail,
- no locally defined domain data structures,
- one implementation class per file,
- one domain protocol ABC as the public contract,
- strict 3-block structure,
- dependency injection for service collaborators,
- shared VOs for domain data.

## Definition of Done

1. ONE implementation class per file.
2. Class inherits ONE domain protocol ABC.
3. Block 2 contains ONLY domain protocol method implementations.
4. Dunder methods, factory classmethods, private helpers in Block 3.
5. Zero I/O, zero side-effecting infrastructure calls.
6. No locally defined domain data structures.
7. Service dependencies use DI via protocol interfaces.
8. Value/configuration fields use shared VOs.
9. Reusable, stateless, domain-agnostic functions extracted to `*_utility.py`.
10. `python -c "import <module>"` passes.

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
| `templates/capabilities_name.py` | New capabilities implementation file |
| `templates/contract_name_protocol.py` | New protocol ABC definition |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Is this pure domain behavior?"**

If yes → keep as capabilities. If no → move I/O to infrastructure.

### Step 2: Check Missing Protocol (AES403)

Does the capability class inherit a protocol ABC? If no → create one.

### Step 3: Create Protocol File if Missing

Create `contract_<name>_protocol.py` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: class definition + `__init__` → protocol methods → dunders/factories/helpers.

### Step 5: Verify Class Discipline

One class, no local data classes, DI via protocol interfaces, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no I/O, no business logic leakage.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
python -c "import <module>"
```

## Quick Commands

```bash
# Check I/O in capabilities (AES404)
grep -n "open(\|Path(\|os\." modules/*/src/capabilities_*.py

# Check forbidden imports
grep -n "^\s*from\s+.*(infrastructure_|agent_)" modules/*/src/capabilities_*.py

# List protocol ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Protocol" modules/*/src/capabilities_*.py
```

## Common Mistakes

- Putting I/O in capabilities.
- Defining domain data classes in capabilities files.
- Using concrete service types as constructor fields.
- Putting private helpers in the protocol ABC.
- Putting constructors in the protocol ABC.
- Placing dunder methods before the domain protocol methods.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Silent error swallowing with `or ""` or `or 0`.
- Magic constants in capabilities logic.
