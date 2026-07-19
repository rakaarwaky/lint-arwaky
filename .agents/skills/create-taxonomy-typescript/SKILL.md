---
name: create-taxonomy-typescript
description: "Create and validate TypeScript taxonomy layer files in shared taxonomy: VOs, entities, errors, events, constants, and pure reusable utilities. Ensures domain data lives only in shared taxonomy and remains pure."
version: 1.3.0
category: refactoring
tags:
  [
    typescript,
    aes,
    taxonomy,
    shared,
    vo,
    entity,
    error,
    event,
    constant,
    utility,
    aes201,
    primitive-to-vo,
  ]
triggers:
  - "create taxonomy typescript"
  - "add taxonomy typescript"
  - "move dataclass to taxonomy typescript"
  - "create vo typescript"
  - "create error taxonomy typescript"
  - "create constant taxonomy typescript"
  - "check taxonomy typescript"
  - "audit taxonomy typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-infrastructure-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - interface-consolidation-typescript
  - fix-primitive-to-vo
  - fix-magic-constant
---

# create-taxonomy-typescript

## Purpose

Create and validate TypeScript **taxonomy layer** files inside `packages/shared/src/<domain>/`.

Taxonomy is the single source of truth for:

- value objects,
- entities,
- domain errors,
- domain events,
- constants,
- pure reusable utility functions.

No domain data structures may be defined in:

- capabilities,
- infrastructure,
- agents,
- surface,
- root/container layers.

Those layers must import domain data from shared taxonomy.

---

## Definition of Done

A taxonomy change is considered valid when:

1. Domain data structures live in `shared/taxonomy`.
2. Taxonomy file naming uses the allowed strict suffixes.
3. Taxonomy files do not import from capability, infrastructure, agent, surface, or root layers.
4. Taxonomy files contain no I/O and no side effects.
5. Utility functions in `*_utility.ts` are stateless, pure, domain-agnostic, and reusable.
6. Value objects validate on construction.
7. Public domain contracts use VOs instead of raw primitives.
8. New taxonomy modules are registered in the relevant `index.ts`.
9. `npx tsc --noEmit` passes.

---

## The Fundamental Question

> **"Is this a data type or an implementor?"**

### Data Type

A data type is a type that carries domain data.

Examples:

- value objects,
- DTOs,
- result objects,
- domain entities,
- domain errors,
- domain events,
- enums representing domain values.

These MUST live in shared taxonomy.

```typescript
export interface OrphanAnalysisResult {
    readonly isOrphan: boolean;
    readonly reason: string;
}
```

### Implementor

An implementor is a class that implements an interface and contains behavior, often with injected dependencies.

Examples:

- `capabilities_*.ts`
- `infrastructure_*.ts`
- `agent_*.ts`

These stay in their layer files.

```typescript
export class CapabilitiesOrphanAnalyzer {
    constructor(private readonly extractor: IOrphanFilenameExtractorProtocol) {}
}
```

---

## Taxonomy Layer Structure

Use snake_case module directories.

```text
packages/shared/src/
├── index.ts
├── common/
│   ├── index.ts
│   ├── taxonomy_*_vo.ts
│   ├── taxonomy_*_error.ts
│   ├── taxonomy_*_constant.ts
│   └── taxonomy_*_utility.ts
│
├── <domain>/
│   ├── index.ts
│   ├── contract_*_protocol.ts
│   ├── contract_*_port.ts
│   ├── contract_*_aggregate.ts
│   ├── taxonomy_*_vo.ts
│   ├── taxonomy_*_entity.ts
│   ├── taxonomy_*_error.ts
│   ├── taxonomy_*_event.ts
│   ├── taxonomy_*_constant.ts
│   └── taxonomy_*_utility.ts
```

Important:

- `contract_*.ts` files are NOT taxonomy files.
- Contract interfaces may import taxonomy types.
- Taxonomy files MUST NOT import contract interfaces.

---

## File Naming Convention

Taxonomy files MUST use strict suffixes.

| Suffix        | Purpose                            | Example                                |
| ------------- | ---------------------------------- | -------------------------------------- |
| `_vo`       | Value objects and value-like enums | `taxonomy_file_path_vo.ts`           |
| `_entity`   | Entities with identity             | `taxonomy_analysis_entity.ts`        |
| `_error`    | Error types                        | `taxonomy_config_error.ts`           |
| `_event`    | Event/message types                | `taxonomy_scan_event.ts`             |
| `_constant` | Static compile-time constants      | `taxonomy_layer_names_constant.ts`   |
| `_utility`  | Stateless pure reusable functions  | `taxonomy_symbol_renamer_utility.ts` |

Allowed taxonomy prefixes:

```text
taxonomy_*_vo.ts
taxonomy_*_entity.ts
taxonomy_*_error.ts
taxonomy_*_event.ts
taxonomy_*_constant.ts
taxonomy_*_utility.ts
```

No other taxonomy suffixes are allowed.

---

## Purity and Import Restrictions (AES201)

Taxonomy must remain pure and stable.

### Allowed Dependencies

| Taxonomy Type | May Import From                              | Must Not Import From                                                |
| ------------- | -------------------------------------------- | ------------------------------------------------------------------- |
| `_vo`       | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_entity`   | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_error`    | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_event`    | other taxonomy types, stdlib                | capabilities, infrastructure, agents, surface, root, contracts, I/O |
| `_constant` | only core/static values                      | external layer imports, I/O, functions                              |
| `_utility`  | taxonomy types, pure stdlib helpers         | capabilities, infrastructure, agents, surface, root, contracts, I/O |

Taxonomy may contain:

- value validation,
- domain invariants inside constructors,
- pure transformations between taxonomy types.

Taxonomy must not contain:

- file I/O,
- network calls,
- database access,
- environment mutation,
- side effects,
- business orchestration,
- use-case logic,
- layer-specific behavior.

---

## Data Type Patterns

### Value Objects (`_vo.ts`)

A value object should wrap domain values with type safety and validation.

Prefer readonly properties.

Bad:

```typescript
export class FilePath {
    constructor(public value: string) {}
}
```

Good:

```typescript
export class FilePath {
    private readonly _value: string;

    constructor(value: string) {
        if (!value.trim()) {
            throw new Error('FilePath cannot be empty');
        }
        this._value = value;
    }

    get value(): string {
        return this._value;
    }

    toString(): string {
        return this._value;
    }
}
```

For simple wrappers, classes may be used:

```typescript
export class FieldNameVO {
    constructor(private readonly _value: string) {}
    get value(): string { return this._value; }
}

export class BooleanVO {
    constructor(private readonly _value: boolean) {}
    get value(): boolean { return this._value; }
}

export class SeverityVO {
    constructor(private readonly _value: number) {}
    get value(): number { return this._value; }
}
```

---

### Composite Value Objects

Composite VOs should use other VOs as fields, not raw primitives.

Bad:

```typescript
export class ImportRuleVO {
    constructor(
        private readonly pattern: string,
        private readonly message: string,
    ) {}
}
```

Good:

```typescript
export class ImportRuleVO {
    private readonly _pattern: RulePattern;
    private readonly _message: RuleMessage;

    constructor(pattern: RulePattern, message: RuleMessage) {
        if (!pattern.value.trim()) {
            throw new Error('RulePattern cannot be empty');
        }
        this._pattern = pattern;
        this._message = message;
    }

    get pattern(): RulePattern {
        return this._pattern;
    }

    get message(): RuleMessage {
        return this._message;
    }
}
```

---

### Entities (`_entity.ts`)

Entities represent domain objects with identity.

```typescript
export class SymbolEntity {
    private readonly _id: SymbolId;
    private readonly _name: SymbolName;

    constructor(id: SymbolId, name: SymbolName) {
        this._id = id;
        this._name = name;
    }

    get id(): SymbolId {
        return this._id;
    }

    get name(): SymbolName {
        return this._name;
    }
}
```

---

### Error Types (`_error.ts`)

Use TypeScript Error classes.

Prefer VO fields instead of raw public strings.

Bad:

```typescript
export class ConfigError extends Error {
    constructor(
        public readonly key: string,
        public readonly message: string,
    ) {
        super(`Config error: ${key} - ${message}`);
        this.name = 'ConfigError';
    }
}
```

Good:

```typescript
export class ConfigError extends Error {
    private readonly _key: ConfigKey;
    private readonly _message: ErrorMessage;

    constructor(key: ConfigKey, message: ErrorMessage) {
        super(`Config error for ${key.value}: ${message.value}`);
        this.name = 'ConfigError';
        this._key = key;
        this._message = message;
    }

    get key(): ConfigKey {
        return this._key;
    }

    get message(): ErrorMessage {
        return this._message;
    }
}
```

If an error wraps lower-level errors:

```typescript
export class FileReadError extends Error {
    private readonly _path: FilePath;
    private readonly _cause: Error;

    constructor(path: FilePath, cause: Error) {
        super(`Failed to read file ${path.value}: ${cause.message}`);
        this.name = 'FileReadError';
        this._path = path;
        this._cause = cause;
    }

    get path(): FilePath {
        return this._path;
    }

    get cause(): Error {
        return this._cause;
    }
}
```

---

### Event Types (`_event.ts`)

Events represent something that happened in the domain.

```typescript
export class ScanCompletedEvent {
    private readonly _scanId: ScanId;

    constructor(scanId: ScanId) {
        this._scanId = scanId;
    }

    get scanId(): ScanId {
        return this._scanId;
    }
}
```

---

### Constants (`_constant.ts`)

Constants are pure static values.

```typescript
/** Default frames per second for animation. */
export const FPS_DEFAULT: number = 24.0;

/** Minimum reveal time in seconds. */
export const MIN_REVEAL_SECONDS: number = 0.5;

/** Manifest filename. */
export const MANIFEST_FILENAME: string = 'manifest.json';
```

Rules:

- no functions,
- no I/O,
- no external layer imports,
- no mutable state.

Constants may be primitive scalars. Consumers should wrap domain-meaningful primitives into VOs when exposing them in public domain contracts.

---

## Utility Functions (`_utility.ts`)

Utility files contain pure reusable tools.

### The Ultimate Boundary

A function belongs in `*_utility.ts` ONLY if ALL of these are true:

1. Stateless: no `this`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no randomness, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Multi-consumer reusable: useful for multiple modules/layers.

---

### Good Utility Example

```typescript
// taxonomy_token_utility.ts

export function matchWholeToken(haystack: string, needle: string): boolean {
    if (!needle) {
        return false;
    }

    const pattern = new RegExp(`(?<!\\w)${escapeRegex(needle)}(?!\\w)`);
    return pattern.test(haystack);
}

function escapeRegex(str: string): string {
    return str.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}
```

This is a dumb reusable tool.

---

### Bad Utility: Domain Knowledge

```typescript
// BAD: knows AES layer mapping rules
export function getTargetLayerFromSuffix(suffix: string): string {
    switch (suffix) {
        case 'port': return 'infrastructure';
        case 'protocol': return 'capabilities';
        default: return 'unknown';
    }
}
```

This belongs in capabilities as a private helper.

---

### Bad Utility: Single Consumer Only

```typescript
// BAD: only used by one checker
export function formatImportViolation(rule: ImportRuleVO): string {
    return `Import rule violation: ${rule.pattern.value}`;
}
```

If only one capability uses it, keep it as a private helper in that capability.

---

## Primitive-to-VO Rules

Taxonomy is the layer that provides VO replacements for primitives.

### General Rule

Domain data MUST use VOs, not raw primitives.

Bad:

```typescript
export interface LintResult {
    filePath: string;
    line: number;
    severity: string;
}
```

Good:

```typescript
export interface LintResult {
    filePath: FilePath;
    line: LineNumber;
    severity: Severity;
}
```

---

### Primitive Policy

This policy must stay consistent with capabilities and infrastructure skills.

| Primitive  | Rule                                                                                |
| ---------- | ----------------------------------------------------------------------------------- |
| `string`   | Forbidden for domain fields and public contract return values. Use VO.              |
| `number`   | Forbidden for domain values. Use VO.                                                |
| `boolean`  | Allowed for semantic toggles when no richer VO is needed.                           |

Prefer VOs for:

- file paths,
- symbol names,
- messages,
- line numbers,
- column numbers,
- severity levels,
- durations,
- counts,
- thresholds,
- identifiers.

---

### VO Construction Rules

VOs MUST validate on construction when the domain has invariants.

Good:

```typescript
export class LineNumber {
    private readonly _value: number;

    constructor(value: number) {
        if (value === 0) {
            throw new Error('LineNumber must be positive');
        }
        this._value = value;
    }

    get value(): number {
        return this._value;
    }
}
```

If validation cannot fail, a simpler constructor may be used.

---

### Optional and Collection Primitives

Bad:

```typescript
export interface RuleSet {
    patterns: string[];
    description: string | null;
}
```

Good:

```typescript
export interface RuleSet {
    patterns: PatternList;
    description: RuleDescription | null;
}
```

Use:

- list VOs for collections,
- optional VOs or `VO | null` when semantically optional.

---

## Detection Patterns

### BAD: Interface Defined in Capabilities

```typescript
// capabilities_orphan_analyzer.ts

interface OrphanResult {
    isOrphan: boolean;
    reason: string;
}
```

Fix:

Move to taxonomy.

```typescript
// shared/orphan_detector/taxonomy_orphan_result_vo.ts
export interface OrphanResult {
    readonly isOrphan: OrphanFlag;
    readonly reason: OrphanReason;
}
```

Then import:

```typescript
import { OrphanResult } from '../shared/orphan_detector/taxonomy_orphan_result_vo';
```

---

### BAD: Interface Defined in Infrastructure

```typescript
// infrastructure_file_cache.ts

interface CacheEntry {
    key: string;
    value: string;
}
```

Fix:

```typescript
// shared/cache/taxonomy_cache_entry_vo.ts
export interface CacheEntry {
    readonly key: CacheKey;
    readonly value: CacheValue;
}
```

---

### BAD: Raw Primitive Fields in Taxonomy VO

```typescript
export interface ImportRuleVO {
    pattern: string;
    message: string;
}
```

Fix:

```typescript
export interface ImportRuleVO {
    readonly pattern: RulePattern;
    readonly message: RuleMessage;
}
```

---

### BAD: Taxonomy Importing Layer Code

```typescript
// taxonomy_orphan_vo.ts

import { OrphanAnalyzer } from '../capabilities_orphan_analyzer'; // BAD
```

Taxonomy must not import from layers.

---

### BAD: Domain Rule Inside Utility

```typescript
// taxonomy_layer_utility.ts

export function isPortTraitName(name: string): boolean {
    return name.endsWith('Port');
}
```

If this knows AES naming conventions or layer rules, it is domain knowledge.

It belongs in capabilities as a helper, not taxonomy utility.

---

### GOOD: Interface in Taxonomy + Implementor with DI

```typescript
// shared/orphan_detector/taxonomy_orphan_analysis_result_vo.ts
export interface OrphanAnalysisResult {
    readonly isOrphan: OrphanFlag;
    readonly reason: OrphanReason;
}
```

```typescript
// capabilities_orphan_analyzer.ts
import { OrphanAnalysisResult } from '../shared/orphan_detector/taxonomy_orphan_analysis_result_vo';
import { IOrphanFilenameExtractorProtocol } from '../shared/orphan_detector/contract_orphan_filename_extractor_protocol';
import { IOrphanFileCachePort } from '../shared/orphan_detector/contract_orphan_file_cache_port';

export class CapabilitiesOrphanAnalyzer {
    constructor(
        private readonly extractor: IOrphanFilenameExtractorProtocol,
        private readonly cache: IOrphanFileCachePort,
    ) {}
}
```

Service dependencies use DI.

Value/result data comes from taxonomy.

---

## Workflow

### Step 1: Identify the Data Type

When you find an interface or type in a layer file, ask:

> Is this a data type or an implementor?

If it carries domain data:

- result object,
- DTO,
- VO,
- entity,
- error,
- event,
- enum,
- constant,

then move it to taxonomy.

If it implements an interface and uses DI, keep it in the layer file.

---

### Step 2: Determine Taxonomy Domain

Choose the correct domain directory under:

```text
packages/shared/src/<domain>/
```

Examples:

| Domain          | Directory                       | Example Types                         |
| --------------- | ------------------------------- | ------------------------------------- |
| common          | `shared/src/common/`          | cross-domain VOs, errors, utilities   |
| orphan_detector | `shared/src/orphan_detector/` | orphan results, reasons, flags        |
| code_analysis   | `shared/src/code_analysis/`   | analysis results, symbols, violations |
| import_rules    | `shared/src/import_rules/`    | import rules, patterns, messages      |
| naming_rules    | `shared/src/naming_rules/`    | naming violations, patterns           |

If a type is used by multiple domains, put it in `common/`.

---

### Step 3: Create or Update Taxonomy File

Use the correct suffix:

```text
taxonomy_*_vo.ts
taxonomy_*_entity.ts
taxonomy_*_error.ts
taxonomy_*_event.ts
taxonomy_*_constant.ts
taxonomy_*_utility.ts
```

Example:

```bash
mkdir -p packages/shared/src/orphan_detector
touch packages/shared/src/orphan_detector/taxonomy_orphan_result_vo.ts
```

---

### Step 4: Register Module

Update the domain `index.ts`.

```typescript
// shared/src/orphan_detector/index.ts

export { OrphanResult } from './taxonomy_orphan_result_vo';
export { OrphanReason } from './taxonomy_orphan_reason_vo';
export { IOrphanProtocol } from './contract_orphan_protocol';
export { IOrphanFileCachePort } from './contract_orphan_file_cache_port';
```

---

### Step 5: Update Imports in Layer Files

Before:

```typescript
interface OrphanResult {
    isOrphan: boolean;
    reason: string;
}
```

After:

```typescript
import { OrphanResult } from '../shared/orphan_detector/taxonomy_orphan_result_vo';
```

---

### Step 6: Verify Purity

Check that taxonomy files do not import from:

- capabilities,
- infrastructure,
- agents,
- surface,
- root containers,
- contract interfaces.

Also check that taxonomy utilities do not perform I/O.

---

### Step 7: Verify Primitive-to-VO Compliance

Ensure:

- no public raw `string` domain fields,
- no numeric primitive domain fields,
- VOs validate on construction,
- contract interfaces use taxonomy VOs.

---

### Step 8: Verify Compilation

```bash
npx tsc --noEmit
```

---

## Verification Checklist

- [ ] All domain data types live in shared/taxonomy.
- [ ] No domain interfaces/types with data are defined in layer files.
- [ ] Taxonomy file naming uses allowed suffixes only.
- [ ] Taxonomy files do not import from capabilities.
- [ ] Taxonomy files do not import from infrastructure.
- [ ] Taxonomy files do not import from agents.
- [ ] Taxonomy files do not import from surface.
- [ ] Taxonomy files do not import from root containers.
- [ ] Taxonomy files do not import contract interfaces.
- [ ] Taxonomy files contain no I/O.
- [ ] Taxonomy utilities are stateless, pure, domain-agnostic, and multi-consumer.
- [ ] Domain-specific stateless helpers are NOT forced into taxonomy utility.
- [ ] Single-consumer helpers remain in their consuming layer.
- [ ] Value objects validate on construction when invariants exist.
- [ ] Single-value VOs expose safe constructors and accessors.
- [ ] Composite VOs use other VOs instead of raw primitives.
- [ ] Error types extend `Error`.
- [ ] Constants are pure static values.
- [ ] New taxonomy modules are registered in `index.ts`.
- [ ] `npx tsc --noEmit` passes.

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `npx tsc --noEmit` or AST-based tooling.

```bash
# Find possible data types in layer files
grep -rn "^interface\|^type \|^enum " packages/*/src/ --exclude-dir=shared

# Check forbidden imports in taxonomy files
grep -n "from.*capabilities_\|from.*infrastructure_\|from.*agent_\|from.*surface_" packages/shared/src/*/taxonomy_*.ts

# Check possible I/O in taxonomy files
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios\|sqlite3\|pg" packages/shared/src/*/taxonomy_*.ts

# List registered taxonomy modules
grep -n "^export.*from.*taxonomy_" packages/shared/src/*/index.ts

# Find magic constants in layer files
grep -n "[0-9]\+\.[0-9]\+" packages/*/src/ --exclude-dir=shared | grep -v "//\|const\|import" | head -20
```

---

### Check Unregistered Taxonomy Files

```bash
for file in packages/shared/src/<domain>/taxonomy_*.ts; do
  name=$(basename "$file" .ts)
  grep -q "from.*$name" packages/shared/src/<domain>/index.ts \
    || echo "UNREGISTERED: $name"
done
```

---

## Naming Convention

| Layer          | File Pattern                | Suffix                        |
| -------------- | --------------------------- | ----------------------------- |
| root           | `root_*_container.ts`     | `_container`                |
| taxonomy       | `taxonomy_*_vo.ts`        | `_vo`                       |
| taxonomy       | `taxonomy_*_entity.ts`    | `_entity`                   |
| taxonomy       | `taxonomy_*_error.ts`     | `_error`                    |
| taxonomy       | `taxonomy_*_event.ts`     | `_event`                    |
| taxonomy       | `taxonomy_*_constant.ts`  | `_constant`                 |
| taxonomy       | `taxonomy_*_utility.ts`   | `_utility`                  |
| contract       | `contract_*_protocol.ts`  | `_protocol`                 |
| contract       | `contract_*_port.ts`      | `_port`                     |
| contract       | `contract_*_aggregate.ts` | `_aggregate`                |
| capabilities   | `capabilities_*.ts`       | flexible                      |
| infrastructure | `infrastructure_*.ts`     | flexible                      |
| agent          | `agent_*.ts`              | `_orchestrator`             |
| surface        | `surface_*.ts`            | `_command`, `_controller` |

---

## Magic Constant Definitions

All domain constants MUST live in taxonomy constant files.

```typescript
// packages/shared/src/animator/taxonomy_animator_constant.ts

/** Default frames per second for animation. */
export const FPS_DEFAULT: number = 24.0;

/** Minimum reveal time in seconds. */
export const MIN_REVEAL_SECONDS: number = 0.5;

/** Manifest filename. */
export const MANIFEST_FILENAME: string = 'manifest.json';
```

Layer consumption:

```typescript
import { FPS_DEFAULT } from '../shared/animator/taxonomy_animator_constant';
```

```typescript
import { MIN_REVEAL_SECONDS } from '../shared/animator/taxonomy_animator_constant';
```

```typescript
import { MANIFEST_FILENAME } from '../shared/animator/taxonomy_animator_constant';
```

If a constant represents a domain value, wrap it in a VO at the consuming boundary when exposing it through public domain contracts.

---

## Common Mistakes

- Defining interfaces/types in layer files.
- Defining domain enums in layer files.
- Importing non-taxonomy layer types into taxonomy files.
- Importing contract interfaces into taxonomy files.
- Using wrong suffix for taxonomy files.
- Forgetting to register taxonomy modules in `index.ts`.
- Putting domain knowledge into `*_utility.ts`.
- Putting single-consumer helpers into `*_utility.ts`.
- Keeping reusable domain-agnostic utilities inside layer files.
- Exposing public raw `string` fields in VOs.
- Exposing public numeric primitive fields in domain types.
- Creating VOs without validation when domain invariants exist.
- Duplicating taxonomy types across domains.
- Putting cross-domain types in a specific domain instead of `common/`.
- Creating taxonomy utility functions with I/O.
- Treating every stateless function as utility.
- Treating every concrete field as DI violation.
- Forgetting that value fields may be shared VOs, while service fields must use DI.
