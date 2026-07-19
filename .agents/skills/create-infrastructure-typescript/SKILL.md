---
name: create-infrastructure-typescript
description: "Create and validate infrastructure layer files following AES rules: 3-block structure, one class per file, port contracts, zero business logic."
version: 1.1.0
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
    utility-extraction,
  ]
triggers:
  - "create infrastructure typescript"
  - "add infrastructure typescript"
  - "fix infrastructure structure typescript"
  - "create port typescript"
  - "infrastructure missing port typescript"
  - "verify infrastructure typescript"
  - "extract utility typescript"
  - "free function typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - trait-consolidation-typescript
  - module_logic_validator-typescript
  - fix-capability-structure-typescript
  - create-missing-protocols-typescript
---

# create-infrastructure-typescript

## Purpose

Create and validate TypeScript **infrastructure layer** files following clean architecture rules. Ensures infrastructure contains zero business logic, implement port interfaces, follow the 3-Block Structure, use DI for all fields, and extract stateless free functions to taxonomy utility modules.

## Rules

### Layer Boundaries (AES)

**Infrastructure Layer (`infrastructure_*.ts`)**

| Allowed                                      | Forbidden                                        |
| -------------------------------------------- | ------------------------------------------------ |
| File I/O (`fs.`, `readFile`, `writeFile`)    | Business rules                                   |
| Network calls (`fetch`, `axios`, `http`)     | Domain logic                                     |
| Database operations (`sqlite3`, `pg`)        | Calculations (should be in capabilities)         |
| External API calls                           | Direct import from `agent_*`                     |
| Interface implementation                     | Direct import from `capabilities_*`              |

### Structural Rules (All Layers)

- **1 file = 1 class** — each infrastructure file contains exactly ONE main class
- **All data types in shared** — no interfaces/types/enums may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive port interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions MUST be extracted to `*_utility.ts` modules in shared/taxonomy
- **No module-level `export function` in infrastructure files** — free functions outside the class are forbidden; extract to `*_utility.ts`

### Helper vs Utility Decision (The Litmus Test)

> **The Litmus Test:** "If I copy-paste this function to a completely different file, would it still work 100% the same without changing a single line of code?"
> - If **YES** → **Extract to Utility File**.
> - If **NO** (needs `this`, class state, or class context) → **Keep as Private Helper**.

#### When to Extract to Utility (`*_utility.ts`)

Extract if **ALL** conditions are met:

1. **Stateless**: No `this`, no class-level state access
2. **Pure Function**: Input A always produces output B. No side effects (no I/O, no random, no global state mutation)
3. **Domain-Agnostic / Reusable**: Logic is general enough that other classes could use it in the future

#### When to Keep as Private Helper (Block 3)

Keep if **ANY** condition is met:

1. **Needs Instance State**: Accesses `this.field`
2. **Needs Class State**: Accesses `static` fields
3. **Tightly Coupled**: Logic is specific to this class only and doesn't make sense elsewhere (e.g., formatting error messages that reference this class name, mapping internal data to a class-specific output format)
4. **Factory Method**: `static create()`, `static from()`, `static of()` — specific to instantiating this class

#### I/O Blocker (CRITICAL)

A function can be stateless but STILL **cannot** be extracted to taxonomy if it has I/O:

- `fs.readFileSync()`, `fs.promises.readFile()`, `open()`
- `fetch()`, `axios`, `http` (network)
- `sqlite3`, `pg`, `mysql2` (database)

**Rule:** Stateless + I/O = Keep in layer (or move to infrastructure), **NOT** taxonomy utility.

```typescript
function readFileContent(path: string): string {
    // Stateless ✓ (no this)
    // But uses fs.readFileSync() ✗ (I/O)
    // → CANNOT extract to taxonomy utility
    // → Keep in infrastructure layer (this is correct — infra IS for I/O)
}
```

### The 3-Block Structure

Every implementation file MUST follow this exact order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
   - `export class <Type> implements I<Name>Port {` declaration
   - `constructor(...)` with DI fields (port interfaces)
   - Private field declarations (`private readonly _field: IPort`)

2. **Block 2 — Port Methods** (Public Contract)
   - Methods that satisfy the `I<Name>Port` interface signatures.
   - Contains **ONLY** the domain port methods.
   - **NO** utility methods (`toString()`, `toJSON()`, `equals()`) here.
   - **NO** static factory methods (`create()`, `from()`, `of()`) here.
   - **NO** `private` helper methods here.

3. **Block 3 — Utility Methods, Factories & Helpers**
   - Utility/serialization methods: `toString()`, `toJSON()`, `valueOf()`, `equals()`
   - Symbol methods: `[Symbol.iterator]()`, `[Symbol.toPrimitive]()`
   - Static factory methods: `static create()`, `static from()`, `static of()`
   - `private` helper methods that use `this`
   - `private static` helpers that use class-level state

**CRITICAL:** Block 2 is **RESERVED** for domain port methods ONLY. Utility methods (`toString()`, `toJSON()`, `equals()`) and static factory methods belong in **Block 3** because they are utilities/constructors, not the public domain contract.

**CRITICAL:** Stateless free functions (no `this`, no class-level state) MUST be extracted OUT of the class into their own `*_utility.ts` modules in shared/taxonomy. They do NOT belong in Block 3, Block 2, or at module level in infrastructure files.

#### Method Placement Decision Rule

```
Method / function found in an infrastructure file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in infrastructure)
  │
  ├─ Defined in the I<Name>Port interface?
  │   └─ YES → Block 2 (Port Methods)
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
import { FilePath } from '../shared/common/taxonomy_path';
import { IFileReaderPort } from '../shared/common/contract_file_reader_port';


// ─── Block 1: Class Definition & Constructor ──────────────
export class FileCacheAdapter implements IFileReaderPort {
    constructor(
        private readonly _cacheDir: FilePath,
    ) {}


    // ─── Block 2: Port Methods (domain contract ONLY) ─────
    read(path: FilePath): string {
        const fullPath = `${this._cacheDir.value}/${path.value}`;
        return require('fs').readFileSync(fullPath, 'utf-8');
    }


    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return `FileCacheAdapter(cacheDir=${this._cacheDir.value})`;
    }

    equals(other: unknown): boolean {
        return other instanceof FileCacheAdapter && this._cacheDir === other._cacheDir;
    }

    static create(): FileCacheAdapter {
        return new FileCacheAdapter(new FilePath('.cache'));
    }
}
```

#### Example: Extracted Utility Module

```typescript
// shared/common/taxonomy_file_utility.ts
/**
 * Stateless utility functions for file operations.
 */

import * as fs from 'fs';
import * as path from 'path';

export function ensureParentDir(filePath: string): void {
    const dir = path.dirname(filePath);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
}

export function normalizePath(filePath: string): string {
    return path.normalize(filePath);
}
```

### Port Rules

- **Every infrastructure class MUST implement a port interface**
- **Port MUST define methods for all public methods**
- **Port contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (`private` methods)
- **Constructors in Block 1** — `constructor` receives port interfaces
- **Utility methods (`toString`, `toJSON`, `equals`) in Block 3**
- **Static factory methods (`create`, `from`, `of`) in Block 3**
- **Stateless `static` methods (no class dependency) → extract to `*_utility.ts`**

## The Fundamental Question

> **"Is this file pure I/O or external system integration?"**

If yes → **`infrastructure_*.ts` + implement port interface**
If no (has business logic) → **split into capabilities layer instead**

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

### BAD: Infrastructure Without Port (AES404)

```typescript
// BAD: No port implementation
class FileCache {
    read(): string { ... }
}
```

### BAD: Business Logic in Infrastructure

```typescript
// BAD: Business logic in infrastructure layer
class OrphanFileCache {
    analyze(content: string): boolean {
        // ← DOMAIN LOGIC — should be in capabilities
        const isOrphan = content.includes("orphan");
        return isOrphan;
    }
}
```

### BAD: Interface in Layer File

```typescript
// BAD: Domain data defined in infrastructure layer
interface CacheEntry {  // ← INTERFACE — should be in shared/taxonomy
    key: string;
    value: string;
    timestamp: number;
}

class OrphanFileCache {
    entry: CacheEntry;  // ← concrete type, not DI
}
```

### BAD: Utility Methods in Block 2

```typescript
// BAD: toString / equals mixed in with port methods
export class FileCacheAdapter implements IFileReaderPort {
    constructor(private readonly _cacheDir: FilePath) {}

    toString(): string {                    // ← Block 2 position, NOT a port method
        return 'FileCacheAdapter()';
    }

    read(path: FilePath): string { ... }    // ← pushed down

    equals(other: unknown): boolean {       // ← also in Block 2 position
        return other instanceof FileCacheAdapter;
    }
}
```

### BAD: Module-Level Free Function in Infrastructure File

```typescript
// BAD: Free function outside class in infrastructure file
// infrastructure_file_adapter.ts

export function ensureParentDir(filePath: string): void {   // ← FREE FUNCTION — extract to utility
    const dir = path.dirname(filePath);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
}

export function normalizePath(filePath: string): string {   // ← FREE FUNCTION — extract to utility
    return path.normalize(filePath);
}

export class FileCacheAdapter implements IFileReaderPort {
    read(path: FilePath): string {
        const normalized = normalizePath(path.value);
        ...
    }
}
```

### BAD: Stateless Static Method That Should Be Extracted

```typescript
// BAD: static method with zero class dependency — belongs in utility
export class FileCacheAdapter implements IFileReaderPort {

    static normalizePath(filePath: string): string {    // ← no this, no class state, pure logic
        return path.normalize(filePath);
    }

    static ensureParentDir(filePath: string): void {    // ← no this, no class state, pure logic
        const dir = path.dirname(filePath);
        if (!fs.existsSync(dir)) {
            fs.mkdirSync(dir, { recursive: true });
        }
    }

    read(path: FilePath): string {
        FileCacheAdapter.ensureParentDir(path.value);   // ← could be a free function
        ...
    }
}
```

### GOOD: Class with Shared Data

```typescript
// GOOD: All data from shared, fields use DI
import { FilePath } from '../shared/common/taxonomy_path';
import { IFileReaderPort } from '../shared/common/contract_file_reader_port';

export class FileCacheAdapter {
    constructor(
        private readonly _cacheDir: FilePath,
    ) {}
}
```

### GOOD: Correct 3-Block with Utility Methods

```typescript
// GOOD: Port methods in Block 2, utility + factories in Block 3
export class FileCacheAdapter implements IFileReaderPort {

    constructor(private readonly _cacheDir: FilePath) {}  // Block 1: constructor

    read(path: FilePath): string { ... }                  // Block 2: port method ONLY

    toString(): string {                                  // Block 3: utility method
        return `FileCacheAdapter(cacheDir=${this._cacheDir.value})`;
    }

    static create(): FileCacheAdapter {                   // Block 3: factory
        return new FileCacheAdapter(new FilePath('.cache'));
    }

    private resolvePath(filePath: string): string {       // Block 3: private helper
        return `${this._cacheDir.value}/${filePath}`;
    }
}
```

### GOOD: Extracted to Taxonomy Utility

```typescript
// GOOD: shared/common/taxonomy_file_utility.ts

import * as fs from 'fs';
import * as path from 'path';

export function ensureParentDir(filePath: string): void {
    const dir = path.dirname(filePath);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
}

export function normalizePath(filePath: string): string {
    return path.normalize(filePath);
}
```

```typescript
// GOOD: infrastructure_file_adapter.ts (consumer)

import { ensureParentDir, normalizePath } from '../shared/common/taxonomy_file_utility';

export class FileCacheAdapter implements IFileReaderPort {

    read(path: FilePath): string {
        ensureParentDir(path.value);        // ← imported from utility
        const normalized = normalizePath(path.value);  // ← imported from utility
        ...
    }
}
```

## Workflow

### Step 1: Analyze File

Read file and check for mixed responsibilities. Ask: **"Is this code in the right layer?"**

- If it has business logic → **MOVE to Capabilities** (AES404)
- If pure I/O/external integration → continue to Step 2

### Step 2: Check for Missing Port

Does the infrastructure class implement a port interface? If no → create one.

```bash
# Find infrastructure without port implementations
grep -rn "^export class \|^class " packages/*/src/infrastructure_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "implements.*Port" "$file" || echo "MISSING: $file has $class without port"
done
```

### Step 3: Create Port File (if missing)

Create `contract_<name>_port.ts` in the shared package with interface methods.

**Port location:**

| Package    | Port Path                                             |
| ---------- | ----------------------------------------------------- |
| compositor | `packages/shared/src/compositor/contract_*_port.ts`  |
| animator   | `packages/shared/src/animator/contract_*_port.ts`    |
| scripting  | `packages/shared/src/scripting/contract_*_port.ts`   |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order within the class body:

1. `export class <Type> implements I<Name>Port` + `constructor` (class definition with DI fields)
2. Interface method implementations (**domain port methods ONLY**)
3. Utility methods (`toString`, `toJSON`, `equals`), static factories (`create`, `from`), `private` helpers

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All interfaces/types in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use interfaces** — constructor receives port interfaces, not concrete types
- **No standalone functions remain in Block 3** — extract to `*_utility.ts` modules

### Step 6: Extract Free Functions to Utility

Scan the file for functions that have **no `this` / class-state dependency**:

```bash
# Find module-level export functions (outside class) — ALWAYS forbidden
grep -n "^export function \|^function " packages/*/src/infrastructure_*.ts

# Find static methods inside class
grep -n "static " packages/*/src/infrastructure_*.ts

# Find standalone exported constants that may belong in utility
grep -n "^export const " packages/*/src/infrastructure_*.ts
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
4. Add import in infrastructure file: `import { funcName } from '../shared/<domain>/taxonomy_<name>_utility';`
5. Remove original function from infrastructure file
6. Export from `index.ts` barrel if needed
7. Verify: `npx tsc --noEmit`

### Step 7: Verify Layer Compliance

Check forbidden imports and business logic patterns:

```bash
# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate" packages/*/src/infrastructure_*.ts

# Check for forbidden imports
grep -n "capabilities_\|agent_" packages/*/src/infrastructure_*.ts
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
| No `this`, no class state                     | `normalizePath(filePath: string): string`     |
| All data via parameters                       | `ensureParentDir(filePath: string): void`     |
| Deterministic, no side effects                | `isAccessible(filePath: string): boolean`     |

```typescript
// taxonomy_file_utility.ts (SHARED / TAXONOMY)
export function normalizePath(filePath: string): string {
    return path.normalize(filePath);
}

// infrastructure_file_adapter.ts (CONSUMER)
import { normalizePath } from '../shared/common/taxonomy_file_utility';
```

### Option B: Keep as Instance/Static Method (Stateful or Side-Effectful)

Use when the code requires **instance state, class state, or side effects**:

| Condition                     | Example                                         |
| ----------------------------- | ----------------------------------------------- |
| Uses `this` / instance fields | `this._cacheDir.value`                          |
| Uses class-level static state | `FileCacheAdapter._registry[name]`              |
| Has side effects / I/O        | File operations, logging with context           |
| Tightly coupled to class semantics | References class-level types or constants  |

```typescript
// infrastructure_file_adapter.ts (STAYS IN CLASS — Block 3)
export class FileCacheAdapter implements IFileReaderPort {
    private static readonly _DEFAULT_CACHE_DIR = '.cache';

    constructor(private readonly _cacheDir: FilePath) {}

    private resolvePath(filePath: string): string {   // uses this → stays
        return `${this._cacheDir.value}/${filePath}`;
    }

    static fromEnv(): FileCacheAdapter { // uses class state → stays
        return new FileCacheAdapter(new FilePath(process.env.CACHE_DIR ?? FileCacheAdapter._DEFAULT_CACHE_DIR));
    }
}
```

### Decision Tree

```
Function found in infrastructure file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in infrastructure)
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
  │   ├─ Defined in I<Name>Port interface?
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

- [ ] File follows the **3-Block Structure** (Class + `constructor` → Port Methods → Utility/Factories/Helpers).
- [ ] **Block 2 contains ONLY port interface method implementations**. No utility methods (`toString`, `equals`), no static factories, no `private` helpers in Block 2.
- [ ] **Utility methods** (`toString`, `toJSON`, `equals`, `valueOf`) and **static factories** (`create`, `from`, `of`) are in **Block 3**.
- [ ] Infrastructure class implements a port interface.
- [ ] Interface contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3 (`private` methods).
- [ ] Constructors receive port interfaces via `constructor`.
- [ ] **No module-level `export function`** exists outside the class in infrastructure files.
- [ ] **No stateless `static` method** (zero class dependency) remains in class — extracted to `*_utility.ts`.
- [ ] Stateless utilities exist in their own `*_utility.ts` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All interfaces/types imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use port interfaces, not concrete types.
- [ ] **Zero business logic** in infrastructure layer (no domain rules, no calculations).
- [ ] No forbidden imports (no capabilities\_\_, no agent\_\_).
- [ ] Port module is registered in the shared package's `index.ts`.
- [ ] Utility module is registered in the shared package's `index.ts`.
- [ ] `npx tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^export class\|^    [a-z]\|^    private \|^    static " packages/<package>/src/infrastructure_*.ts

# Find infrastructure without port implementations
grep -rn "^export class \|^class " packages/*/src/infrastructure_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "implements.*Port" "$file" || echo "MISSING: $file has $class without port"
done

# Ensure port does NOT contain helper methods
grep -E "(helper|util|private|_)" packages/shared/src/contract_*_port.ts || echo "Clean: No helpers in port"

# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate\|business" packages/*/src/infrastructure_*.ts

# Check for interfaces/types defined in layer files
grep -rn "^interface \|^type \|^enum " packages/*/src/ | grep -v "shared/" | grep infrastructure

# Check for concrete type fields (non-interface)
grep -n "constructor" packages/*/src/infrastructure_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "constructor" "$file" | grep -v "I[A-Z].*:" || echo "NON-INTERFACE FIELD: $file"
done

# Find module-level free functions in infrastructure files (ALWAYS forbidden)
grep -n "^export function \|^function " packages/*/src/infrastructure_*.ts

# Find static methods that may need extraction (no class state)
grep -n "static " packages/*/src/infrastructure_*.ts | grep -v "private static\|static create\|static from\|static of"

# Find module-level constants that should be in taxonomy
grep -n "^export const " packages/*/src/infrastructure_*.ts

# Detect utility methods appearing BEFORE port methods (wrong block order)
awk '
    /^    (toString|toJSON|valueOf|equals)\(/ { if (!util_line) util_line = NR }
    /^    [a-z][a-zA-Z]*\(/ && !/^    (toString|toJSON|valueOf|equals|constructor)\(/ { if (!port_line) port_line = NR }
    END { if (util_line && port_line && util_line < port_line) print "VIOLATION: utility method (line " util_line ") before port method (line " port_line ")" }
' packages/*/src/infrastructure_*.ts

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Putting business logic in infrastructure**: Domain rules, calculations, and validation MUST be in capabilities layer.
- ❌ **Defining interfaces/types in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive port interfaces, not concrete implementations.
- ❌ **Putting helper methods in the port**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave port methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions MUST be extracted to standalone `*_utility.ts` modules.
- ❌ **Creating "God Ports"**: If a port has >10 methods or mixes unrelated concerns, split it into multiple ports.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-typescript` if merging multiple files.
- ❌ **Placing utility methods (`toString`, `toJSON`, `equals`) in Block 2**: Block 2 is RESERVED for port method implementations ONLY. Utility methods belong in Block 3.
- ❌ **Placing static factories (`create`, `from`, `of`) before port methods**: Factories are constructors and belong in Block 3, after port methods.
- ❌ **Leaving module-level `export function` in infrastructure files**: Free functions outside the class MUST be extracted to `*_utility.ts` in shared/taxonomy. No exceptions.
- ❌ **Keeping stateless `static` methods in class**: If a `static` method uses no `this`, no class-level state, and is not a factory, it belongs in `*_utility.ts`, not in the class body.
- ❌ **Defining `export const` at module level in infrastructure files**: Domain constants MUST live in `taxonomy_<name>_constant.ts` in shared/taxonomy.
