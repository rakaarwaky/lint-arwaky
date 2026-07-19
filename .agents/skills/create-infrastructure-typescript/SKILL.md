---
name: create-infrastructure-typescript
description: "Create and validate infrastructure layer files following AES rules: 3-block structure, one class per file, port interfaces, zero business logic."
version: 1.0.0
category: refactoring
tags:
  [typescript, aes, infrastructure, port, structure, aes404, 3-block-structure, di]
triggers:
  - "create infrastructure typescript"
  - "add infrastructure typescript"
  - "fix infrastructure structure typescript"
  - "create port typescript"
  - "infrastructure missing port typescript"
  - "verify infrastructure typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - interface-consolidation-typescript
  - module_logic_validator-typescript
  - fix-capability-structure-typescript
  - create-missing-protocols-typescript
---

# create-infrastructure-typescript

## Purpose

Create and validate TypeScript **infrastructure layer** files following clean architecture rules. Ensures infrastructure contains zero business logic, implement port interfaces, follow the 3-Block Structure, and use DI for all fields.

## Rules

### Layer Boundaries (AES)

**Infrastructure Layer (`infrastructure_*.ts`)**

| Allowed                                      | Forbidden                                |
| -------------------------------------------- | ---------------------------------------- |
| File I/O (`fs.`, `readFile`, `writeFile`)    | Business rules                           |
| Network calls (`fetch`, `axios`, `http`)     | Domain logic                             |
| Database operations (`sqlite3`, `pg`)        | Calculations (should be in capabilities) |
| External API calls                           | Direct import from `agent_*`             |
| Interface implementation                     | Direct import from `capabilities_*`      |

### Structural Rules (All Layers)

- **1 file = 1 class** — each infrastructure file contains exactly ONE main class
- **All data types in shared** — no interfaces/types may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive port interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions should be extracted to `*_utility.ts` modules in shared/taxonomy

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. `class Definition` (class definition with DI fields)
2. `class Methods` implementing Port (Public Contract)
3. `private` and Helper methods

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic functions MUST be extracted OUT of the class into their own `*_utility.ts` modules in shared/taxonomy. They do NOT belong in Block 3.

### Port Rules

- **Every infrastructure class MUST implement a port interface**
- **Port MUST define methods for all public methods**
- **Port contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (private methods)
- **Constructors in class body** — constructor receives port interfaces

## The Fundamental Question

> **"Is this file pure I/O or external system integration?"**

If yes → **`infrastructure_*.ts` + implement port interface**
If no (has business logic) → **split into capabilities layer instead**

## Naming Convention

| Layer              | File Pattern          | Interface File                | Interface Name    |
| ------------------ | --------------------- | ----------------------------- | ----------------- |
| **Capabilities**   | `capabilities_*.ts`   | `contract_<name>_protocol.ts` | `I<Name>Protocol` |
| **Infrastructure** | `infrastructure_*.ts` | `contract_<name>_port.ts`     | `I<Name>Port`     |
| **Agents**         | `agent_*.ts`          | `contract_<name>_aggregate.ts`| `I<Name>Aggregate`|

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

### GOOD: Class with Shared Data

```typescript
// GOOD: All data from shared, fields use DI
import { FilePath } from '../shared/common/taxonomy_path';

class OrphanFileCache {
    constructor(private extractor: IOrphanFilenameExtractorProtocol) {}  // ← DI via interface
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
grep -rn "^class " packages/*/src/infrastructure_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Port" "$file" || echo "MISSING: $file has $class without port"
done
```

### Step 3: Create Port File (if missing)

Create `contract_<name>_port.ts` in the shared package with interface methods.

**Port location:**

| Package     | Port Path                                          |
| ----------- | -------------------------------------------------- |
| compositor  | `packages/shared/src/compositor/contract_*_port.ts`|
| animator    | `packages/shared/src/animator/contract_*_port.ts`  |
| scripting   | `packages/shared/src/scripting/contract_*_port.ts` |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `class <Type>` (class definition with DI fields)
2. Interface methods implementing Port (public contract)
3. `private` and helper methods

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All interfaces in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use interfaces** — constructor receives port interfaces, not concrete types
- **No standalone functions remain in Block 3** — extract to `*_utility.ts` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and business logic patterns:

```bash
# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate" packages/*/src/infrastructure_*.ts

# Check for forbidden imports
grep -n "capabilities_\|agent_" packages/*/src/infrastructure_*.ts
```

### Step 7: Verify

Run TypeScript compiler to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class -> Port Methods -> Helpers).
- [ ] Infrastructure class implements a port interface.
- [ ] Port contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3 (private methods).
- [ ] Constructors receive port interfaces via constructor.
- [ ] No standalone functions remain in class — extracted to `*_utility.ts` modules.
- [ ] Stateless utilities exist in their own `*_utility.ts` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All interfaces imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use port interfaces, not concrete types.
- [ ] **Zero business logic** in infrastructure layer (no domain rules, no calculations).
- [ ] No forbidden imports (no capabilities__, no agent__).
- [ ] Port module is registered in the shared package's `index.ts`.
- [ ] `tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^    \|^private " packages/<package>/src/infrastructure_*.ts

# Find infrastructure without port implementations
grep -rn "^class " packages/*/src/infrastructure_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Port" "$file" || echo "MISSING: $file has $class without port"
done

# Ensure port does NOT contain helper methods
grep -E "(helper|util|private|_)" packages/shared/src/contract_*_port.ts || echo "Clean: No helpers in port"

# Check for business logic in infrastructure
grep -n "is_orphan\|analyze\|validate\|business" packages/*/src/infrastructure_*.ts

# Check for interfaces defined in layer files
grep -rn "^interface\|^type " packages/*/src/ | grep -v "shared/" | grep infrastructure

# Check for concrete type fields (non-interface)
grep -n "constructor" packages/*/src/infrastructure_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "constructor" "$file" | grep -v "I[A-Z].*:" || echo "NON-INTERFACE FIELD: $file"
done

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Putting business logic in infrastructure**: Domain rules, calculations, and validation MUST be in capabilities layer.
- ❌ **Defining interfaces in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive port interfaces, not concrete implementations.
- ❌ **Putting helper methods in the port**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave port methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions MUST be extracted to standalone `*_utility.ts` modules.
- ❌ **Creating "God Ports"**: If a port has >10 methods or mixes unrelated concerns, split it into multiple ports.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-typescript` if merging multiple files.
