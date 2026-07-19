---
name: create-agent-typescript
description: "Create and validate agent layer files following AES rules: 3-block structure, one class per file, aggregate interfaces, zero computation/I/O/business logic."
version: 1.0.0
category: refactoring
tags:
  [
    typescript,
    aes,
    agent,
    aggregate,
    structure,
    3-block-structure,
    di,
    orchestration,
  ]
triggers:
  - "create agent typescript"
  - "add agent typescript"
  - "fix agent structure typescript"
  - "create aggregate typescript"
  - "agent missing aggregate typescript"
  - "validate agent logic typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-infrastructure-typescript
  - enforce-1-class-per-file-typescript
  - interface-consolidation-typescript
  - module_logic_validator-typescript
  - fix-agent-di-typescript
---

# create-agent-typescript

## Purpose

Create and validate TypeScript **agent layer** files following clean architecture rules. Ensures agents contain zero computation, zero I/O, and zero business logic — they are orchestration/pipeline execution only. Agents implement aggregate interfaces, follow the 3-Block Structure, and use DI for all fields.

## Rules

### Layer Boundaries (AES)

**Agent Layer (`agent_*.ts`)**

| Allowed                                          | Forbidden                                  |
| ------------------------------------------------ | ------------------------------------------ |
| `for`, `while`, `do-while` (orchestration flow)  | Computation (`reduce()`, `length`, arithmetic) |
| `if/else`, `switch` (control flow)               | Business rules, domain logic               |
| `try/catch`, `throw` (error propagation)         | File I/O (`fs.`, `readFile`, `writeFile`)  |
| `Promise`, `async/await` (async)                 | Network (`fetch`, `axios`, `http`)         |
| Sequential statements (orchestration)            | Database (`sqlite3`, `pg`, `mongoose`)     |
| Interface implementation                         | Domain model definition (`interface`)      |
|                                                  | Direct import from `capabilities_*`        |
|                                                  | Direct import from `infrastructure_*`      |

### Structural Rules (All Layers)

- **1 file = 1 class** — each agent file contains exactly ONE main class
- **All data types in shared** — no interfaces/types may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions should be extracted to `*_utility.ts` modules in shared/taxonomy

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. `class Definition` (class definition with DI fields)
2. `class Methods` implementing Aggregate (Public Contract)
3. `private` and Helper methods

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic functions MUST be extracted OUT of the class into their own `*_utility.ts` modules in shared/taxonomy. They do NOT belong in Block 3.

### Aggregate Rules

- **Every agent class MUST implement an aggregate interface**
- **Aggregate MUST define methods for all public methods**
- **Aggregate contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (private methods)
- **Constructors in class body** — constructor receives aggregate interfaces

## The Fundamental Question

> **"Is this file orchestration/pipeline execution only?"**

If yes → **`agent_*.ts` + implement aggregate interface**
If no (has computation, I/O, or business logic) → **split into appropriate layer**

## Naming Convention

| Layer              | File Pattern          | Interface File                | Interface Name    |
| ------------------ | --------------------- | ----------------------------- | ----------------- |
| **Capabilities**   | `capabilities_*.ts`   | `contract_<name>_protocol.ts` | `I<Name>Protocol` |
| **Infrastructure** | `infrastructure_*.ts` | `contract_<name>_port.ts`     | `I<Name>Port`     |
| **Agents**         | `agent_*.ts`          | `contract_<name>_aggregate.ts`| `I<Name>Aggregate`|

## Agent Layer Purpose

Agents are the **orchestration layer** — they coordinate between capabilities and infrastructure but contain:

- **NO computation** (no arithmetic, no data transformation)
- **NO business logic** (no domain rules, no validation)
- **NO I/O** (no file reads, no network calls, no database queries)

Their sole purpose is to orchestrate pipeline execution by calling into capabilities and infrastructure.

## Detection Patterns

### BAD: Computation in Agent

```typescript
// BAD: Computation in agent layer
class OrphanOrchestrator {
    process() {
        const total = this.files.length;  // ← COMPUTATION — should be in capabilities
        const sum = this.files.reduce((acc, f) => acc + f.size, 0);  // ← FORBIDDEN
    }
}
```

### BAD: Business Logic in Agent

```typescript
// BAD: Domain logic in agent layer
class OrphanOrchestrator {
    analyze(content: string): boolean {
        return content.includes("orphan");  // ← BUSINESS RULE — should be in capabilities
    }
}
```

### BAD: Interface in Layer File

```typescript
// BAD: Domain data defined in agent layer
interface OrphanReport {  // ← INTERFACE — should be in shared/taxonomy
    results: string[];
    timestamp: number;
}

class OrphanOrchestrator {
    report: OrphanReport;  // ← concrete type, not DI
}
```

### GOOD: Class with Shared Data

```typescript
// GOOD: All data from shared, fields use DI
import { ICapabilitiesOrphanProtocol } from '../contract/orphan_protocol';

class OrphanOrchestrator {
    constructor(private analyzer: ICapabilitiesOrphanProtocol) {}  // ← DI via interface
}
```

## Workflow

### Step 1: Analyze File

Read file and check for prohibited content. Ask: **"Is this orchestration only?"**

- If it has computation → **MOVE to Capabilities**
- If it has I/O or business logic → **split into appropriate layer**
- If pure orchestration → continue to Step 2

### Step 2: Check for Missing Aggregate

Does the agent class implement an aggregate interface? If no → create one.

```bash
# Find agents without aggregate implementations
grep -rn "^class " packages/*/src/agent_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Aggregate" "$file" || echo "MISSING: $file has $class without aggregate"
done
```

### Step 3: Create Aggregate File (if missing)

Create `contract_<name>_aggregate.ts` in the shared package with interface methods.

**Aggregate location:**

| Package     | Aggregate Path                                          |
| ----------- | ------------------------------------------------------- |
| compositor  | `packages/shared/src/compositor/contract_*_aggregate.ts`|
| animator    | `packages/shared/src/animator/contract_*_aggregate.ts`  |
| scripting   | `packages/shared/src/scripting/contract_*_aggregate.ts` |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `class <Type>` (class definition with DI fields)
2. Interface methods implementing Aggregate (public contract)
3. `private` and helper methods

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All interfaces in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use interfaces** — constructor receives aggregate interfaces, not concrete types
- **No standalone functions remain in Block 3** — extract to `*_utility.ts` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and prohibited patterns:

```bash
# Check for computation in agents
grep -n "\.length\|\.reduce\|\.map\|\.filter" packages/*/src/agent_*.ts

# Check for forbidden imports
grep -n "capabilities_\|infrastructure_" packages/*/src/agent_*.ts
```

### Step 7: Verify

Run TypeScript compiler to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class -> Aggregate Methods -> Helpers).
- [ ] Agent class implements an aggregate interface.
- [ ] Aggregate contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3 (private methods).
- [ ] Constructors receive aggregate interfaces via constructor.
- [ ] No standalone functions remain in class — extracted to `*_utility.ts` modules.
- [ ] Stateless utilities exist in their own `*_utility.ts` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All interfaces imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use aggregate interfaces, not concrete types.
- [ ] **Zero computation** in agent layer (no reduce(), no length, no iteration transforms).
- [ ] **Zero I/O** in agent layer (no fs, no network, no database).
- [ ] **Zero business logic** in agent layer (no domain rules, no validation).
- [ ] No forbidden imports (no capabilities__, no infrastructure__).
- [ ] Aggregate module is registered in the shared package's `index.ts`.
- [ ] `tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^    \|^private " packages/<package>/src/agent_*.ts

# Find agents without aggregate implementations
grep -rn "^class " packages/*/src/agent_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Aggregate" "$file" || echo "MISSING: $file has $class without aggregate"
done

# Ensure aggregate does NOT contain helper methods
grep -E "(helper|util|private|_)" packages/shared/src/contract_*_aggregate.ts || echo "Clean: No helpers in aggregate"

# Check for computation in agents
grep -n "\.length\|\.reduce\|\.map\|\.filter" packages/*/src/agent_*.ts

# Check for I/O in agents
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/agent_*.ts

# Check for business logic in agents
grep -n "is_orphan\|analyze\|validate" packages/*/src/agent_*.ts

# Check for interfaces defined in layer files
grep -rn "^interface\|^type " packages/*/src/ | grep -v "shared/" | grep agent

# Check for concrete type fields (non-interface)
grep -n "constructor" packages/*/src/agent_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "constructor" "$file" | grep -v "I[A-Z].*:" || echo "NON-INTERFACE FIELD: $file"
done

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Putting computation in agents**: Arithmetic, reduce(), length, and data transformation MUST be in capabilities layer.
- ❌ **Putting I/O in agents**: File reads, network calls, and database queries MUST be in infrastructure layer.
- ❌ **Putting business logic in agents**: Domain rules, validation, and computation MUST be in capabilities layer.
- ❌ **Defining interfaces in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive aggregate interfaces, not concrete implementations.
- ❌ **Putting helper methods in the aggregate**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave aggregate methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions MUST be extracted to standalone `*_utility.ts` modules.
- ❌ **Creating "God Aggregates"**: If an aggregate has >10 methods or mixes unrelated concerns, split it into multiple aggregates.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-typescript` if merging multiple files.
