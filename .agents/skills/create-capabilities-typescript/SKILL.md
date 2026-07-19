---
name: create-capabilities-typescript
description: "Create and validate capabilities layer files following AES rules: 3-block structure, one class per file, protocol contracts, zero I/O."
version: 1.1.0
category: refactoring
tags:
  [
    typescript,
    aes,
    capability,
    protocol,
    structure,
    aes403,
    aes404,
    3-block-structure,
    di,
    utility-extraction,
  ]
triggers:
  - "create capability typescript"
  - "add capability typescript"
  - "fix capability structure typescript"
  - "create protocol typescript"
  - "capability missing protocol typescript"
  - "check capabilities typescript"
  - "extract utility typescript"
  - "free function typescript"
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

Create and validate TypeScript **capabilities layer** files following clean architecture rules. Ensures capabilities contain zero I/O, implement protocol interfaces, follow the 3-Block Structure, use DI for all fields, and extract stateless free functions to taxonomy utility modules.

## Rules

### Layer Boundaries (AES)

**Capabilities Layer (`capabilities_*.ts`)**

| Allowed                               | Forbidden                                        |
| ------------------------------------- | ------------------------------------------------ |
| Computation, validation, calculation  | File I/O (`fs.`, `readFile`, `writeFile`)  |
| Data transformation, business rules   | Network calls (`fetch`, `axios`, `http`)     |
| Domain logic, domain model definition | Database operations (`sqlite3`, `pg`)        |
| Interface implementation              | Direct import from `infrastructure_*`          |
|                                       | Direct import from `agent_*`                   |
|                                       | Direct import from `capabilities_*` (self)     |

### Structural Rules (All Layers)

- **1 file = 1 class** — each capabilities file contains exactly ONE main class
- **All data types in shared** — no interfaces/types/enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions MUST be extracted to `*_utility.ts` modules in shared/taxonomy
- **No module-level `export function` in capabilities files** — free functions outside the class are forbidden; extract to `*_utility.ts`

### The 3-Block Structure

Every implementation file MUST follow this exact order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
   - `export class <Type> implements I<Name>Protocol {` declaration
   - `constructor(...)` with DI fields (protocol interfaces)
   - Private field declarations (`private readonly _field: IProtocol`)

2. **Block 2 — Protocol Methods** (Public Contract)
   - Methods that satisfy the `I<Name>Protocol` interface signatures.
   - Contains **ONLY** the domain protocol methods.
   - **NO** utility methods (`toString()`, `toJSON()`, `equals()`) here.
   - **NO** static factory methods (`create()`, `from()`, `of()`) here.
   - **NO** `private` helper methods here.

3. **Block 3 — Utility Methods, Factories & Helpers**
   - Utility/serialization methods: `toString()`, `toJSON()`, `valueOf()`, `equals()`
   - Symbol methods: `[Symbol.iterator]()`, `[Symbol.toPrimitive]()`
   - Static factory methods: `static create()`, `static from()`, `static of()`
   - `private` helper methods that use `this`
   - `private static` helpers that use class-level state

**CRITICAL:** Block 2 is **RESERVED** for domain protocol methods ONLY. Utility methods (`toString()`, `toJSON()`, `equals()`) and static factory methods belong in **Block 3** because they are utilities/constructors, not the public domain contract.

**CRITICAL:** Stateless free functions (no `this`, no class-level state) MUST be extracted OUT of the class into their own `*_utility.ts` modules in shared/taxonomy. They do NOT belong in Block 3, Block 2, or at module level in capabilities files.

#### Method Placement Decision Rule

```
Method / function found in a capabilities file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in capabilities)
  │
  ├─ Defined in the I<Name>Protocol interface?
  │   └─ YES → Block 2 (Protocol Methods)
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
import { LintResult } from '../shared/code_analysis/taxonomy_result_vo';
import { Severity } from '../shared/code_analysis/taxonomy_severity_vo';
import { ILineCheckerProtocol } from '../shared/code_analysis/contract_line_checker_protocol';
import { LayerDefinition } from '../shared/taxonomy_definition_vo';
import { isBarrelFile, countLines } from '../shared/code_analysis/taxonomy_line_checker_utility';


// ─── Block 1: Class Definition & Constructor ──────────────
export class ArchLineChecker implements ILineCheckerProtocol {
    constructor() {
        // stateless — no DI fields needed
    }


    // ─── Block 2: Protocol Methods (domain contract ONLY) ─
    checkLineCounts(
        file: string,
        definition: LayerDefinition | null,
        content: string,
        violations: LintResult[],
    ): void {
        const basename = file.split('/').pop() ?? '';

        if (isBarrelFile(basename)) {
            return;
        }

        if (definition === null) {
            return;
        }

        if (definition.exceptions.values.includes(basename)) {
            return;
        }

        const count = countLines(content);

        if (definition.codeAnalysis.minLines.value > 0 && count < definition.codeAnalysis.minLines.value) {
            violations.push(LintResult.newArch(
                file, 0, 'AES302', Severity.HIGH,
                `File too short (min: ${definition.codeAnalysis.minLines.value}).`,
            ));
        }

        if (definition.codeAnalysis.maxLines.value > 0 && count > definition.codeAnalysis.maxLines.value) {
            violations.push(LintResult.newArch(
                file, 0, 'AES301', Severity.HIGH,
                `File too large (max: ${definition.codeAnalysis.maxLines.value}).`,
            ));
        }
    }


    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'ArchLineChecker()';
    }

    equals(other: unknown): boolean {
        return other instanceof ArchLineChecker;
    }

    static create(): ArchLineChecker {
        return new ArchLineChecker();
    }
}
```

#### Example: Extracted Utility Module

```typescript
// shared/code_analysis/taxonomy_line_checker_utility.ts
/**
 * Stateless utility functions for line-count checking logic.
 */

export const BARREL_FILES: readonly string[] = ['__init__.py', 'mod.rs', 'index.ts'] as const;

export function isBarrelFile(basename: string): boolean {
    return BARREL_FILES.includes(basename);
}

export function countLines(content: string): number {
    return content.split('\n').length;
}

export function normalizePath(path: string): string {
    return path.trim().toLowerCase();
}
```

### Protocol Rules

- **Every capability class MUST implement a protocol interface** (AES403)
- **Protocol MUST define methods for all public methods**
- **Protocol contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`private` methods)
- **Constructors in Block 1** — `constructor` receives protocol interfaces
- **Utility methods (`toString`, `toJSON`, `equals`) in Block 3**
- **Static factory methods (`create`, `from`, `of`) in Block 3**
- **Stateless `static` methods (no class dependency) → extract to `*_utility.ts`**

## The Fundamental Question

> **"Is this file pure business logic?"**

If yes → **`capabilities_*.ts` + implement protocol interface**
If no (has I/O) → **split into infrastructure layer instead**

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

## Detection Patterns

### BAD: Capability Without Interface (AES403)

```typescript
// BAD: No interface implementation
class FrameComposer {
    composeFrame(): void { ... }
}
```

### BAD: Mixed Logic in Capabilities

```typescript
// BAD: I/O in capabilities layer
class MyCapability {
    process() {
        const content = fs.readFileSync("file.txt");  // ← FORBIDDEN
    }
}
```

### BAD: Interface in Layer File

```typescript
// BAD: Domain data defined in capabilities layer
interface OrphanResult {  // ← INTERFACE — should be in shared/taxonomy
    isOrphan: boolean;
    reason: string;
    severity: string;
}

class CapabilitiesOrphanAnalyzer {
    result: OrphanResult;  // ← concrete type, not DI
}
```

### BAD: Utility Methods in Block 2

```typescript
// BAD: toString / equals mixed in with protocol methods
export class ArchLineChecker implements ILineCheckerProtocol {
    constructor() {}

    toString(): string {                    // ← Block 2 position, NOT a protocol method
        return 'ArchLineChecker()';
    }

    checkLineCounts(...): void {            // ← pushed down
        ...
    }

    equals(other: unknown): boolean {       // ← also in Block 2 position
        return other instanceof ArchLineChecker;
    }
}
```

### BAD: Module-Level Free Function in Capabilities File

```typescript
// BAD: Free function outside class in capabilities file
// capabilities_line_checker.ts

export function normalizePath(path: string): string {   // ← FREE FUNCTION — extract to utility
    return path.trim().toLowerCase();
}

export function countLines(content: string): number {   // ← FREE FUNCTION — extract to utility
    return content.split('\n').length;
}

export class ArchLineChecker implements ILineCheckerProtocol {
    checkLineCounts(...): void {
        const normalized = normalizePath(file);
        ...
    }
}
```

### BAD: Stateless Static Method That Should Be Extracted

```typescript
// BAD: static method with zero class dependency — belongs in utility
export class ArchLineChecker implements ILineCheckerProtocol {

    static normalizePath(path: string): string {    // ← no this, no class state, pure logic
        return path.trim().toLowerCase();
    }

    static isBarrelFile(name: string): boolean {    // ← no this, no class state, pure logic
        return ['__init__.py', 'mod.rs'].includes(name);
    }

    checkLineCounts(...): void {
        if (ArchLineChecker.isBarrelFile(basename)) {   // ← could be a free function
            return;
        }
    }
}
```

### GOOD: Class with Shared Data

```typescript
// GOOD: All data from shared, fields use DI
import { OrphanIndicatorResult } from '../shared/code_analysis/taxonomy_analysis';
import { IOrphanFilenameExtractorProtocol } from '../contract/orphan_protocol';

export class CapabilitiesOrphanAnalyzer {
    constructor(
        private readonly extractor: IOrphanFilenameExtractorProtocol,  // ← DI via interface
    ) {}
}
```

### GOOD: Correct 3-Block with Utility Methods

```typescript
// GOOD: Protocol methods in Block 2, utility + factories in Block 3
export class ArchLineChecker implements ILineCheckerProtocol {

    constructor() {}                                   // Block 1: constructor

    checkLineCounts(...): void { ... }                 // Block 2: protocol method ONLY

    toString(): string {                               // Block 3: utility method
        return 'ArchLineChecker()';
    }

    static create(): ArchLineChecker {                 // Block 3: factory
        return new ArchLineChecker();
    }

    private resolveThreshold(layer: string): number {  // Block 3: private helper
        return this._config.getThreshold(layer);
    }
}
```

### GOOD: Extracted to Taxonomy Utility

```typescript
// GOOD: shared/code_analysis/taxonomy_line_checker_utility.ts

export const BARREL_FILES: readonly string[] = ['__init__.py', 'mod.rs', 'index.ts'] as const;

export function isBarrelFile(basename: string): boolean {
    return BARREL_FILES.includes(basename);
}

export function countLines(content: string): number {
    return content.split('\n').length;
}
```

```typescript
// GOOD: capabilities_line_checker.ts (consumer)

import { isBarrelFile, countLines } from '../shared/code_analysis/taxonomy_line_checker_utility';

export class ArchLineChecker implements ILineCheckerProtocol {

    checkLineCounts(...): void {
        if (isBarrelFile(basename)) {       // ← imported from utility
            return;
        }
        const count = countLines(content);  // ← imported from utility
        ...
    }
}
```

## Workflow

### Step 1: Analyze File

Read file and check for mixed responsibilities. Ask: **"Is this code in the right layer?"**

- If it has I/O → **MOVE to Infrastructure** (AES404)
- If pure business logic → continue to Step 2

### Step 2: Check for Missing Interface (AES403)

Does the capability class implement a protocol interface? If no → create one.

```bash
# Find capabilities without interface implementations
grep -rn "^export class \|^class " packages/*/src/capabilities_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "implements.*Protocol" "$file" || echo "MISSING: $file has $class without interface"
done
```

### Step 3: Create Interface File (if missing)

Create `contract_<name>_protocol.ts` in the shared package with interface methods.

**Interface location:**

| Package    | Interface Path                                             |
| ---------- | ---------------------------------------------------------- |
| compositor | `packages/shared/src/compositor/contract_*_protocol.ts`  |
| animator   | `packages/shared/src/animator/contract_*_protocol.ts`    |
| scripting  | `packages/shared/src/scripting/contract_*_protocol.ts`   |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order within the class body:

1. `export class <Type> implements I<Name>Protocol` + `constructor` (class definition with DI fields)
2. Interface method implementations (**domain protocol methods ONLY**)
3. Utility methods (`toString`, `toJSON`, `equals`), static factories (`create`, `from`), `private` helpers

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All interfaces/types in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use interfaces** — constructor receives protocol interfaces, not concrete types
- **No standalone functions remain in Block 3** — extract to `*_utility.ts` modules

### Step 6: Extract Free Functions to Utility

Scan the file for functions that have **no `this` / class-state dependency**:

```bash
# Find module-level export functions (outside class) — ALWAYS forbidden
grep -n "^export function \|^function " packages/*/src/capabilities_*.ts

# Find static methods inside class
grep -n "static " packages/*/src/capabilities_*.ts

# Find standalone exported constants that may belong in utility
grep -n "^export const " packages/*/src/capabilities_*.ts
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
4. Add import in capabilities file: `import { funcName } from '../shared/<domain>/taxonomy_<name>_utility';`
5. Remove original function from capabilities file
6. Export from `index.ts` barrel if needed
7. Verify: `npx tsc --noEmit`

### Step 7: Verify Layer Compliance

Check forbidden imports and I/O patterns:

```bash
# Check for I/O in capabilities
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/capabilities_*.ts

# Check for forbidden imports
grep -n "infrastructure_\|agent_" packages/*/src/capabilities_*.ts
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
| No `this`, no class state                     | `normalizePath(path: string): string`         |
| All data via parameters                       | `countLines(content: string): number`         |
| Deterministic, no side effects                | `isBarrelFile(name: string): boolean`         |

```typescript
// taxonomy_line_checker_utility.ts (SHARED / TAXONOMY)
export function isBarrelFile(basename: string): boolean {
    return BARREL_FILES.includes(basename);
}

// capabilities_line_checker.ts (CONSUMER)
import { isBarrelFile } from '../shared/code_analysis/taxonomy_line_checker_utility';
```

### Option B: Keep as Instance/Static Method (Stateful or Side-Effectful)

Use when the code requires **instance state, class state, or side effects**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Uses `this` / instance fields | `this._cache.get(key)`                          |
| Uses class-level static state | `ArchLineChecker._registry[name]`               |
| Has side effects / I/O        | File operations, logging with context           |
| Tightly coupled to class semantics | References class-level types or constants  |

```typescript
// capabilities_line_checker.ts (STAYS IN CLASS — Block 3)
export class ArchLineChecker implements ILineCheckerProtocol {
    private static readonly _THRESHOLD_KEY = 'line_count';

    constructor(private readonly _config: ICheckerConfigProtocol) {}

    private resolveThreshold(layer: string): number {   // uses this → stays
        return this._config.getThreshold(layer);
    }

    static fromRegistry(name: string): ArchLineChecker { // uses class state → stays
        return ArchLineChecker._registry.get(name)!;
    }
}
```

### Decision Tree

```
Function found in capabilities file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in capabilities)
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
  │   ├─ Defined in I<Name>Protocol interface?
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

- [ ] File follows the **3-Block Structure** (Class + `constructor` → Protocol Methods → Utility/Factories/Helpers).
- [ ] **Block 2 contains ONLY protocol interface method implementations**. No utility methods (`toString`, `equals`), no static factories, no `private` helpers in Block 2.
- [ ] **Utility methods** (`toString`, `toJSON`, `equals`, `valueOf`) and **static factories** (`create`, `from`, `of`) are in **Block 3**.
- [ ] Capability class implements a protocol interface (AES403 resolved).
- [ ] Interface contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3 (`private` methods).
- [ ] Constructors receive protocol interfaces via `constructor`.
- [ ] **No module-level `export function`** exists outside the class in capabilities files.
- [ ] **No stateless `static` method** (zero class dependency) remains in class — extracted to `*_utility.ts`.
- [ ] Stateless utilities exist in their own `*_utility.ts` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All interfaces/types imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use protocol interfaces, not concrete types.
- [ ] **Zero I/O** in capabilities layer (no fs, no network, no database).
- [ ] No forbidden imports (no infrastructure\_\_, no agent\_\_).
- [ ] Interface module is registered in the shared package's `index.ts`.
- [ ] Utility module is registered in the shared package's `index.ts`.
- [ ] `npx tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^export class\|^    [a-z]\|^    private \|^    static " packages/<package>/src/capabilities_*.ts

# Find capabilities without interface implementations
grep -rn "^export class \|^class " packages/*/src/capabilities_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "implements.*Protocol" "$file" || echo "MISSING: $file has $class without interface"
done

# Ensure interface does NOT contain helper methods
grep -E "(helper|util|private|_)" packages/shared/src/contract_*_protocol.ts || echo "Clean: No helpers in interface"

# Check for I/O in capabilities (AES404)
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/capabilities_*.ts

# Check for interfaces/types defined in layer files
grep -rn "^interface \|^type \|^enum " packages/*/src/ | grep -v "shared/" | grep capabilities

# Check for concrete type fields (non-interface)
grep -n "constructor" packages/*/src/capabilities_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "constructor" "$file" | grep -v "I[A-Z].*:" || echo "NON-INTERFACE FIELD: $file"
done

# Find module-level free functions in capabilities files (ALWAYS forbidden)
grep -n "^export function \|^function " packages/*/src/capabilities_*.ts

# Find static methods that may need extraction (no class state)
grep -n "static " packages/*/src/capabilities_*.ts | grep -v "private static\|static create\|static from\|static of"

# Find module-level constants that should be in taxonomy
grep -n "^export const " packages/*/src/capabilities_*.ts

# Detect utility methods appearing BEFORE protocol methods (wrong block order)
awk '
    /^    (toString|toJSON|valueOf|equals)\(/ { if (!util_line) util_line = NR }
    /^    [a-z][a-zA-Z]*\(/ && !/^    (toString|toJSON|valueOf|equals|constructor)\(/ { if (!proto_line) proto_line = NR }
    END { if (util_line && proto_line && util_line < proto_line) print "VIOLATION: utility method (line " util_line ") before protocol method (line " proto_line ")" }
' packages/*/src/capabilities_*.ts

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Putting I/O in capabilities**: File I/O, network calls, and database operations MUST be in infrastructure layer.
- ❌ **Defining interfaces/types in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive protocol interfaces, not concrete implementations.
- ❌ **Putting helper methods in the interface**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave protocol methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions MUST be extracted to standalone `*_utility.ts` modules.
- ❌ **Creating "God Protocols"**: If a protocol has >10 methods or mixes unrelated concerns, split it into multiple protocols.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-typescript` if merging multiple files.
- ❌ **Placing utility methods (`toString`, `toJSON`, `equals`) in Block 2**: Block 2 is RESERVED for protocol method implementations ONLY. Utility methods belong in Block 3.
- ❌ **Placing static factories (`create`, `from`, `of`) before protocol methods**: Factories are constructors and belong in Block 3, after protocol methods.
- ❌ **Leaving module-level `export function` in capabilities files**: Free functions outside the class MUST be extracted to `*_utility.ts` in shared/taxonomy. No exceptions.
- ❌ **Keeping stateless `static` methods in class**: If a `static` method uses no `this`, no class-level state, and is not a factory, it belongs in `*_utility.ts`, not in the class body.
- ❌ **Defining `export const` at module level in capabilities files**: Domain constants MUST live in `taxonomy_<name>_constant.ts` in shared/taxonomy.
