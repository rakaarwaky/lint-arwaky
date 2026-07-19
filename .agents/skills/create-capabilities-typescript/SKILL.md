---
name: create-capabilities-typescript
description: "Create and validate capabilities layer files following AES rules: 3-block structure, one class per file, protocol interfaces, zero I/O."
version: 1.0.0
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
  ]
triggers:
  - "create capability typescript"
  - "add capability typescript"
  - "fix capability structure typescript"
  - "create protocol typescript"
  - "capability missing protocol typescript"
  - "check capabilities typescript"
dependencies: []
related:
  - create-infrastructure-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - interface-consolidation-typescript
  - module_logic_validator-typescript
  - fix-capability-structure-typescript
  - create-missing-protocols-typescript
---

# create-capabilities-typescript

## Purpose

Create and validate TypeScript **capabilities layer** files following clean architecture rules. Ensures capabilities contain zero I/O, implement protocol interfaces, follow the 3-Block Structure, and use DI for all fields.

## Rules

### Layer Boundaries (AES)

**Capabilities Layer (`capabilities_*.ts`)**

| Allowed                               | Forbidden                                    |
| ------------------------------------- | -------------------------------------------- |
| Computation, validation, calculation  | File I/O (`fs.`, `readFile`, `writeFile`)    |
| Data transformation, business rules   | Network calls (`fetch`, `axios`, `http`)     |
| Domain logic, domain model definition | Database operations (`sqlite3`, `pg`)        |
| Interface implementation              | Direct import from `infrastructure_*`        |
|                                       | Direct import from `agent_*`                 |
|                                       | Direct import from `capabilities_*` (self)   |

### Structural Rules (All Layers)

- **1 file = 1 class** — each capabilities file contains exactly ONE main class
- **All data types in shared** — no interfaces/types may be defined outside shared/taxonomy
- **Fields must use DI** — class fields should receive protocol interfaces via constructor
- **Helper methods stay in layer** — helper methods that support the class remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic functions should be extracted to `*_utility.ts` modules in shared/taxonomy

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. `class Definition` (class definition with DI fields)
2. `class Methods` implementing Protocol (Public Contract)
3. `private` and Helper methods

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic functions MUST be extracted OUT of the class into their own `*_utility.ts` modules in shared/taxonomy. They do NOT belong in Block 3.

### Protocol Rules

- **Every capability class MUST implement a protocol interface** (AES403)
- **Protocol MUST define methods for all public methods**
- **Protocol contains ONLY public/contract methods** — no helper methods
- **Helper methods stay in Block 3** (private methods)
- **Constructors in class body** — constructor receives protocol interfaces

## The Fundamental Question

> **"Is this file pure business logic?"**

If yes → **`capabilities_*.ts` + implement protocol interface**
If no (has I/O) → **split into infrastructure layer instead**

## Naming Convention

| Layer              | File Pattern          | Interface File                | Interface Name    |
| ------------------ | --------------------- | ----------------------------- | ----------------- |
| **Capabilities**   | `capabilities_*.ts`   | `contract_<name>_protocol.ts` | `I<Name>Protocol` |
| **Infrastructure** | `infrastructure_*.ts` | `contract_<name>_port.ts`     | `I<Name>Port`     |
| **Agents**         | `agent_*.ts`          | `contract_<name>_aggregate.ts`| `I<Name>Aggregate`|

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

### GOOD: Class with Shared Data

```typescript
// GOOD: All data from shared, fields use DI
import { OrphanIndicatorResult } from '../shared/code_analysis/taxonomy_analysis';
import { IOrphanFilenameExtractorProtocol } from '../contract/orphan_protocol';

class CapabilitiesOrphanAnalyzer {
    constructor(private extractor: IOrphanFilenameExtractorProtocol) {}  // ← DI via interface
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
grep -rn "^class " packages/*/src/capabilities_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Protocol" "$file" || echo "MISSING: $file has $class without interface"
done
```

### Step 3: Create Interface File (if missing)

Create `contract_<name>_protocol.ts` in the shared package with interface methods.

**Interface location:**

| Package     | Interface Path                                          |
| ----------- | ------------------------------------------------------- |
| compositor  | `packages/shared/src/compositor/contract_*_protocol.ts` |
| animator    | `packages/shared/src/animator/contract_*_protocol.ts`   |
| scripting   | `packages/shared/src/scripting/contract_*_protocol.ts`  |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `class <Type>` (class definition with DI fields)
2. Interface methods implementing Protocol (public contract)
3. `private` and helper methods

### Step 5: Verify Class Discipline

- **1 file = 1 class** — no multiple classes in one file
- **All interfaces in shared/taxonomy** — domain types must be imported, not defined locally
- **Fields use interfaces** — constructor receives protocol interfaces, not concrete types
- **No standalone functions remain in Block 3** — extract to `*_utility.ts` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and I/O patterns:

```bash
# Check for I/O in capabilities
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/capabilities_*.ts

# Check for forbidden imports
grep -n "infrastructure_\|agent_" packages/*/src/capabilities_*.ts
```

### Step 7: Verify

Run TypeScript compiler to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Class -> Protocol Methods -> Helpers).
- [ ] Capability class implements a protocol interface (AES403 resolved).
- [ ] Interface contains **only** public/contract methods (no helper methods).
- [ ] Helper methods are in Block 3 (private methods).
- [ ] Constructors receive protocol interfaces via constructor.
- [ ] No standalone functions remain in class — extracted to `*_utility.ts` modules.
- [ ] Stateless utilities exist in their own `*_utility.ts` files in shared/taxonomy.
- [ ] **1 file = 1 class** — no multiple classes in one file.
- [ ] All interfaces imported from shared/taxonomy (none defined locally).
- [ ] Constructor fields use protocol interfaces, not concrete types.
- [ ] **Zero I/O** in capabilities layer (no fs, no network, no database).
- [ ] No forbidden imports (no infrastructure__, no agent__).
- [ ] Interface module is registered in the shared package's `index.ts`.
- [ ] `tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^class\|^    \|^private " packages/<package>/src/capabilities_*.ts

# Find capabilities without interface implementations
grep -rn "^class " packages/*/src/capabilities_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    class=$(echo "$line" | grep -oP 'class \K[a-zA-Z_]+')
    grep -q "Protocol" "$file" || echo "MISSING: $file has $class without interface"
done

# Ensure interface does NOT contain helper methods
grep -E "(helper|util|private|_)" packages/shared/src/contract_*_protocol.ts || echo "Clean: No helpers in interface"

# Check for I/O in capabilities (AES404)
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios" packages/*/src/capabilities_*.ts

# Check for interfaces defined in layer files
grep -rn "^interface\|^type " packages/*/src/ | grep -v "shared/" | grep capabilities

# Check for concrete type fields (non-interface)
grep -n "constructor" packages/*/src/capabilities_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    grep -A5 "constructor" "$file" | grep -v "I[A-Z].*:" || echo "NON-INTERFACE FIELD: $file"
done

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Putting I/O in capabilities**: File I/O, network calls, and database operations MUST be in infrastructure layer.
- ❌ **Defining interfaces in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Using concrete types as constructor fields**: Constructor should receive protocol interfaces, not concrete implementations.
- ❌ **Putting helper methods in the interface**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave protocol methods and helper methods. Keep them in separate sections.
- ❌ **Placing utilities in class body**: Stateless functions MUST be extracted to standalone `*_utility.ts` modules.
- ❌ **Creating "God Protocols"**: If a protocol has >10 methods or mixes unrelated concerns, split it into multiple protocols.
- ❌ **Multiple classes in one file**: Each file should have exactly ONE class. Use `consolidate-files-typescript` if merging multiple files.
