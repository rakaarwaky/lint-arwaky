---
name: create-agent-typescript
description: "Create and validate TypeScript agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, one class per file, aggregate interface contracts, DI for service dependencies, and shared VOs for domain data."
metadata:
    tags: [typescript, aes, agent, aggregate, structure, 3-block-structure, di, orchestration, vo]
    triggers:
        - "create agent typescript"
        - "add agent typescript"
        - "fix agent structure typescript"
        - "create aggregate typescript"
        - "agent missing aggregate typescript"
        - "validate agent logic typescript"
        - "check agent typescript"
        - "audit agent typescript"
    dependencies: []
    related:
        - create-capabilities-typescript
        - create-taxonomy-typescript
        - create-contract-typescript
---

# create-agent-typescript

## Purpose

Create and validate TypeScript **agent layer** files following AES rules.

An agent file contains **orchestration / pipeline execution only**.

Agents coordinate capabilities into executable flows. They control sequence and movement, not business calculation.

Agents MUST NOT contain I/O, business logic, domain rules, domain computation, or domain data definitions.

Agents depend ONLY on Taxonomy, Contract, and Utility layers. They must be completely ignorant of Capabilities implementations.

## Definition of Done

1. ONE implementation class per file.
2. Class implements ONE domain aggregate interface.
3. Block 2 contains ONLY aggregate interface method implementations.
4. Utility methods, static factories, private helpers in Block 3.
5. Zero I/O, zero business logic, zero domain computation.
6. No locally defined domain data structures.
7. Service dependencies use DI via aggregate/protocol interfaces.
8. Value/configuration fields use shared VOs.
9. Aggregate signatures use shared VOs for domain data.
10. `npx tsc --noEmit` passes.

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
| `templates/agent_name.ts` | New agent implementation file |
| `templates/contract_name_aggregate.ts` | New aggregate interface definition |

## Workflow

### Step 1: Analyze File

Read the file and ask: **"Is this orchestration only?"**

If yes → keep as agent. If it contains computation → capabilities, domain data → taxonomy.

### Step 2: Check for Missing Aggregate

Does the agent class implement an aggregate interface? If no → create one.

### Step 3: Create Aggregate File if Missing

Create `contract_<name>_aggregate.ts` in the appropriate shared domain folder.

### Step 4: Enforce 3-Block Structure

Reorganize: class definition + `constructor` → aggregate methods → utility/factories/helpers.

### Step 5: Verify Class Discipline

One class, no local data interfaces, DI via port interfaces, shared VOs.

### Step 6: Verify Helper vs Utility Boundary

See `references/helper-vs-utility.md` for the decision tree.

### Step 7: Verify Layer Compliance

No forbidden imports, no I/O, no business logic, no domain computation.

### Step 8: Verify Error Handling, VO, and Constants

See `references/error-handling.md` and `references/primitive-vo-policy.md`.

### Step 9: Verify Compilation

```bash
npx tsc --noEmit
```

## Quick Commands

```bash
# List aggregate interface implementations
grep -n "implements I[A-Za-z0-9_]*Aggregate" packages/*/src/agent_*.ts

# Check computation patterns
grep -n "\.length\|\.reduce\|\.map\|\.filter" packages/*/src/agent_*.ts

# Check forbidden imports (agent must only depend on taxonomy + contract + utility)
grep -n "^\s*from\s+.*capabilities_\|^\s*from\s+.*infrastructure_" packages/*/src/agent_*.ts
```

## Common Mistakes

- Putting domain computation in agents.
- Putting business logic in agents.
- Putting I/O in agents.
- Defining domain data interfaces in agent files.
- Using concrete service types as constructor fields.
- Putting private helpers in the aggregate interface.
- Placing utility methods before the aggregate interface methods.
- Mixing Block 2 and Block 3 responsibilities.
- Silent error swallowing with `?? ''` or `|| 0`.
- Magic constants in agent logic.
