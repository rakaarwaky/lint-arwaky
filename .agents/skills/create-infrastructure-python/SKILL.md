---
name: create-infrastructure-python
description: "Create and validate Python infrastructure layer files following AES rules: I/O and external integration only, zero business logic, 3-block structure, one class per file, port ABC contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    python,
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
  - "create infrastructure python"
  - "add infrastructure python"
  - "fix infrastructure structure python"
  - "create port python"
  - "infrastructure missing port python"
  - "check infrastructure python"
  - "audit infrastructure python"
dependencies: []
related:
  - create-capabilities-python
  - create-agent-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - module_logic_validator-python
  - fix-infrastructure-structure-python
  - create-missing-ports-python
---

# create-infrastructure-python

## Purpose

Create and validate Python **infrastructure layer** files following clean architecture / AES rules.

An infrastructure file must contain **I/O and external system integration only**:

- file system access, network calls, database access, external API calls,
- environment/system integration, technical mapping, serialization/deserialization,
- error mapping, adapter implementation for port ABCs.

Infrastructure MUST NOT contain business logic.

## Definition of Done

1. ONE implementation class per file.
2. Class inherits ONE domain port ABC.
3. Block 2 contains ONLY port ABC method implementations.
4. Dunder methods, factory classmethods, private helpers in Block 3.
5. Zero business logic.
6. No locally defined domain data structures.
7. Service dependencies use DI via protocol interfaces.
8. Value/configuration fields use shared VOs.
9. I/O errors are propagated explicitly.
10. `python -c "import <module>"` passes.

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
| `templates/infrastructure.py` | New infrastructure implementation file |
| `templates/port.py` | New port ABC definition |

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask: **"Is this pure I/O or external system integration?"**

If yes → keep as infrastructure. If it contains business logic → move to capabilities.

### Step 2: Check for Missing Port

Does the infrastructure class inherit a port ABC? If no → create one.

### Step 3: Create Port File if Missing

Create `contract_<name>_port.py` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: class definition + `__init__` → port methods → dunders/factories/helpers.

### Step 5: Verify Class Discipline

One class, no local data classes, DI via port interfaces, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no business logic, no local domain data definitions.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
python -c "import <module>"
```

## Quick Commands

```bash
# List port ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Port" modules/*/src/infrastructure_*.py

# Check business logic keywords
grep -n "is_orphan\|analyze\|validate\|calculate\|compute\|business" modules/*/src/infrastructure_*.py

# Check forbidden imports
grep -n "^\s*from\s+.*(capabilities_|agent_)" modules/*/src/infrastructure_*.py
```

## Common Mistakes

- Putting business logic in infrastructure.
- Defining domain data classes in infrastructure files.
- Using concrete service types as constructor fields.
- Putting private helpers in the port ABC.
- Putting constructors in the port ABC.
- Placing dunder methods before the port ABC methods.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Silent error swallowing with `or ""` or `or 0`.
- Magic constants in infrastructure logic.
- Infrastructure returning lint results directly.
