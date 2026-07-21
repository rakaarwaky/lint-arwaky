---
name: create-agent-python
description: "Create and validate Python agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, one class per file, aggregate ABC contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    python,
    aes,
    agent,
    aggregate,
    structure,
    3-block-structure,
    di,
    orchestration,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create agent python"
  - "add agent python"
  - "fix agent structure python"
  - "create aggregate python"
  - "agent missing aggregate python"
  - "validate agent logic python"
  - "check agent python"
  - "audit agent python"
dependencies: []
related:
  - create-capabilities-python
  - create-taxonomy-python
  - enforce-1-class-per-file-python
  - trait-consolidation-python
  - module_logic_validator-python
  - fix-agent-di
---

# create-agent-python

## Purpose

Create and validate Python **agent layer** files following AES rules.

An agent file contains **orchestration / pipeline execution only**.

Agents coordinate capabilities into executable flows. They control sequence and movement, not business calculation.

Agents MUST NOT contain I/O, business logic, domain rules, domain computation, or domain data definitions.

Agents depend ONLY on Taxonomy, Contract, and Utility layers. They must be completely ignorant of Capabilities implementations.

## Definition of Done

1. ONE implementation class per file.
2. Class inherits ONE domain aggregate ABC.
3. Block 2 contains ONLY aggregate ABC method implementations.
4. Dunder methods, factory classmethods, private helpers in Block 3.
5. Zero I/O, zero business logic, zero domain computation.
6. No locally defined domain data structures.
7. Service dependencies use DI via aggregate/protocol interfaces.
8. Value/configuration fields use shared VOs.
9. Aggregate signatures use shared VOs for domain data.
10. `python -c "import <module>"` passes.

## References

| File | Content |
|------|---------|
| `references/layer-boundaries.md` | Allowed/Forbidden imports and dependencies |
| `references/3-block-structure.md` | Block 1/2/3 definitions, method placement rules |
| `references/helper-vs-utility.md` | Helper vs utility decision, I/O Blocker, decision tree |
| `references/computation-detection.md` | Computation detection rules |
| `references/error-handling.md` | Error handling rules |
| `references/primitive-vo-policy.md` | Primitive policy table, VO rules |
| `references/examples.md` | All BAD/GOOD code examples |
| `references/commands.md` | Quick heuristic check commands |
| `references/checklist.md` | Verification checklist |

## Templates

| File | Purpose |
|------|---------|
| `templates/agent_name.py` | New agent implementation file |
| `templates/contract_name_aggregate.py` | New aggregate ABC definition |

## Workflow

### Step 1: Analyze File

Read the file and ask: **"Is this orchestration only?"**

If yes → keep as agent. If it contains computation → capabilities, domain data → taxonomy.

### Step 2: Check for Missing Aggregate

Does the agent class inherit an aggregate ABC? If no → create one.

### Step 3: Create Aggregate File if Missing

Create `contract_<name>_aggregate.py` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: class definition + `__init__` → aggregate methods → dunders/factories/helpers.

### Step 5: Verify Class Discipline

One class, no local data classes, DI via protocol interfaces, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no I/O, no business logic, no domain computation.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
python -c "import <module>"
```

## Quick Commands

```bash
# List aggregate ABC implementations
grep -n "class.*I[A-Za-z0-9_]*Aggregate" modules/*/src/agent_*.py

# Check computation patterns
grep -n "sum(\|len(\|\.iter\(\)\|\.map(" modules/*/src/agent_*.py

# Check forbidden imports (agent must only depend on taxonomy + contract + utility)
grep -n "^\s*from\s+.*(capabilities_|infrastructure_)" modules/*/src/agent_*.py
```

## Common Mistakes

- Putting domain computation in agents.
- Putting business logic in agents.
- Putting I/O in agents.
- Defining domain data classes in agent files.
- Using concrete service types as constructor fields.
- Putting private helpers in the aggregate ABC.
- Placing dunder methods before the aggregate ABC methods.
- Mixing Block 2 and Block 3 responsibilities.
- Silent error swallowing with `or ""` or `or 0`.
- Magic constants in agent logic.
