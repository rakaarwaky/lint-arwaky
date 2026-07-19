---
name: create-agent-typescript
description: "Create and validate agent layer files following AES rules: 3-block structure, one class per file, aggregate contracts, zero computation/I/O/business logic."
version: 1.1.0
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
    utility-extraction,
  ]
triggers:
  - "create agent typescript"
  - "add agent typescript"
  - "fix agent structure typescript"
  - "create aggregate typescript"
  - "agent missing aggregate typescript"
  - "validate agent logic typescript"
  - "extract utility typescript"
  - "free function typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-infrastructure-typescript
  - enforce-1-class-per-file-typescript
  - trait-consolidation-typescript
  - module_logic_validator-typescript
  - fix-agent-di-typescript
---

# create-agent-typescript

## Purpose

Create and validate TypeScript **agent layer** files following clean architecture rules. Ensures agents contain zero computation, zero I/O, and zero business logic — they are orchestration/pipeline execution only. Agents implement aggregate interfaces, follow the 3-Block Structure, use DI for all fields, and extract stateless free functions to taxonomy utility modules.

## Rules

### Layer Boundaries (AES)

**Agent Layer (`agent_*.ts`)**

| Allowed                                          | Forbidden                                        |
| ------------------------------------------------ | ------------------------------------------------ |
| `for`, `while`, `do-while` (orchestration flow)  | Computation (`reduce()`, `length`, arithmetic)   |
| `if/else`, `switch` (control flow)               | Business rules, domain logic                     |
| `try/catch`, `throw` (error propagation)         | File I/O (`fs.`, `readFile`, `writeFile`)        |
| `Promise`, `async/await` (async)                 | Network (`fetch`, `axios`, `http`)               |
| Sequential statements (orchestration)            | Database (`sqlite3`, `pg`, `mongoose`)           |
| Interface implementation                         | Domain model definition (`interface`)            |
|                                                  | Direct import from `capabilities_*`              |
|                                                  | Direct import from `infrastructure_*`            |

### Structural Rules (All Layers)

- **1 file = 1 class** — each agent file contains exactly ONE main class
- **All data types in shared** — no interfaces/types/enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive aggregate interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions MUST be extracted to `*_utility.ts` modules in shared/taxonomy
- **No module-level `export function` in agent files** — free functions outside the class are forbidden; extract to `*_utility.ts`

### The 3-Block Structure

Every implementation file MUST follow this exact order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
   - `export class <Type> implements I<Name>Aggregate {` declaration
   - `constructor(...)` with DI fields (aggregate interfaces)
   - Private field declarations (`private readonly _field: IAggregate`)

2. **Block 2 — Aggregate Methods** (Public Contract)
   - Methods that satisfy the `I<Name>Aggregate` interface signatures.
   - Contains **ONLY** the domain aggregate methods.
   - **NO** utility methods (`toString()`, `toJSON()`, `equals()`) here.
   - **NO** static factory methods (`create()`, `from()`, `of()`) here.
   - **NO** `private` helper methods here.

3. **Block 3 — Utility Methods, Factories & Helpers**
   - Utility/serialization methods: `toString()`, `toJSON()`, `valueOf()`, `equals()`
   - Symbol methods: `[Symbol.iterator]()`, `[Symbol.toPrimitive]()`
   - Static factory methods: `static create()`, `static from()`, `static of()`
   - `private` helper methods that use `this`
   - `private static` helpers that use class-level state

**CRITICAL:** Block 2 is **RESERVED** for domain aggregate methods ONLY. Utility methods (`toString()`, `toJSON()`, `equals()`) and static factory methods belong in **Block 3** because they are utilities/constructors, not the public domain contract.

**CRITICAL:** Stateless free functions (no `this`, no class-level state) MUST be extracted OUT of the class into their own `*_utility.ts` modules in shared/taxonomy. They do NOT belong in Block 3, Block 2, or at module level in agent files.

#### Method Placement Decision Rule

```
Method / function found in an agent file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in agent)
  │
  ├─ Defined in the I<Name>Aggregate interface?
  │   └─ YES → Block 2 (Aggregate Methods)
  │
  ├─ Utility/serialization method? (toString, toJSON, valueOf, equals)
  │   └─ YES → Block 3 (Utility Methods & Helpers)
  │
  ├─ Symbol method? ([Symbol.iterator], [Symbol.toPrimitive])
  │   └─ YES → Block 3 (Utility Methods & Helpers)
  │
  ├─ Static factory method? (static create, static from, static of)
  │   └─ YES → Block 3 (Utility Methods & Helpers)
  │
  ├─ Static method?
  │   ├─ Uses class-level state (static fields)?
  │   │   └─ YES → Block 3 (keep as private static)
  │   ├─ Tightly coupled to class semantics?
  │   │   └─ YES → Block 3 (keep as static)
  │   └─ Pure logic, no class dependency?
  │       └─ YES → EXTRACT to *_utility.ts
  │
  └─ Private instance method (uses this)?
      └─ YES → Block 3 (Private Helpers)
```

#### Example: Correct 3-Block Order

```typescript
import { ICapabilitiesOrphanProtocol } from '../shared/orphan_detector/contract_orphan_protocol';
import { IOrphanOrchestratorAggregate } from '../shared/orphan_detector/contract_orphan_aggregate';
import { LintResult } from '../shared/code_analysis/taxonomy_result_vo';


// ─── Block 1: Class Definition & Constructor ──────────────
export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {
    constructor(
        private readonly _analyzer: ICapabilitiesOrphanProtocol,
    ) {}


    // ─── Block 2: Aggregate Methods (domain contract ONLY) ─
    execute(files: string[]): LintResult[] {
        const violations: LintResult[] = [];
        for (const file of files) {
            try {
                const result = this._analyzer.analyze(file);
                violations.push(result);
            } catch (e) {
                violations.push(LintResult.newArch(
                    file, 0, 'ANALYZE_ERROR', String(e),
                ));
            }
        }
        return violations;
    }


    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'OrphanOrchestrator()';
    }

    equals(other: unknown): boolean {
        return other instanceof OrphanOrchestrator;
    }

    static create(): OrphanOrchestrator {
        const { CapabilitiesOrphanAnalyzer } = require('../shared/orphan_detector/capabilities_orphan_analyzer');
        return new OrphanOrchestrator(new CapabilitiesOrphanAnalyzer());
    }
}
```

#### Example: Extracted Utility Module

```typescript
// shared/orphan_detector/taxonomy_orphan_utility.ts
/**
 * Stateless utility functions for orphan detection logic.
 */

export const ORPHAN_MARKERS: readonly string[] = ['orphan', 'deprecated', 'unused'] as const;

export function isOrphanCandidate(filename: string): boolean {
    const lower = filename.toLowerCase();
    return ORPHAN_MARKERS.some(marker => lower.includes(marker));
}

export function formatViolationMessage(code: string, detail: string): string {
    return `[${code}] ${detail}`;
}
```

### Aggregate Rules

- **Every agent class MUST implement an aggregate interface**
- **Aggregate MUST define methods for all public methods**
- **Aggregate contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`private` methods)
- **Constructors in Block 1** — `constructor` receives aggregate interfaces
- **Utility methods (`toString`, `toJSON`, `equals`) in Block 3**
- **Static factory methods (`create`, `from`, `of`) in Block 3**
- **Stateless `static` methods (no class dependency) → extract to `*_utility.ts`**

## The Fundamental Question

> **"Is this file orchestration/pipeline execution only?"**

If yes → **`agent_*.ts` + implement aggregate interface**
If no (has computation, I/O, or business logic) → **split into appropriate layer**

> **"Does this function need the class?"**

If no (`this` / class state unused) → **extract to `*_utility.ts` in shared/taxonomy**
If yes → **keep in Block 3**

## Naming Convention

| Layer                    | File Pattern            | Interface File                     | Interface Name        |
| ------------------------ | ----------------------- | ---------------------------------- | --------------------- |
| **Capabilities**   | `capabilities_*.ts`   | `contract_<name>_protocol.ts`    | `I<Name>Protocol`   |
| **Infrastructure** | `infrastructure_*.ts` | `contract_<name>_port.ts`        | `I<Name>Port`       |
| **Agents**         | `agent_*.ts`          | `contract_<name>_aggregate.ts`   | `I<Name>Aggregate`  |
| **Utility**        | `taxonomy_<name>_utility.ts` | —                            | — (free functions)  |

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

### BAD: Utility Methods in Block 2

```typescript
// BAD: toString / equals mixed in with aggregate methods
export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {
    constructor(private readonly _analyzer: ICapabilitiesOrphanProtocol) {}

    toString(): string {                    // ← Block 2 position, NOT an aggregate method
        return 'OrphanOrchestrator()';
    }

    execute(files: string[]): LintResult[] { ... }  // ← pushed down

    equals(other: unknown): boolean {       // ← also in Block 2 position
        return other instanceof OrphanOrchestrator;
    }
}
```

### BAD: Module-Level Free Function in Agent File

```typescript
// BAD: Free function outside class in agent file
// agent_orphan_orchestrator.ts

export function isOrphanCandidate(filename: string): boolean {   // ← FREE FUNCTION — extract to utility
    return filename.toLowerCase().includes('orphan');
}

export function formatViolationMessage(code: string, detail: string): string {  // ← FREE FUNCTION
    return `[${code}] ${detail}`;
}

export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {
    execute(files: string[]): LintResult[] {
        if (isOrphanCandidate(files[0])) {  // ← could be imported from utility
            ...
        }
    }
}
```

### BAD: Stateless Static Method That Should Be Extracted

```typescript
// BAD: static method with zero class dependency — belongs in utility
export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {

    static isOrphanCandidate(filename: string): boolean {    // ← no this, no class state, pure logic
        return filename.toLowerCase().includes('orphan');
    }

    static formatMessage(code: string, detail: string): string {  // ← no this, no class state, pure logic
        return `[${code}] ${detail}`;
    }

    execute(files: string[]): LintResult[] {
        if (OrphanOrchestrator.isOrphanCandidate(files[0])) {   // ← could be a free function
            ...
        }
    }
}
```

### GOOD: Class with Shared Data

```typescript
// GOOD: All data from shared, fields use DI
import { ICapabilitiesOrphanProtocol } from '../contract/orphan_protocol';

export class OrphanOrchestrator {
    constructor(
        private readonly _analyzer: ICapabilitiesOrphanProtocol,  // ← DI via interface
    ) {}
}
```

### GOOD: Correct 3-Block with Utility Methods

```typescript
// GOOD: Aggregate methods in Block 2, utility + factories in Block 3
export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {

    constructor(private readonly _analyzer: ICapabilitiesOrphanProtocol) {}  // Block 1: constructor

    execute(files: string[]): LintResult[] { ... }                           // Block 2: aggregate method ONLY

    toString(): string {                                                     // Block 3: utility method
        return 'OrphanOrchestrator()';
    }

    static create(): OrphanOrchestrator {                                    // Block 3: factory
        ...
    }

    private handleError(e: unknown, file: string): LintResult {             // Block 3: private helper
        return LintResult.newArch(file, 0, 'ERROR', String(e));
    }
}
```

### GOOD: Extracted to Taxonomy Utility

```typescript
// GOOD: shared/orphan_detector/taxonomy_orphan_utility.ts

export const ORPHAN_MARKERS: readonly string[] = ['orphan', 'deprecated', 'unused'] as const;

export function isOrphanCandidate(filename: string): boolean {
    const lower = filename.toLowerCase();
    return ORPHAN_MARKERS.some(marker => lower.includes(marker));
}
```

```typescript
// GOOD: agent_orphan_orchestrator.ts (consumer)

import { isOrphanCandidate } from '../shared/orphan_detector/taxonomy_orphan_utility';

export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {

    execute(files: string[]): LintResult[] {
        if (isOrphanCandidate(files[0])) {    // ← imported from utility
            ...
        }
    }
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
grep -rn "^export class \|^class " packages/*/src/agent_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "implements.*Aggregate" "$file" || echo "MISSING: $file has $class without aggregate"
done
```

### Step 3: Create Aggregate File (if missing)

Create `contract_<name>_aggregate.ts` in the shared package with interface methods.

**Aggregate location:**

| Package    | Aggregate Path                                              |
| ---------- | ----------------------------------------------------------- |
| compositor | `packages/shared/src/compositor/contract_*_aggregate.ts`  |
| animator   | `packages/shared/src/animator/contract_*_aggregate.ts`    |
| scripting  | `packages/shared/src/scripting/contract_*_aggregate.ts`   |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order within the class body:

1. `export class <Type> implements I<Name>Aggregate` + `constructor` (class definition with DI fields)
2. Interface method implementations (**domain aggregate methods ONLY**)
3. Utility methods (`toString`, `toJSON`, `equals`), static factories (`create`, `from`), `private` helpers

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All interfaces/types in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use interfaces** — constructor receives aggregate interfaces, not concrete types
- **No standalone functions remain in Block 3** — extract to `*_utility.ts` modules

### Step 6: Extract Free Functions to Utility

Scan the file for functions that have **no `this` / class-state dependency**:

```bash
# Find module-level export functions (outside class) — ALWAYS forbidden
grep -n "^export function \|^function " packages/*/src/agent_*.ts

# Find static methods inside class
grep -n "static " packages/*/src/agent_*.ts

# Find standalone exported constants that may belong in utility
grep -n "^export const " packages/*/src/agent_*.ts
```

For each candidate, ask:

| Question | YES → | NO → |
|----------|-------|------|
| Uses `this` or instance state? | Keep in Block 3 | Continue ↓ |
| Uses class-level `static` fields? | Keep as `private static` in Block 3 | Continue ↓ |
| Tightly coupled to class semantics (e.g., references class types)? | Keep as `static` in Block 3 | Continue ↓ |
| Pure logic, deterministic, no side effects? | **Extract to `*_utility.ts`** | Keep in Block 3 |
| Domain-agnostic (not specific to this class)? | **Extract to `*_utility.ts`** | Keep in Block 3 |

**Extraction process:**

1. Create `packages/shared/src/<domain>/taxonomy_<name>_utility.ts`
2. Move function(s) to utility file with JSDoc comments
3. Extract magic constants to `taxonomy_<name>_constant.ts` if needed
4. Add import in agent file: `import { funcName } from '../shared/<domain>/taxonomy_<name>_utility';`
5. Remove original function from agent file
6. Export from `index.ts` barrel if needed
7. Verify: `npx tsc --noEmit`

### Step 7: Verify Layer Compliance

Check forbidden imports and prohibited patterns:

```bash
# Check for computation in agents
grep -n "\.length\|\.reduce\|\.map\|\.filter" packages/*/src/agent_*.ts

# Check for forbidden imports
grep -n "capabilities_\|infrastructure_" packages/*/src/agent_*.ts
```

### Step 8: Verify

Run TypeScript compiler to confirm no violations.

```bash
npx tsc --noEmit
```

## Import Strategy

When deciding where a function belongs:

### Option A: Extract to Taxonomy Utility (Standalone Free Functions)

Use when the code is **stateless, pure logic** with no side effects:

| Condition                                     | Example                                       |
| --------------------------------------------- | --------------------------------------------- |
| No `this`, no class state                     | `isOrphanCandidate(name: string): boolean`    |
| All data via parameters                       | `formatViolationMessage(code, detail): string` |
| Deterministic, no side effects                | `normalizeFilename(name: string): string`     |

```typescript
// taxonomy_orphan_utility.ts (SHARED / TAXONOMY)
export function isOrphanCandidate(filename: string): boolean {
    return ORPHAN_MARKERS.some(m => filename.toLowerCase().includes(m));
}

// agent_orphan_orchestrator.ts (CONSUMER)
import { isOrphanCandidate } from '../shared/orphan_detector/taxonomy_orphan_utility';
```

### Option B: Keep as Instance/Static Method (Stateful or Side-Effectful)

Use when the code requires **instance state, class state, or side effects**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Uses `this` / instance fields | `this._analyzer.analyze(file)`                  |
| Uses class-level static state | `OrphanOrchestrator._registry[name]`            |
| Has side effects / I/O        | File operations, logging with context           |
| Tightly coupled to class semantics | References class-level types or constants  |

```typescript
// agent_orphan_orchestrator.ts (STAYS IN CLASS — Block 3)
export class OrphanOrchestrator implements IOrphanOrchestratorAggregate {
    private static readonly _MAX_RETRIES = 3;

    constructor(private readonly _analyzer: ICapabilitiesOrphanProtocol) {}

    private shouldRetry(attempt: number): boolean {   // uses this → stays
        return attempt < OrphanOrchestrator._MAX_RETRIES;
    }

    static fromConfig(config: IOrchestratorConfig): OrphanOrchestrator { // uses class state → stays
        return new OrphanOrchestrator(config.createAnalyzer());
    }
}
```

### Decision Tree

```
Function found in agent file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in agent)
  │
  ├─ Static method inside class?
  │   ├─ Pure logic, no class dependency?
  │   │   └─ YES → EXTRACT to *_utility.ts
  │   ├─ Factory (create, from, of)?
  │   │   └─ YES → Keep in Block 3
  │   ├─ Uses class-level static state?
  │   │   └─ YES → Keep in Block 3
  │   └─ Tightly coupled to class semantics?
  │       └─ YES → Keep in Block 3
  │
  ├─ Instance method?
  │   ├─ Defined in I<Name>Aggregate interface?
  │   │   └─ YES → Block 2
  │   ├─ Utility method (toString, toJSON, equals)?
  │   │   └─ YES → Block 3
  │   └─ Private helper (uses this)?
  │       └─ YES → Block 3
  │
  └─ Module-level export const (outside class)?
      ├─ Domain constant?
      │   └─ YES → EXTRACT to taxonomy_<name>_constant.ts
      └─ Class-specific config?
          └─ YES → Move inside class as private static readonly
```

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class + `constructor` → Aggregate Methods → Utility/Factories/Helpers).
- [ ] **Block 2 contains ONLY aggregate interface method implementations**. No utility methods (`toString`, `equals`), no static factories, no `private` helpers in Block 2.
- [ ] **Utility methods** (`toString`, `toJSON`, `equals`, `valueOf`) and **static factories** (`create`, `from`, `of`) are in **Block 3**.
- [ ] Agent class implements an aggregate interface.
- [ ] Interface contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3 (`private` methods).
- [ ] Constructors receive aggregate interfaces via `constructor`.
- [ ] **No module-level `export function`** exists outside the class in agent files.
- [ ] **No stateless `static` method** (zero class dependency) remains in class — extracted to `*_utility.ts`.
- [ ] Stateless utilities exist in their own `*_utility.ts` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All interfaces/types imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use aggregate interfaces, not concrete types.
- [ ] **Zero computation** in agent layer (no reduce(), no length, no iteration transforms).
- [ ] **Zero I/O** in agent layer (no fs, no network, no database).
- [ ] **Zero business logic** in agent layer (no domain rules, no validation).
- [ ] No forbidden imports (no capabilities\_\_, no infrastructure\_\_).
- [ ] Aggregate module is registered in the shared package's `index.ts`.
- [ ] Utility module is registered in the shared package's `index.ts`.
- [ ] `npx tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^export class\|^    [a-z]\|^    private \|^    static " packages/<package>/src/agent_*.ts

# Find agents without aggregate implementations
grep -rn "^export class \|^class " packages/*/src/agent_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "implements.*Aggregate" "$file" || echo "MISSING: $file has $class without aggregate"
done

# Ensure aggregate does NOT contain helper methods
grep -E "(helper|util|private|_)" packages/shared/src/contract_*_aggregate.ts || echo "Clean: No helpers in aggregate"

# Check for computation in agents
grep -n "\.length\|\.reduce\|\.map\|\.filter" packages/*/src/agent_*.ts

# Check for I/O in agents
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/agent_*.ts

# Check for business logic in agents
grep -n "is_orphan\|analyze\|validate" packages/*/src/agent_*.ts

# Check for interfaces/types defined in layer files
grep -rn "^interface \|^type \|^enum " packages/*/src/ | grep -v "shared/" | grep agent

# Check for concrete type fields (non-interface)
grep -n "constructor" packages/*/src/agent_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "constructor" "$file" | grep -v "I[A-Z].*:" || echo "NON-INTERFACE FIELD: $file"
done

# Find module-level free functions in agent files (ALWAYS forbidden)
grep -n "^export function \|^function " packages/*/src/agent_*.ts

# Find static methods that may need extraction (no class state)
grep -n "static " packages/*/src/agent_*.ts | grep -v "private static\|static create\|static from\|static of"

# Find module-level constants that should be in taxonomy
grep -n "^export const " packages/*/src/agent_*.ts

# Detect utility methods appearing BEFORE aggregate methods (wrong block order)
awk '
    /^    (toString|toJSON|valueOf|equals)\(/ { if (!util_line) util_line = NR }
    /^    [a-z][a-zA-Z]*\(/ && !/^    (toString|toJSON|valueOf|equals|constructor)\(/ { if (!agg_line) agg_line = NR }
    END { if (util_line && agg_line && util_line < agg_line) print "VIOLATION: utility method (line " util_line ") before aggregate method (line " agg_line ")" }
' packages/*/src/agent_*.ts

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Putting computation in agents**: Arithmetic, reduce(), length, and data transformation MUST be in capabilities layer.
- ❌ **Putting I/O in agents**: File reads, network calls, and database queries MUST be in infrastructure layer.
- ❌ **Putting business logic in agents**: Domain rules, validation, and computation MUST be in capabilities layer.
- ❌ **Defining interfaces/types in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive aggregate interfaces, not concrete implementations.
- ❌ **Putting helper methods in the aggregate**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave aggregate methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions MUST be extracted to standalone `*_utility.ts` modules.
- ❌ **Creating "God Aggregates"**: If an aggregate has >10 methods or mixes unrelated concerns, split it into multiple aggregates.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-typescript` if merging multiple files.
- ❌ **Placing utility methods (`toString`, `toJSON`, `equals`) in Block 2**: Block 2 is RESERVED for aggregate method implementations ONLY. Utility methods belong in Block 3.
- ❌ **Placing static factories (`create`, `from`, `of`) before aggregate methods**: Factories are constructors and belong in Block 3, after aggregate methods.
- ❌ **Leaving module-level `export function` in agent files**: Free functions outside the class MUST be extracted to `*_utility.ts` in shared/taxonomy. No exceptions.
- ❌ **Keeping stateless `static` methods in class**: If a `static` method uses no `this`, no class-level state, and is not a factory, it belongs in `*_utility.ts`, not in the class body.
- ❌ **Defining `export const` at module level in agent files**: Domain constants MUST live in `taxonomy_<name>_constant.ts` in shared/taxonomy.
