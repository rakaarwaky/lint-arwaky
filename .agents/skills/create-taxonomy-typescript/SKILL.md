---
name: create-taxonomy-typescript
description: "Create and validate taxonomy layer files (shared/taxonomy) — all data types, VOs, errors, constants, and utilities must live here following strict naming conventions."
version: 1.1.0
category: refactoring
tags:
  [
    typescript,
    aes,
    taxonomy,
    shared,
    dataclass,
    vo,
    entity,
    utility,
    structure,
  ]
triggers:
  - "create taxonomy typescript"
  - "add taxonomy typescript"
  - "move to taxonomy typescript"
  - "interface in shared typescript"
  - "create value object typescript"
  - "create taxonomy entity typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-infrastructure-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - interface-consolidation-typescript
  - method_classifier-typescript
---

# create-taxonomy-typescript

## Purpose

Create and validate TypeScript **taxonomy layer** files in `packages/shared/src/<domain>/`. This is where ALL data types, value objects, errors, constants, and stateless utility functions MUST live. No domain types may be defined in capabilities, infrastructure, agents, or surface layers.

## Rules

### The Fundamental Question

> **"Is this a data type?"**

- **Data type** (with domain data, DTOs, results, VOs) → **MUST be in shared/taxonomy**. Never in capabilities/infrastructure/agents/surface.
- **Class** (that implements an interface, uses DI) → belongs in the layer file (`capabilities_*.ts`, `infrastructure_*.ts`, `agent_*.ts`).

### Taxonomy Layer Structure

```
packages/shared/src/
├── index.ts                 # Top-level module declarations
├── common/                  # Cross-domain shared types
│   ├── index.ts
│   └── taxonomy_*.ts
├── <domain>/                # Domain-specific taxonomy
│   ├── index.ts             # Module exports for this domain
│   ├── contract_*.ts        # Contract interfaces (port, protocol, aggregate)
│   ├── taxonomy_*_vo.ts     # Value Objects
│   ├── taxonomy_*_entity.ts # Entity types
│   ├── taxonomy_*_error.ts  # Error types
│   └── taxonomy_*_utility.ts# Stateless utility functions
```

### File Naming Convention

Taxonomy files follow strict naming patterns:

| Suffix      | Purpose                              | Allowed? | Example                              |
| ----------- | ------------------------------------ | -------- | ------------------------------------ |
| `_vo`       | Value Objects (wraps a single value) | ✅ YES   | `taxonomy_import_rule_vo.ts`         |
| `_entity`   | Domain entities with identity        | ✅ YES   | `taxonomy_analysis_entity.ts`        |
| `_error`    | Error types (`Error` class)          | ✅ YES   | `taxonomy_config_error.ts`           |
| `_event`    | Event/message types                  | ✅ YES   | `taxonomy_scan_event.ts`             |
| `_constant` | Static compile-time constants        | ✅ YES   | `taxonomy_layer_names_constant.ts`   |
| `_utility`  | Stateless functions                  | ✅ YES   | `taxonomy_symbol_renamer_utility.ts` |

**CRITICAL:** These suffixes are **strict** — only `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility` are allowed for `taxonomy_` prefixed files. No other suffixes.

### Import Restrictions (AES201)

Taxonomy files must remain **completely pure**:

| Taxonomy Type                                          | Can Import From              | Cannot Import From                                              |
| ------------------------------------------------------ | ---------------------------- | --------------------------------------------------------------- |
| **taxonomy(vo)**                                       | Other taxonomy types         | agents, infrastructure, surfaces, contracts, capabilities, root |
| **taxonomy(entity), taxonomy(error), taxonomy(event)** | taxonomy VOs/constants       | agents, infrastructure, surfaces, contracts, capabilities       |
| **taxonomy(constant)**                                 | Nothing (pure static values) | Any external imports                                            |
| **taxonomy(utility)**                                  | taxonomy types               | Non-taxonomy layers                                             |

### Data Type Patterns

#### Value Objects (`_vo.ts`)

Wrap a single value with type safety:

```typescript
// taxonomy_import_rule_vo.ts
export class ImportRuleVO {
    constructor(
        private readonly pattern: string,
        private readonly message: string
    ) {}

    value(): string {
        return this.pattern;
    }
}
```

#### Macro-Generated Value Objects

For simple wrappers, use classes:

```typescript
// taxonomy_common_vo.ts
export class FieldNameVO {
    constructor(private readonly value: string) {}
}

export class BooleanVO {
    constructor(private readonly value: boolean) {}
}

export class SeverityVO {
    constructor(private readonly value: number) {}
}
```

#### Error Types (`_error.ts`)

Use TypeScript Error classes:

```typescript
// taxonomy_config_error.ts
export class ConfigError extends Error {
    constructor(
        public readonly key: string,
        public readonly message: string
    ) {
        super(`Config error: ${key} - ${message}`);
        this.name = 'ConfigError';
    }
}
```

#### Utility Functions (`_utility.ts`)

Stateless functions (no side effects) that act as **Dumb Tools**.

**🚨 CRITICAL: The Ultimate Boundary for Utilities**
A function belongs in `*_utility.ts` ONLY if it meets ALL of these:

1. **Stateless**: No `this` context, no class field access.
2. **Pure Function**: Input A always produces output B. No side effects (no I/O).
3. **Domain-Agnostic / Reusable**: It does NOT know about specific business rules or domain-specific validation logic. It is a blind data manipulator (e.g., regex matching, string normalization, AST parsing).
4. **Multi-Consumer Reusable**: Function serves multiple capabilities/infrastructures (could be same domain or cross-domain), not just one class.

If a stateless function contains **Domain Knowledge** OR only serves **ONE capability/infrastructure class**, it MUST stay in the capabilities layer as a **Private Helper**, NOT extracted to taxonomy utility.

```typescript
// ✅ GOOD: Dumb Tool (Domain-Agnostic, Multi-Consumer Reusable)
export function extractTraitName(content: string): string | null {
    // Just regex, doesn't know what a "trait" means in domain context
    // Multiple capabilities/infrastructures can use this
    // ...
}

// ❌ BAD: Domain Knowledge masquerading as utility
export function getTargetLayerFromSuffix(suffix: string): string {
    // KNOWS business rules: port = infrastructure.
    // This belongs in capabilities as a private helper!
    switch (suffix) {
        case 'port': return 'infrastructure';
        case 'protocol': return 'capabilities';
        default: return 'unknown';
    }
}

// ❌ BAD: Single Consumer Only
export function formatImportViolation(rule: ImportRule): string {
    // Only used by one capability class, not reusable by others
    // This belongs in capabilities as a private helper!
    return `Import rule violation: ${rule.pattern}`;
}
```

## Detection Patterns

### BAD: Interface Defined in Layer File

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

### BAD: Interface Defined in Infrastructure

```typescript
// BAD: Domain data defined in infrastructure layer
interface CacheEntry {  // ← INTERFACE — should be in shared/taxonomy
    key: string;
    value: string;
    timestamp: number;
}
```

### GOOD: Interface in Taxonomy + Class with DI

```typescript
// GOOD: Interface in taxonomy
// packages/shared/src/orphan-detector/taxonomy_analysis_vo.ts
export interface OrphanIndicatorResult {
    isOrphan: boolean;
    reason: string;
    severity: string;
}

// GOOD: Class imports from taxonomy
// packages/orphan-detector/src/capabilities_orphan_analyzer.ts
import { OrphanIndicatorResult } from '../shared/orphan_detector/taxonomy_analysis';
import { IOrphanFilenameExtractorProtocol } from '../contract/orphan_protocol';

class CapabilitiesOrphanAnalyzer {
    constructor(private extractor: IOrphanFilenameExtractorProtocol) {}  // ← DI
}
```

## Workflow

### Step 1: Identify the Data Type

When you find an interface in a layer file (capabilities/infrastructure/agent/surface), ask: **"Is this a data type or a class?"**

- If it contains domain data, DTOs, results, or value wrappers → **data type → move to taxonomy**
- If it implements an interface and uses DI → **class → stays in layer file**

### Step 2: Determine Taxonomy Domain

Find the correct domain directory under `packages/shared/src/<domain>/`:

| Domain            | Directory                     | Example Types                              |
| ----------------- | ----------------------------- | ------------------------------------------ |
| `common`          | `shared/src/common/`          | Cross-domain types (PathVO, BooleanVO)     |
| `orphan-detector` | `shared/src/orphan-detector/` | Orphan results, severity, violations       |
| `code-analysis`   | `shared/src/code-analysis/`   | Analysis results, reachability, violations |
| `import-rules`    | `shared/src/import-rules/`    | Import rules, violations, language types   |
| `naming-rules`    | `shared/src/naming-rules/`    | Naming violations, patterns                |

### Step 3: Create or Update Taxonomy File

**Option A: New taxonomy domain** — Create `<domain>/` directory with `index.ts`, then add taxonomy files.

**Option B: Existing domain** — Add new file to existing domain directory.

**Naming:** Use the correct suffix (`_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`).

```bash
# Example: Create orphan result interface in taxonomy
mkdir -p packages/shared/src/orphan-detector/
# Create taxonomy_orphan_vo.ts
```

### Step 4: Register Module

Update the domain's `index.ts` to export the new taxonomy module:

```typescript
// shared/src/orphan-detector/index.ts
export { OrphanResult } from './taxonomy_orphan_vo';  // ← Add this line
export { OrphanIndicatorResult } from './taxonomy_analysis';
```

### Step 5: Update Imports in Layer Files

Replace local interface definitions with imports from taxonomy:

```typescript
// BEFORE (BAD): Local interface
interface OrphanResult {
    isOrphan: boolean;
    reason: string;
}

// AFTER (GOOD): Import from taxonomy
import { OrphanResult } from '../shared/orphan_detector/taxonomy_orphan_vo';
```

### Step 6: Verify

Run TypeScript compiler to confirm no violations.

## Verification Checklist

- [ ] **All interfaces in shared/taxonomy** — no interfaces/types defined in layer files.
- [ ] **Taxonomy file naming follows strict suffixes** — `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility`.
- [ ] **Taxonomy files import only from taxonomy** — no imports from capabilities, infrastructure, agents, contracts, or surface.
- [ ] **Utility functions in `*_utility.ts` are purely domain-agnostic AND serve MULTIPLE capabilities/infrastructures** — functions containing business rules OR serving only ONE class stay in capabilities as private helpers.
- [ ] **Layer files import data types from taxonomy** — not defined locally.
- [ ] **Domain's `index.ts` exports new taxonomy modules** — `export { ... } from './taxonomy_<name>'`.
- [ ] **Value Objects are immutable** — readonly properties by default.
- [ ] **Error types extend `Error`** — with proper error messages.
- [ ] **Constants are pure static values** — no imports, no functions.
- [ ] **Contract signatures use VOs, not primitives** — ALL primitives are FORBIDDEN in contract method signatures:
  - `string` → use domain-specific VO (e.g., `FilePath`, `SymbolName`)
  - `number` → use domain-specific VO (e.g., `LineNumber`, `Count`)
  - `boolean` → use `BooleanVO`
  - `string[]` → use domain-specific list VO (e.g., `PatternList`)
  - `Record<string, T>` → use domain-specific VO
- [ ] `tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Find interfaces defined in layer files (not in shared/taxonomy)
grep -rn "^interface\|^type " packages/*/src/ | grep -v "shared/" | grep -v "index.ts"

# Check for forbidden imports in taxonomy files
grep -n "from.*capabilities_\|from.*infrastructure_\|from.*agent_" packages/shared/src/*/taxonomy_*.ts

# Find layer files with concrete type fields (non-interface) that might need taxonomy interfaces
grep -n "constructor" packages/*/src/ | grep -v "I[A-Z].*:" | grep -v "shared/"

# Verify taxonomy module exports are registered
grep -n "^export.*from.*taxonomy_" packages/shared/src/*/index.ts

# Check for unregistered taxonomy files (exist on disk but not in index.ts)
find packages/shared/src/<domain>/ -name "taxonomy_*.ts" | while read f; do
    name=$(basename "$f" .ts)
    grep -q "from.*$name" packages/shared/src/<domain>/index.ts || echo "UNREGISTERED: $name"
done

# Check for interfaces in layer files that should be moved to taxonomy
grep -rn "^interface\|^type " packages/*/src/ | grep -v "shared/" | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    interface=$(echo "$line" | grep -oP '(?<=interface |type )\K[a-zA-Z_]+')
    echo "POSSIBLE INTERFACE: $file has $interface"
done
```

## Naming Convention (from fix-naming)

**All Layer File Naming:**

| Layer                    | Pattern                    | Suffix                                     |
| ------------------------ | -------------------------- | ------------------------------------------ |
| **root**           | `root_*_container.ts`    | `_container`                             |
| **taxonomy**       | `taxonomy_*_vo.ts`       | `_vo`, `_constant`, `_utility`, etc. |
| **contract**       | `contract_*_protocol.ts` | `_protocol`, `_port`, `_aggregate`   |
| **capabilities**   | `capabilities_*.ts`      | flexible                                   |
| **infrastructure** | `infrastructure_*.ts`    | flexible                                   |
| **agent**          | `agent_*.ts`             | `_orchestrator`                          |
| **surface**        | `surface_*.ts`           | `_command`, `_controller`              |

## Primitive-to-VO Patterns (from fix-primitive-to-vo)

**Taxonomy Layer VO Creation Rules:**

- Entity fields MUST use VOs, not primitives (`string`, `number`, `boolean`).
- **Contract signatures MUST use VOs** — ALL primitives are FORBIDDEN in contract method signatures. The VOs created here are the mandatory replacements:
  - `string` → use domain-specific VO (e.g., `FilePath`, `SymbolName`)
  - `number` → use domain-specific VO (e.g., `LineNumber`, `Count`, `Score`)
  - `boolean` → use `BooleanVO`
  - `string[]` → use domain-specific list VO (e.g., `PatternList`)
  - `Record<string, T>` → use domain-specific VO
- VOs MUST validate on construction.

```typescript
// BEFORE (primitive in layer file)
interface LintResult {
    filePath: string;   // ← primitive
    line: number;       // ← primitive
    severity: string;   // ← primitive
}

// AFTER (VO in taxonomy)
// packages/shared/src/import-rules/taxonomy_file_path_vo.ts
export class FilePath {
    constructor(private readonly value: string) {}
    value(): string { return this.value; }
}

// packages/shared/src/import-rules/taxonomy_line_number_vo.ts
export class LineNumber {
    constructor(private readonly value: number) {}
    value(): number { return this.value; }
}

// packages/shared/src/import-rules/taxonomy_severity_vo.ts
export class SeverityVO {
    constructor(private readonly value: string) {}
    value(): string { return this.value; }
}

interface LintResult {
    filePath: FilePath;   // ← VO
    line: LineNumber;     // ← VO
    severity: SeverityVO; // ← VO
}
```

## Magic Constant Definitions (from fix-magic-constant)

**Taxonomy Layer Constant Rules:**

- All domain values live in `taxonomy_*_constant.ts` files.
- Constants are static compile-time values — no functions, no imports.
- Used by agent, capabilities, and infrastructure layers.

```typescript
// packages/shared/src/animator/taxonomy_animator_constant.ts
/** Default frames per second for animation */
export const FPS_DEFAULT = 24.0;

/** Minimum reveal time in seconds */
export const MIN_REVEAL_SECONDS = 0.5;

/** Manifest filename constant */
export const MANIFEST_FILENAME = 'manifest.json';
```

**Layer consumption:**

```typescript
// Agent layer
import { FPS_DEFAULT } from '../shared/animator/taxonomy_animator_constant';
const result = this.process(FPS_DEFAULT);

// Capabilities layer
import { MIN_REVEAL_SECONDS } from '../shared/animator/taxonomy_animator_constant';
function calculateDuration(): number { return MIN_REVEAL_SECONDS; }

// Infrastructure layer
import { MANIFEST_FILENAME } from '../shared/animator/taxonomy_animator_constant';
const file = fs.createWriteStream(MANIFEST_FILENAME);
```

## Common Mistakes (AVOID)

- ❌ **Defining interfaces in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Importing non-taxonomy types into taxonomy files**: Taxonomy must remain completely pure — no imports from capabilities, infrastructure, agents, contracts, or surface.
- ❌ **Using wrong suffix for taxonomy files**: Only `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility` are allowed. No other suffixes.
- ❌ **Forgetting to register new taxonomy modules in index.ts**: Every `taxonomy_*.ts` file must have a corresponding `export { ... } from './taxonomy_<name>'` in the domain's `index.ts`.
- ❌ **Placing Domain Knowledge in Utility files**: If a stateless function contains business-specific rules or domain logic (e.g., layer mappings, validation rules tied to a specific domain), it belongs in capabilities as a private helper, NOT in `*_utility.ts`.
- ❌ **Placing Single-Consumer functions in Utility files**: If a function only serves ONE capability/infrastructure class, it belongs in capabilities as a private helper, NOT in `*_utility.ts`.
- ❌ **Placing utility functions in layer files**: Stateless, domain-agnostic free functions (no `this` context) that serve MULTIPLE capabilities/infrastructures MUST be extracted to `*_utility.ts` modules in shared/taxonomy.
- ❌ **Creating multiple data types with different names for the same concept**: Consolidate into a single taxonomy file.
- ❌ **Duplicating taxonomy types across domains**: If a type belongs to multiple domains, put it in `common/` and import from there.
