---
name: create-capabilities-typescript
description: "Create and validate TypeScript capabilities layer files following AES rules: pure domain behavior, zero I/O, 3-block structure, one class per file, protocol interface contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    typescript,
    aes,
    capability,
    protocol,
    structure,
    aes402,
    aes403,
    aes404,
    3-block-structure,
    di,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create capability typescript"
  - "add capability typescript"
  - "fix capability structure typescript"
  - "create protocol typescript"
  - "capability missing protocol typescript"
  - "check capabilities typescript"
  - "audit capabilities typescript"
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

Create and validate TypeScript **capabilities layer** files following clean architecture / AES rules.

A capabilities file must contain **pure domain behavior**:

- no I/O,
- no infrastructure detail,
- no agent detail,
- no locally defined domain data structures,
- one implementation class per file,
- one domain protocol interface as the public contract,
- strict 3-block structure,
- dependency injection for service collaborators,
- shared VOs for domain data.

---

## Definition of Done

A capabilities file is considered valid when:

1. It contains exactly **ONE implementation class**.
2. The class implements exactly **ONE domain protocol interface**.
3. Block 2 contains **ONLY** the domain protocol method implementations.
4. Utility methods, static factories, and private helpers are placed in Block 3.
5. The file contains **zero I/O** and zero side-effecting infrastructure calls.
6. The file does **not** define domain data structures locally.
7. Service dependencies use DI via protocol interfaces.
8. Value/configuration fields use shared VOs, not raw primitives.
9. Reusable, stateless, domain-agnostic functions are extracted to `*_utility.ts`.
10. Domain-specific helpers may remain inside the implementation file.
11. `npx tsc --noEmit` passes.

---

## Rules

### Layer Boundaries (AES)

#### Capabilities Layer (`capabilities_*.ts`)

| Allowed                                      | Forbidden                                             |
| -------------------------------------------- | ----------------------------------------------------- |
| Computation, validation, calculation         | File I/O (`fs.`, `readFile`, `writeFile`)             |
| Data transformation, business rules          | Network calls (`fetch`, `axios`, `http`)              |
| Domain behavior using shared models          | Database operations (`sqlite3`, `pg`)                 |
| Interface implementation                     | Direct stdout/stderr printing                         |
| Private helpers supporting the impl class    | Direct environment/system-clock/global-state mutation |
| Calling injected port/protocol traits        | Direct import from `infrastructure_*`                 |
|                                              | Direct import from `agent_*`                          |
|                                              | Direct dependency on concrete `capabilities_*` modules |
|                                              | Locally defined domain data structures                |

Capabilities may depend on:

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- protocol/port interfaces

Capabilities must not depend on concrete infrastructure or concrete agent implementations.

---

### Structural Rules

#### 1. One implementation class per file

Each capabilities file contains exactly ONE main implementation class.

```typescript
export class CapabilitiesOrphanAnalyzer {
    // ...
}
```

Do not define multiple service classes in the same file.

---

#### 2. Only the implementation class may be defined in the layer file

A capabilities file may define the implementation class only.

Domain data structures MUST live in `shared/<domain>/taxonomy_*`.

Forbidden in capabilities files:

```typescript
interface OrphanResult {
    isOrphan: boolean;
    reason: string;
}
```

Allowed:

```typescript
import { OrphanResult } from '../shared/orphan_detector/taxonomy_orphan_result_vo';
```

---

#### 3. Service dependencies must use DI

Fields that represent collaborators, services, adapters, or ports MUST use protocol interfaces.

```typescript
export class CapabilitiesOrphanAnalyzer {
    constructor(
        private readonly extractor: IOrphanFilenameExtractorProtocol,
        private readonly cache: IOrphanFileCachePort,
    ) {}
}
```

Do not use concrete service types:

```typescript
export class CapabilitiesOrphanAnalyzer {
    constructor(private readonly extractor: FilenameExtractor) {} // BAD: concrete dependency
}
```

---

#### 4. Value/configuration fields must use shared VOs

Fields that represent domain values, configuration, identifiers, paths, names, thresholds, etc. should use shared VOs.

```typescript
export class FrameExporter {
    constructor(private readonly outputDir: OutputDirectory) {} // shared VO
}
```

Avoid raw primitives for domain values:

```typescript
export class FrameExporter {
    constructor(private readonly outputDir: string) {} // BAD: primitive domain value
}
```

---

### Helper vs Utility Decision

The boundary is not only about `this`.

The real question is:

> Does this function know about specific business/domain rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this class, or by multiple modules?

---

#### Keep as Private Helper in Block 3

Keep the function inside the capabilities file if ANY of these is true:

1. It contains business/domain rules.
2. It knows AES-specific patterns, layer names, suffixes, violation codes, or taxonomy conventions.
3. It accesses `this.field` or instance state.
4. It is tightly coupled to this capability only.
5. It is a factory method such as `static create()` or `static from()`.
6. It is stateless but only used by this one class and is domain-specific.

Example:

```typescript
class ContractRoleChecker {
    private resolveScope(scope: string): [string, string[]] {
        // Domain-specific parsing logic.
        // Even without `this`, this can remain a private helper
        // if only this checker uses it.
        ...
    }
}
```

---

#### Extract to Utility (`*_utility.ts`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `this`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business rules.
5. Reusable: useful for multiple capabilities/infrastructures/modules.

Example:

```typescript
// shared/code_analysis/taxonomy_string_utility.ts
export function matchWholeToken(haystack: string, needle: string): boolean {
    // generic token matching
    ...
}
```

---

#### I/O Blocker

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It also MUST NOT stay in capabilities.

```typescript
// BAD in capabilities layer
function readConfig(filePath: string): string | null {
    return fs.readFileSync(filePath, 'utf-8'); // I/O
}
```

Correct placement:

```typescript
// infrastructure_config_reader.ts
export class FileSystemConfigReader implements IConfigReaderPort {
    read(filePath: FilePath): Result<ConfigContent, ConfigReadError> {
        try {
            const raw = fs.readFileSync(filePath.value(), 'utf-8');
            return Ok(ConfigContent.new(raw));
        } catch (e) {
            return Err(new ConfigReadError.Io(e));
        }
    }
}
```

Rule:

```text
Stateless + I/O = infrastructure/port implementation
NOT taxonomy utility
NOT capabilities layer
```

---

## The 3-Block Structure

Every implementation file MUST follow this order **within the class body**:

1. **Block 1 — Class Definition & Constructor**
2. **Block 2 — Domain Protocol Method Implementation**
3. **Block 3 — Utility Methods, Factories, and Private Helpers**

---

### Block 1 — Class Definition & Constructor

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    constructor() {}
}
```

Or with dependencies:

```typescript
export class CapabilitiesOrphanAnalyzer implements ILineCheckerProtocol {
    constructor(
        private readonly extractor: IOrphanFilenameExtractorProtocol,
        private readonly cache: IOrphanFileCachePort,
        private readonly policy: OrphanAnalysisPolicy,
    ) {}
}
```

---

### Block 2 — Public Contract

Block 2 is RESERVED for the domain protocol methods ONLY.

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    checkLineCounts(
        file: FilePath,
        definition: LayerDefinition | null,
        source: SourceContentVO,
        violations: LintResult[],
    ): void {
        // domain behavior
        ...
    }
}
```

Do NOT put these in Block 2:

```typescript
toString(): string
toJSON(): object
valueOf(): unknown
equals(other: unknown): boolean
[Symbol.iterator](): Iterator<...>
static create(): ArchLineChecker
static from(...): ArchLineChecker
private helper(...): ...
```

Those belong in Block 3.

---

### Block 3 — Utility Methods, Factories, and Helpers

Block 3 contains:

- Utility/serialization methods: `toString()`, `toJSON()`, `valueOf()`, `equals()`
- Symbol methods: `[Symbol.iterator]()`, `[Symbol.toPrimitive]()`
- Static factory methods: `static create()`, `static from()`, `static of()`
- `private` helper methods that use `this`
- `private static` helpers that use class-level state

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    toString(): string {
        return 'ArchLineChecker()';
    }

    equals(other: unknown): boolean {
        return other instanceof ArchLineChecker;
    }

    static create(): ArchLineChecker {
        return new ArchLineChecker();
    }

    private resolveThreshold(layer: string): number {
        // private helper
        ...
    }
}
```

---

### Utility Functions Do Not Belong in Block 3

If a function is:

- stateless,
- pure,
- domain-agnostic,
- and reusable across multiple modules,

then extract it to shared utility.

```typescript
import { isBarrelFile } from '../shared/code_analysis/taxonomy_line_checker_utility';
```

But if the function is domain-specific and only used by this class, it may remain in Block 3.

---

### Method Placement Decision Rule

```text
Method / function found in a capabilities file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in capabilities)
  │
  ├─ Is it defined in the I<Name>Protocol interface?
  │   └─ YES → Block 2
  │
  ├─ Is it a utility/serialization method? (toString, toJSON, valueOf, equals)
  │   └─ YES → Block 3
  │
  ├─ Is it a Symbol method? ([Symbol.iterator], [Symbol.toPrimitive])
  │   └─ YES → Block 3
  │
  ├─ Is it a static factory method? (static create, static from, static of)
  │   └─ YES → Block 3
  │
  ├─ Is it a static method?
  │   ├─ Uses class-level state (static fields)?
  │   │   └─ YES → Block 3 (keep as private static)
  │   ├─ Tightly coupled to class semantics?
  │   │   └─ YES → Block 3 (keep as static)
  │   └─ Pure logic, no class dependency?
  │       └─ YES → EXTRACT to *_utility.ts
  │
  └─ Is it a private instance method using this?
      └─ YES → Block 3
```

---

## Example: Correct 3-Block Structure

```typescript
import { FilePath } from '../shared/code_analysis/taxonomy_file_path_vo';
import { LayerDefinition } from '../shared/code_analysis/taxonomy_layer_definition_vo';
import { ILineCheckerProtocol } from '../shared/code_analysis/contract_line_checker_protocol';
import { isBarrelFile } from '../shared/code_analysis/taxonomy_line_checker_utility';
import { LintResult } from '../shared/code_analysis/taxonomy_lint_result_vo';
import { SourceContentVO } from '../shared/code_analysis/taxonomy_source_vo';


// ─── Block 1: Class Definition & Constructor ──────────────
export class ArchLineChecker implements ILineCheckerProtocol {
    constructor() {}


    // ─── Block 2: Public Contract (domain protocol ONLY) ──
    checkLineCounts(
        file: FilePath,
        definition: LayerDefinition | null,
        source: SourceContentVO,
        violations: LintResult[],
    ): void {
        const basename = file.basename();

        if (isBarrelFile(basename)) {
            return;
        }

        // Remaining domain logic...
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

    private isLayerRelevant(definition: LayerDefinition): boolean {
        // Private helper specific to this checker.
        return true;
    }
}
```

---

## Protocol Rules

### AES403 — Capability Must Implement Protocol Interface

Every capability class MUST implement a domain protocol interface.

```typescript
export class CapabilitiesOrphanAnalyzer implements IOrphanCheckerProtocol {
    // public contract
    ...
}
```

---

### Protocol file naming

| Layer          | File Pattern            | Protocol File                       | Protocol Name           |
| -------------- | ----------------------- | ----------------------------------- | ----------------------- |
| Capabilities   | `capabilities_*.ts`   | `contract_<name>_protocol.ts`     | `I<Name>Protocol`     |
| Infrastructure | `infrastructure_*.ts` | `contract_<name>_port.ts`         | `I<Name>Port`         |
| Agents         | `agent_*.ts`          | `contract_<name>_aggregate.ts`    | `I<Name>Aggregate`    |

---

### Protocol content rules

The protocol interface MUST contain only public domain contract methods.

Good:

```typescript
export interface ILineCheckerProtocol {
    checkLineCounts(
        file: FilePath,
        definition: LayerDefinition | null,
        source: SourceContentVO,
        violations: LintResult[],
    ): void;
}
```

Bad:

```typescript
export interface ILineCheckerProtocol {
    checkLineCounts(...): void;

    privateHelper(): void; // BAD: helper in interface
}
```

---

### Constructors are not protocol methods

`constructor` and static factory methods MUST stay in Block 1 / Block 3.

Bad:

```typescript
export interface ILineCheckerProtocol {
    create(): ILineCheckerProtocol; // BAD
}
```

Good:

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    static create(): ArchLineChecker {
        return new ArchLineChecker();
    }
}
```

---

## Detection Patterns

### BAD: Capability Without Interface (AES403)

```typescript
export class FrameComposer {
    composeFrame(): void {
        // public behavior without protocol interface
        ...
    }
}
```

Fix:

```typescript
export class FrameComposer implements IFrameComposerProtocol {
    composeFrame(): void {
        // contract implementation
        ...
    }
}
```

---

### BAD: I/O in Capabilities (AES404)

```typescript
export class MyCapability {
    process(): void {
        const content = fs.readFileSync('file.txt', 'utf-8'); // FORBIDDEN
    }
}
```

Fix:

Move I/O to infrastructure or port implementation.

```typescript
// infrastructure_source_reader.ts
export class FileSystemSourceReader implements ISourceReaderPort {
    read(path: FilePath): Result<SourceContentVO, SourceReadError> {
        try {
            const raw = fs.readFileSync(path.value(), 'utf-8');
            return Ok(SourceContentVO.new(path, raw));
        } catch (e) {
            return Err(new SourceReadError.Io(e));
        }
    }
}
```

Capabilities receives already-loaded data:

```typescript
export class ImportChecker implements IImportCheckerProtocol {
    check(source: SourceContentVO): LintResult[] {
        // pure analysis
        return [];
    }
}
```

---

### BAD: Interface Defined in Layer File

```typescript
interface OrphanResult {  // ← INTERFACE — should be in shared/taxonomy
    isOrphan: boolean;
    reason: string;
}

class CapabilitiesOrphanAnalyzer {
    result: OrphanResult;  // ← concrete type, not DI
}
```

Fix:

Move to shared taxonomy:

```typescript
// shared/orphan_detector/taxonomy_orphan_result_vo.ts
export interface OrphanResult {
    readonly isOrphan: OrphanFlag;
    readonly reason: OrphanReason;
}
```

Then import it:

```typescript
import { OrphanResult } from '../shared/orphan_detector/taxonomy_orphan_result_vo';
```

---

### BAD: Concrete Service Field

```typescript
export class CapabilitiesOrphanAnalyzer {
    constructor(private readonly extractor: FilenameExtractor) {} // BAD
}
```

Fix:

```typescript
export class CapabilitiesOrphanAnalyzer {
    constructor(private readonly extractor: IOrphanFilenameExtractorProtocol) {}
}
```

---

### BAD: Utility Methods in Block 2

```typescript
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

Fix:

```typescript
export class ArchLineChecker implements ILineCheckerProtocol {
    constructor() {}

    checkLineCounts(...): void {            // ← Block 2: protocol method
        ...
    }

    toString(): string {                    // ← Block 3: utility method
        return 'ArchLineChecker()';
    }

    equals(other: unknown): boolean {       // ← Block 3
        return other instanceof ArchLineChecker;
    }
}
```

---

### GOOD: Capability with DI and Shared VO

```typescript
import { OrphanAnalysisPolicy } from '../shared/orphan_detector/taxonomy_orphan_analysis_policy_vo';
import { IOrphanFileCachePort } from '../shared/orphan_detector/contract_orphan_file_cache_port';
import { IOrphanFilenameExtractorProtocol } from '../shared/orphan_detector/contract_orphan_filename_extractor_protocol';
import { ICapabilitiesOrphanProtocol } from '../shared/orphan_detector/contract_capabilities_orphan_protocol';

export class CapabilitiesOrphanAnalyzer implements ICapabilitiesOrphanProtocol {
    constructor(
        private readonly extractor: IOrphanFilenameExtractorProtocol,
        private readonly cache: IOrphanFileCachePort,
        private readonly policy: OrphanAnalysisPolicy,
    ) {}
}
```

---

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask:

> Is this pure domain behavior?

If yes → keep as capabilities.

If no → move I/O or side-effecting code to infrastructure.

Examples of code that must move out of capabilities:

- `fs.*`, `readFile`, `writeFile`
- `fetch`, `axios`, `http`
- `sqlite3`, `pg`
- direct `console.log`
- environment mutation
- system clock access
- global state mutation

---

### Step 2: Check Missing Interface (AES403)

Does the capability class implement a protocol interface?

If no:

1. create `contract_<name>_protocol.ts`
2. define `I<Name>Protocol`
3. move public domain method signatures into the interface
4. make the class implement the interface

---

### Step 3: Create Interface File if Missing

Create interface file in the appropriate shared domain folder.

Examples:

| Package        | Protocol Path                                                  |
| -------------- | -------------------------------------------------------------- |
| import-rules   | `packages/shared/src/import_rules/contract_*_protocol.ts`    |
| code-analysis  | `packages/shared/src/code_analysis/contract_*_protocol.ts`   |
| orphan-detector| `packages/shared/src/orphan_detector/contract_*_protocol.ts` |

Register the module in the relevant `index.ts`.

---

### Step 4: Enforce 3-Block Structure

Reorganize the file into:

1. class definition + `constructor`
2. domain protocol method implementations
3. utility methods, static factories, private helpers

---

### Step 5: Verify Class Discipline

Check:

- exactly one implementation class
- no local domain data interfaces/types
- no local enums/VOs/DTOs/constants
- service fields use protocol interfaces
- value fields use shared VOs

---

### Step 6: Verify Helper vs Utility Boundary

For each helper/function:

```text
Does it know domain rules?
├─ YES → keep as helper in Block 3
└─ NO
   Is it stateless, pure, and reusable by multiple modules?
   ├─ YES → extract to *_utility.ts
   └─ NO → keep as helper in Block 3
```

---

### Step 7: Verify Layer Compliance

Ensure no forbidden imports or I/O patterns.

---

### Step 8: Verify Error Handling, VO, and Constants

Check:

- no silent `?? ''` or `|| 0` error swallowing
- fallible operations return descriptive error types or throw meaningful errors
- check/analysis methods may return `LintResult[]`
- domain data uses VOs
- no magic constants

---

### Step 9: Verify Compilation

Run:

```bash
npx tsc --noEmit
```

---

## Verification Checklist

- [ ] File follows the 3-Block Structure.
- [ ] Block 1 contains exactly one implementation class + `constructor`.
- [ ] Block 2 contains ONLY the domain protocol method implementations.
- [ ] Block 3 contains utility methods, factories, and private helpers.
- [ ] Capability class implements a protocol interface (AES403).
- [ ] Interface contains only public domain contract methods.
- [ ] Private helpers are not declared in the interface.
- [ ] Constructors are not declared in the interface.
- [ ] Utility methods are in Block 3.
- [ ] Domain-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `*_utility.ts`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] One file contains exactly one implementation class.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use protocol interfaces via DI.
- [ ] Value/configuration fields use shared VOs.
- [ ] Zero I/O in capabilities layer (AES404).
- [ ] No forbidden imports from `infrastructure_*`.
- [ ] No forbidden imports from `agent_*`.
- [ ] No direct dependency on concrete `capabilities_*` implementations.
- [ ] Protocol module is registered in the shared package's `index.ts`.
- [ ] `npx tsc --noEmit` passes.

---

## Error Handling Rules

Capabilities error handling must be explicit.

### Rule 1: Do not silently discard errors

Forbidden:

```typescript
const value = result ?? '';
```

Forbidden:

```typescript
const value = result || 0;
```

---

### Rule 2: Fallible operations should return `Result` or throw

If a method represents an operation that can fail unexpectedly, return a result type or throw a meaningful error.

```typescript
function parseManifest(content: ManifestContent): Result<Manifest, ManifestParseError> {
    // ...
    ...
}
```

---

### Rule 3: Check/analysis methods may return `LintResult[]`

For linting/analysis use cases, violations are expected domain outcomes.

```typescript
function checkImports(source: SourceContentVO): LintResult[] {
    const violations: LintResult[] = [];

    // analysis logic

    return violations;
}
```

This is allowed.

---

### Rule 4: I/O errors belong to infrastructure/port implementations

Bad in capabilities:

```typescript
function checkFile(path: FilePath): LintResult[] {
    const content = fs.readFileSync(path.value(), 'utf-8'); // BAD: I/O in capabilities
    return [];
}
```

Good:

```typescript
// infrastructure_source_reader.ts
export class FileSystemSourceReader implements ISourceReaderPort {
    read(path: FilePath): Result<SourceContentVO, SourceReadError> {
        try {
            const raw = fs.readFileSync(path.value(), 'utf-8');
            return Ok(SourceContentVO.new(path, raw));
        } catch (e) {
            return Err(new SourceReadError.Io(e));
        }
    }
}
```

```typescript
// capabilities_import_checker.ts
export class ImportChecker implements IImportCheckerProtocol {
    check(source: SourceContentVO): LintResult[] {
        // pure analysis using already-read source
        return [];
    }
}
```

---

## Primitive-to-VO Replacement Rules (AES402)

### General Rule

Domain data MUST use shared VOs, not raw primitives.

Bad:

```typescript
interface LintResult {
    filePath: string;
    line: number;
    severity: string;
}
```

Good:

```typescript
interface LintResult {
    filePath: FilePath;
    line: LineNumber;
    severity: Severity;
}
```

---

### Primitive Policy

| Primitive   | Rule                                                                                |
| ----------- | ----------------------------------------------------------------------------------- |
| `string`    | Forbidden for domain fields and contract return values. Use VO.                     |
| `number`    | Forbidden. Use domain VO.                                                           |
| `boolean`   | Allowed for semantic toggles when no richer VO is needed.                           |

Prefer VOs for:

- file paths
- symbol names
- messages
- line numbers
- column numbers
- severity
- durations
- counts
- thresholds
- identifiers

---

## Magic Constant Extraction Rules

No hardcoded domain literals in capabilities.

Bad:

```typescript
function calculateDuration(): number {
    return 0.5;
}
```

Good:

```typescript
import { MIN_REVEAL_SECONDS } from '../shared/animator/taxonomy_animator_constant';

function calculateDuration(): number {
    return MIN_REVEAL_SECONDS;
}
```

Constants MUST live in:

```text
taxonomy_*_constant.ts
```

---

## Import Strategy

When fixing cross-import violations in capabilities, choose one of these options.

---

### Option A: Extract to Taxonomy Utility

Use when the code is:

- stateless,
- pure,
- domain-agnostic,
- reusable by multiple modules.

Example:

```typescript
// shared/code_analysis/taxonomy_path_utility.ts
export function normalizeRelativePath(path: string): string | null {
    return path.startsWith('/') ? path.slice(1) : null;
}
```

Consumer:

```typescript
import { normalizeRelativePath } from '../shared/code_analysis/taxonomy_path_utility';
```

---

### Option B: Dependency Injection via Port/Protocol Interface

Use when the code needs:

- state,
- collaborators,
- side effects,
- infrastructure behavior,
- layer-specific implementation.

Example:

```typescript
// contract_output_path_builder_protocol.ts
export interface IOutputPathBuilderProtocol {
    buildFramePath(frame: Frame): FrameOutputPath;
}
```

```typescript
// capabilities_frame_exporter.ts
export class FrameExporter implements IFrameExporterProtocol {
    constructor(private readonly pathBuilder: IOutputPathBuilderProtocol) {}

    export(frame: Frame): FrameOutputPath {
        return this.pathBuilder.buildFramePath(frame);
    }
}
```

The capability depends only on the protocol, not on concrete infrastructure.

---

## Decision Tree

```text
Found reusable code in capabilities?
  │
  ├─ Does it know business/domain rules?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need this or class state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → move to infrastructure/port implementation
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `npx tsc --noEmit` or AST-based tooling.

```bash
# Check possible I/O in capabilities (AES404)
grep -n "fs\.\|readFile\|writeFile\|fetch\|axios\|sqlite3\|pg" packages/*/src/capabilities_*.ts

# Check forbidden imports
grep -n "^\s*from\s+.*infrastructure_\|^\s*from\s+.*agent_" packages/*/src/capabilities_*.ts

# List classes in capabilities files
grep -n "^export class " packages/*/src/capabilities_*.ts

# List protocol interface implementations
grep -n "implements I[A-Za-z0-9_]*Protocol" packages/*/src/capabilities_*.ts

# Find error swallowing patterns
grep -n "?? ''\|?? \"\"\||| 0\||| ''\||| \"\"" packages/*/src/capabilities_*.ts

# Find possible magic numbers
grep -n "[0-9]\+\.[0-9]\+" packages/*/src/capabilities_*.ts | grep -v "//\|const\|import" | head -20

# Check TypeScript
npx tsc --noEmit
```

---

### Check Wrong Block Order

```bash
awk '
    /^    (toString|toJSON|valueOf|equals)\(/ { if (!util_line) util_line = NR }
    /^    [a-z][a-zA-Z]*\(/ && !/^    (toString|toJSON|valueOf|equals|constructor)\(/ { if (!proto_line) proto_line = NR }
    END { if (util_line && proto_line && util_line < proto_line) print "VIOLATION: utility method (line " util_line ") before protocol method (line " proto_line ")" }
' packages/*/src/capabilities_*.ts
```

---

## Common Mistakes

- Putting I/O in capabilities.
- Defining domain data interfaces/types in capabilities files.
- Using concrete service types as constructor fields.
- Using raw primitives for domain value fields.
- Putting private helpers in the protocol interface.
- Putting constructors in the protocol interface.
- Placing utility methods before the domain protocol methods.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Extracting domain-specific single-consumer helpers to shared utility too early.
- Creating god interfaces with too many unrelated methods.
- Multiple implementation classes in one file.
- Direct dependency on concrete capabilities implementations.
- Silent error swallowing with `?? ''` or `|| 0`.
- Magic constants in capabilities logic.
