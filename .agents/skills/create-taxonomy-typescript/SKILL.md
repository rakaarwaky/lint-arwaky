---
name: create-taxonomy-typescript
description: "Create and validate taxonomy layer files (shared/taxonomy) — all data types, VOs, errors, and utilities must live here following strict naming conventions."
version: 1.0.0
category: refactoring
tags: [typescript, aes, taxonomy, shared, interface, vo, entity, utility, structure]
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

Stateless functions (no side effects):

```typescript
// taxonomy_symbol_renamer_utility.ts

/** Stateless formatting utility — no side effects needed */
export function formatBytes(bytes: number): string {
    return `${(bytes / 1024).toFixed(1)}KB`;
}

/** Stateless math utility — no side effects needed */
export function clamp(value: number, minVal: number, maxVal: number): number {
    return Math.max(minVal, Math.min(value, maxVal));
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
- [ ] **Utility functions in `*_utility.ts`** — standalone functions extracted to modules.
- [ ] **Layer files import data types from taxonomy** — not defined locally.
- [ ] **Domain's `index.ts` exports new taxonomy modules** — `export { ... } from './taxonomy_<name>'`.
- [ ] **Value Objects are immutable** — readonly properties by default.
- [ ] **Error types extend `Error`** — with proper error messages.
- [ ] **Constants are pure static values** — no imports, no functions.
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

## Common Mistakes (AVOID)

- ❌ **Defining interfaces in layer files**: Domain data must be in shared/taxonomy. Only the class belongs in layer files.
- ❌ **Importing non-taxonomy types into taxonomy files**: Taxonomy must remain completely pure — no imports from capabilities, infrastructure, agents, contracts, or surface.
- ❌ **Using wrong suffix for taxonomy files**: Only `_vo`, `_entity`, `_error`, `_event`, `_constant`, `_utility` are allowed. No other suffixes.
- ❌ **Forgetting to register new taxonomy modules in index.ts**: Every `taxonomy_*.ts` file must have a corresponding `export { ... } from './taxonomy_<name>'` in the domain's `index.ts`.
- ❌ **Placing utility functions in layer files**: Standalone functions MUST be extracted to `*_utility.ts` modules in shared/taxonomy.
- ❌ **Creating multiple data types with different names for the same concept**: Consolidate into a single taxonomy file.
- ❌ **Duplicating taxonomy types across domains**: If a type belongs to multiple domains, put it in `common/` and import from there.
