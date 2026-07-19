---
name: create-infrastructure-typescript
description: "Create and validate TypeScript infrastructure layer files following AES rules: I/O and external integration only, zero business logic, 3-block structure, one class per file, port interface contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
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
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create infrastructure typescript"
  - "add infrastructure typescript"
  - "fix infrastructure structure typescript"
  - "create port typescript"
  - "infrastructure missing port typescript"
  - "check infrastructure typescript"
  - "audit infrastructure typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-agent-typescript
  - enforce-1-class-per-file-typescript
  - trait-consolidation-typescript
  - module_logic_validator-typescript
  - fix-infrastructure-structure-typescript
  - create-missing-ports-typescript
---

# create-infrastructure-typescript

## Purpose

Create and validate TypeScript **infrastructure layer** files following clean architecture / AES rules.

An infrastructure file must contain **I/O and external system integration only**:

- file system access,
- network calls,
- database access,
- external API calls,
- environment/system integration,
- technical mapping,
- serialization/deserialization,
- error mapping,
- adapter implementation for port interfaces.

Infrastructure MUST NOT contain business logic.

---

## Definition of Done

An infrastructure file is considered valid when:

1. It contains exactly **ONE implementation class**.
2. The class implements exactly **ONE domain port interface**.
3. Block 2 contains **ONLY** the port interface method implementations.
4. Utility methods, static factories, and private helpers are placed in Block 3.
5. The file contains **zero business logic**.
6. The file does **not** define domain data structures locally.
7. Service dependencies use DI via port interfaces.
8. Value/configuration fields use shared VOs, not raw primitives.
9. Reusable, stateless, domain-agnostic functions are extracted to `*_utility.ts`.
10. Adapter-specific helpers may remain inside the implementation file.
11. I/O errors are propagated explicitly.
12. `npx tsc --noEmit` passes.

---

## Rules

### Layer Boundaries (AES)

#### Infrastructure Layer (`infrastructure_*.ts`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| File I/O (`fs.`, `readFile`, `writeFile`)           | Business rules                                        |
| Network calls (`fetch`, `axios`, `http`)            | Domain logic                                          |
| Database operations (`sqlite3`, `pg`)               | Domain calculations                                   |
| External API calls                                  | Domain validation that decides business correctness   |
| Environment/system access via controlled adapter    | Direct import from concrete `agent_*` modules         |
| Serialization/deserialization                       | Direct import from concrete `capabilities_*` modules  |
| Technical mapping (DTO ↔ VO)                        | Locally defined domain data structures                |
| Error mapping from external libraries               | Raw primitives for domain values in public contracts  |
| Port interface implementation                       | Silent error swallowing                               |
| Private helpers supporting the adapter              |                                                       |

Infrastructure may depend on:

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- port interfaces
- protocol interfaces defined in shared, when required by the adapter contract

Infrastructure must not depend on concrete capabilities or concrete agent implementations.

---

### Structural Rules

#### 1. One implementation class per file

Each infrastructure file contains exactly ONE main implementation class.

```typescript
export class FileSystemSourceReader {
    // ...
}
```

Do not define multiple service classes in the same file.

---

#### 2. Only the implementation class may be defined in the layer file

An infrastructure file may define the implementation class only.

Domain data structures MUST live in `shared/<domain>/taxonomy_*`.

Forbidden in infrastructure files:

```typescript
interface CacheEntry {
    key: string;
    value: string;
}
```

Allowed:

```typescript
import { CacheEntry } from '../shared/cache/taxonomy_cache_entry_vo';
```

---

#### 3. Service dependencies must use DI

Fields that represent collaborators, adapters, clients, repositories, or ports MUST use port interfaces.

```typescript
export class OrphanFileCache {
    constructor(private readonly store: IKeyValueStorePort) {}
}
```

Do not use concrete service types:

```typescript
export class OrphanFileCache {
    constructor(private readonly store: RedisKeyValueStore) {} // BAD: concrete dependency
}
```

---

#### 4. Value/configuration fields must use shared VOs

Fields that represent domain values, configuration, identifiers, paths, timeouts, thresholds, etc. should use shared VOs.

```typescript
export class HttpManifestClient {
    constructor(
        private readonly baseUrl: BaseUrl,
        private readonly timeout: TimeoutSeconds,
    ) {}
}
```

Avoid raw primitives for domain values:

```typescript
export class HttpManifestClient {
    constructor(
        private readonly baseUrl: string,  // BAD
        private readonly timeout: number,  // BAD
    ) {}
}
```

---

### Helper vs Utility Decision

The boundary is not only about `this`.

The real question is:

> Does this function know about adapter-specific or domain-specific rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this class, or by multiple modules?

---

### When to Keep as Private Helper (Block 3)

Keep the function inside the infrastructure file if ANY of these is true:

1. It accesses `this.field` or instance state.
2. It accesses adapter-specific static/state.
3. It performs adapter-specific mapping.
4. It maps external errors into port-specific errors.
5. It knows infrastructure-specific configuration.
6. It is tightly coupled to this adapter only.
7. It is a factory method such as `static create()` or `static from()`.
8. It is stateless but adapter-specific and only used by this class.

Example:

```typescript
class FileSystemSourceReader {
    private mapIoError(path: FilePath, err: Error): FileReadError {
        return FileReadError.io(path, err);
    }
}
```

This helper is infrastructure-specific and may remain in Block 3.

---

### When to Extract to Utility (`*_utility.ts`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `this`, no class field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business or adapter rules.
5. Reusable: useful for multiple infrastructure/capabilities/modules.

Example:

```typescript
// shared/common/taxonomy_string_utility.ts
export function normalizeWhitespace(input: string): string {
    return input.split(/\s+/).join(' ');
}
```

---

### I/O Blocker (CRITICAL)

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It belongs in infrastructure.

```typescript
function readFileContent(path: FilePath): Result<FileContent, FileReadError> {
    try {
        const raw = fs.readFileSync(path.value(), 'utf-8');
        return Ok(FileContent.new(raw));
    } catch (err) {
        return Err(FileReadError.io(path, err));
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
2. **Block 2 — Port Interface Method Implementation**
3. **Block 3 — Utility Methods, Factories, and Private Helpers**

---

### Block 1 — Class Definition & Constructor

```typescript
export class FileSystemSourceReader implements IFileReaderPort {
    constructor() {}
}
```

Or with dependencies:

```typescript
export class OrphanFileCache implements IOrphanFileCachePort {
    constructor(
        private readonly store: IKeyValueStorePort,
        private readonly policy: CachePolicy,
    ) {}
}
```

---

### Block 2 — Public Contract

Block 2 is RESERVED for the domain port interface methods ONLY.

```typescript
export class FileSystemSourceReader implements IFileReaderPort {
    read(path: FilePath): Result<FileContent, FileReadError> {
        // port implementation
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
static create(): FileSystemSourceReader
static from(...): FileSystemSourceReader
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
export class FileSystemSourceReader implements IFileReaderPort {
    toString(): string {
        return 'FileSystemSourceReader()';
    }

    equals(other: unknown): boolean {
        return other instanceof FileSystemSourceReader;
    }

    static create(): FileSystemSourceReader {
        return new FileSystemSourceReader();
    }

    private ensureParentDir(path: FilePath): Result<void, FileWriteError> {
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
import { normalizeRelativePath } from '../shared/common/taxonomy_path_utility';
```

But if the function is adapter-specific or infrastructure-specific, it may remain in Block 3.

---

## Method Placement Decision Rule

```text
Method / function found in an infrastructure file?
  │
  ├─ Module-level export function (outside class)?
  │   └─ YES → EXTRACT to *_utility.ts (ALWAYS forbidden in infrastructure)
  │
  ├─ Is it defined in the I<Name>Port interface?
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
import { FileContent } from '../shared/file_system/taxonomy_file_content_vo';
import { FilePath } from '../shared/file_system/taxonomy_file_path_vo';
import { FileReadError } from '../shared/file_system/taxonomy_file_read_error';
import { IFileReaderPort } from '../shared/file_system/contract_file_reader_port';


// ─── Block 1: Class Definition & Constructor ──────────────
export class FileSystemSourceReader implements IFileReaderPort {
    constructor() {}


    // ─── Block 2: Public Contract (domain port ONLY) ──────
    read(path: FilePath): Result<FileContent, FileReadError> {
        try {
            const raw = fs.readFileSync(path.value(), 'utf-8');
            return Ok(FileContent.new(raw));
        } catch (err) {
            return Err(FileReadError.io(path, err));
        }
    }


    // ─── Block 3: Utility Methods, Factories & Helpers ────
    toString(): string {
        return 'FileSystemSourceReader()';
    }

    equals(other: unknown): boolean {
        return other instanceof FileSystemSourceReader;
    }

    static create(): FileSystemSourceReader {
        return new FileSystemSourceReader();
    }

    private isNotFound(err: Error): boolean {
        return err instanceof FileNotFoundError;
    }
}
```

---

## Port Rules

### AES404 — Infrastructure Must Implement Port Interface

Every infrastructure class MUST implement a port interface.

```typescript
export class FileSystemSourceReader implements IFileReaderPort {
    // public contract
    ...
}
```

---

### Port file naming

| Layer            | File Pattern            | Port File                         | Port Name             |
| ---------------- | ----------------------- | --------------------------------- | --------------------- |
| Capabilities     | `capabilities_*.ts`   | `contract_<name>_protocol.ts`   | `I<Name>Protocol`   |
| Infrastructure   | `infrastructure_*.ts` | `contract_<name>_port.ts`       | `I<Name>Port`       |
| Agents           | `agent_*.ts`          | `contract_<name>_aggregate.ts`  | `I<Name>Aggregate`  |

---

### Port content rules

The port interface MUST contain only public contract methods.

Good:

```typescript
export interface IFileReaderPort {
    read(path: FilePath): Result<FileContent, FileReadError>;
}
```

Bad:

```typescript
export interface IFileReaderPort {
    read(path: FilePath): Result<FileContent, FileReadError>;

    privateHelper(): void; // BAD: helper in interface
}
```

---

### Constructors are not port methods

`constructor` and static factory methods MUST stay in Block 1 / Block 3.

Bad:

```typescript
export interface IFileReaderPort {
    create(): IFileReaderPort; // BAD
}
```

Good:

```typescript
export class FileSystemSourceReader implements IFileReaderPort {
    static create(): FileSystemSourceReader {
        return new FileSystemSourceReader();
    }
}
```

---

### Port methods should use shared VOs

Port contracts should avoid raw primitives for domain values.

Bad:

```typescript
export interface IFileReaderPort {
    read(path: string): string;
}
```

Good:

```typescript
export interface IFileReaderPort {
    read(path: FilePath): Result<FileContent, FileReadError>;
}
```

---

## The Fundamental Question

> **"Is this file pure I/O or external system integration?"**

If yes → **`infrastructure_*.ts` + implement port interface**

If no, and it contains business logic → **move to capabilities layer**

---

## Naming Convention

| Layer            | File Pattern            | Port File                         | Port Name             |
| ---------------- | ----------------------- | --------------------------------- | --------------------- |
| Capabilities     | `capabilities_*.ts`   | `contract_<name>_protocol.ts`   | `I<Name>Protocol`   |
| Infrastructure   | `infrastructure_*.ts` | `contract_<name>_port.ts`       | `I<Name>Port`       |
| Agents           | `agent_*.ts`          | `contract_<name>_aggregate.ts`  | `I<Name>Aggregate`  |

---

## Detection Patterns

### BAD: Infrastructure Without Port (AES404)

```typescript
export class FileCache {
    read(): string {
        // public behavior without port interface
        ...
    }
}
```

Fix:

```typescript
export class FileCache implements IFileCachePort {
    read(): string {
        // contract implementation
        ...
    }
}
```

---

### BAD: Business Logic in Infrastructure

```typescript
export class OrphanFileCache {
    analyze(content: FileContent): boolean {
        // BAD: domain logic
        return content.value.includes('orphan');
    }
}
```

Fix:

Move analysis to capabilities.

```typescript
// capabilities_orphan_analyzer.ts
export class OrphanAnalyzer implements IOrphanAnalyzerProtocol {
    analyze(content: FileContent): OrphanAnalysisResult {
        // domain logic here
        ...
    }
}
```

Infrastructure should only load/save/cache data.

---

### BAD: Interface Defined in Layer File

```typescript
interface CacheEntry {  // ← INTERFACE — should be in shared/taxonomy
    key: string;
    value: string;
}

class OrphanFileCache {
    entry: CacheEntry;  // ← concrete type, not DI
}
```

Fix:

Move to shared taxonomy:

```typescript
// shared/cache/taxonomy_cache_entry_vo.ts
export interface CacheEntry {
    readonly key: CacheKey;
    readonly value: CacheValue;
}
```

Then import it:

```typescript
import { CacheEntry } from '../shared/cache/taxonomy_cache_entry_vo';
```

---

### BAD: Concrete Service Field

```typescript
export class OrphanFileCache {
    constructor(private readonly store: RedisKeyValueStore) {} // BAD
}
```

Fix:

```typescript
export class OrphanFileCache {
    constructor(private readonly store: IKeyValueStorePort) {}
}
```

---

### BAD: Utility Methods in Block 2

```typescript
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

Fix:

```typescript
export class FileCacheAdapter implements IFileReaderPort {
    constructor(private readonly _cacheDir: FilePath) {}

    read(path: FilePath): string { ... }    // ← Block 2: port method

    toString(): string {                    // ← Block 3: utility method
        return 'FileCacheAdapter()';
    }

    equals(other: unknown): boolean {       // ← Block 3
        return other instanceof FileCacheAdapter;
    }
}
```

---

### GOOD: Implementor with Shared Data and DI

```typescript
import { CachePolicy } from '../shared/cache/taxonomy_cache_policy_vo';
import { IKeyValueStorePort } from '../shared/cache/contract_key_value_store_port';
import { IOrphanFileCachePort } from '../shared/orphan_detector/contract_orphan_file_cache_port';

export class OrphanFileCache implements IOrphanFileCachePort {
    constructor(
        private readonly store: IKeyValueStorePort,
        private readonly policy: CachePolicy,
    ) {}
}
```

---

### GOOD: Correct 3-Block with Utility Methods

```typescript
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

---

## Workflow

### Step 1: Analyze File Responsibility

Read the file and ask:

> Is this code pure I/O or external system integration?

If yes → keep as infrastructure.

If it contains business logic → move to capabilities.

Examples of business logic that must move out of infrastructure:

- deciding whether a file is orphan
- calculating domain severity
- validating business rules
- computing domain metrics
- interpreting domain meaning from content

Technical mapping is still allowed:

- DTO to VO mapping
- serialization
- deserialization
- external error mapping
- connection handling
- retry mechanics
- transport-level normalization

---

### Step 2: Check for Missing Port

Does the infrastructure class implement a port interface?

If no:

1. create `contract_<name>_port.ts`
2. define `I<Name>Port`
3. move public method signatures into the port
4. make the class implement the port

---

### Step 3: Create Port File if Missing

Create port file in the appropriate shared domain folder.

Examples:

| Package        | Port Path                                                |
| -------------- | -------------------------------------------------------- |
| import-rules   | `packages/shared/src/import_rules/contract_*_port.ts`   |
| code-analysis  | `packages/shared/src/code_analysis/contract_*_port.ts`  |
| orphan-detector| `packages/shared/src/orphan_detector/contract_*_port.ts`|

Register the module in the relevant `index.ts`.

---

### Step 4: Enforce 3-Block Structure

Reorganize the file into:

1. class definition + `constructor`
2. port interface method implementations
3. utility methods, static factories, private helpers

---

### Step 5: Verify Class Discipline

Check:

- exactly one implementation class
- no local domain data interfaces/types
- no local enums/VOs/DTOs/constants
- service fields use port interfaces
- value fields use shared VOs

---

### Step 6: Verify Helper vs Utility Boundary

For each helper/function:

```text
Does it know adapter-specific or infrastructure-specific details?
├─ YES → keep as helper in Block 3
└─ NO
   Is it stateless, pure, and reusable by multiple modules?
   ├─ YES → extract to *_utility.ts
   └─ NO → keep as helper in Block 3
```

---

### Step 7: Verify Layer Compliance

Ensure:

- no forbidden imports from concrete capabilities
- no forbidden imports from concrete agents
- no business logic
- no domain calculations
- no local domain data definitions

---

### Step 8: Verify Error Handling, VO, and Constants

Check:

- no silent `?? ''` or `|| 0` error swallowing
- fallible port methods return descriptive error types or throw meaningful errors
- I/O errors are propagated
- public contracts use shared VOs
- no magic constants for domain values

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
- [ ] Block 2 contains ONLY the port interface method implementations.
- [ ] Block 3 contains utility methods, factories, and private helpers.
- [ ] Infrastructure class implements a port interface (AES404).
- [ ] Port contains only public contract methods.
- [ ] Private helpers are not declared in the port.
- [ ] Constructors are not declared in the port.
- [ ] Utility methods are in Block 3.
- [ ] Adapter-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `*_utility.ts`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] One file contains exactly one implementation class.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use port interfaces via DI.
- [ ] Value/configuration fields use shared VOs.
- [ ] Infrastructure contains zero business logic.
- [ ] No forbidden imports from concrete `capabilities_*`.
- [ ] No forbidden imports from concrete `agent_*`.
- [ ] Port module is registered in the shared package's `index.ts`.
- [ ] `npx tsc --noEmit` passes.

---

## Error Handling Rules

Infrastructure error handling must be explicit.

### Rule 1: Do not silently discard errors

Forbidden:

```typescript
const content = fs.readFileSync(path.value(), 'utf-8') ?? '';
```

Forbidden:

```typescript
const value = result || 0;
```

Unless the value is genuinely optional and the default is an explicit domain/technical decision.

---

### Rule 2: Fallible port methods should return `Result` or throw

If a port method can fail due to I/O, network, database, parsing, or validation, return a result type or throw a meaningful error.

```typescript
read(path: FilePath): Result<FileContent, FileReadError>;
```

---

### Rule 3: Use descriptive error types

Prefer custom error types from shared taxonomy.

```typescript
export class FileReadError extends Error {
    constructor(
        public readonly path: FilePath,
        public readonly cause: Error,
    ) {
        super(`Failed to read ${path.value()}: ${cause.message}`);
    }
}
```

Avoid losing context:

```typescript
catch (e) {
    return String(e); // BAD: context lost
}
```

---

### Rule 4: Infrastructure should not produce lint results directly

Infrastructure should return data, errors, or VOs.

Lint violations are usually domain/analysis outcomes and belong to capabilities.

Bad:

```typescript
read(path: FilePath): LintResult[] {
    // BAD: infrastructure deciding lint outcomes
    ...
}
```

Good:

```typescript
read(path: FilePath): Result<FileContent, FileReadError> {
    // infrastructure returns data or error
    ...
}
```

Capabilities then decides whether an error becomes a lint violation.

---

### Proper Patterns

```typescript
// OK: explicit I/O error propagation
read(path: FilePath): Result<FileContent, FileReadError> {
    try {
        const raw = fs.readFileSync(path.value(), 'utf-8');
        return Ok(FileContent.new(raw));
    } catch (err) {
        return Err(FileReadError.io(path, err));
    }
}
```

```typescript
// OK: optional config with explicit default constant
timeout(): TimeoutSeconds {
    return this.config.timeout() ?? DEFAULT_TIMEOUT_SECONDS;
}
```

---

## Primitive and VO Rules

Infrastructure public contracts should use shared VOs for domain data.

Bad:

```typescript
export interface IFileWriterPort {
    write(path: string, content: string): void;
}
```

Good:

```typescript
export interface IFileWriterPort {
    write(path: FilePath, content: FileContent): Result<void, FileWriteError>;
}
```

### Primitive Policy

| Primitive  | Rule |
| ---------- | ---- |
| `string`   | Forbidden for domain fields and public contract values. Use VO. |
| `number`   | Forbidden for domain values. Use VO. |
| `boolean`  | Allowed for technical toggles when no richer VO is needed. |

Prefer VOs for:

- file paths
- URLs
- timeouts
- durations
- cache keys
- cache values
- query results
- identifiers
- messages

---

## Magic Constant Extraction Rules

No hardcoded domain literals in infrastructure.

Bad:

```typescript
save(): Result<void, FileWriteError> {
    fs.writeFileSync('manifest.json', data); // BAD: magic string
    return Ok(undefined);
}
```

Good:

```typescript
import { MANIFEST_FILENAME } from '../shared/manifest/taxonomy_manifest_constant';

save(): Result<void, FileWriteError> {
    fs.writeFileSync(MANIFEST_FILENAME.value, data);
    return Ok(undefined);
}
```

Constants MUST live in:

```text
taxonomy_*_constant.ts
```

Technical defaults should also be named constants or come from configuration VOs.

---

## Import Strategy

When fixing cross-import violations in infrastructure, choose one of these options.

---

### Option A: Extract to Taxonomy Utility

Use when the code is:

- stateless,
- pure,
- domain-agnostic,
- reusable by multiple modules.

Example:

```typescript
// shared/common/taxonomy_path_utility.ts
export function normalizeRelativePath(path: string): string | null {
    return path.startsWith('/') ? path.slice(1) : null;
}
```

Consumer:

```typescript
import { normalizeRelativePath } from '../shared/common/taxonomy_path_utility';
```

---

### Option B: Dependency Injection via Port Interface

Use when the code needs:

- state,
- collaborators,
- side effects,
- I/O,
- layer-specific implementation.

Example:

```typescript
// contract_file_writer_port.ts
export interface IFileWriterPort {
    write(path: FilePath, content: FileContent): Result<void, FileWriteError>;
}
```

```typescript
// infrastructure_file_writer_adapter.ts
export class FileWriterAdapter implements IFileWriterPort {
    write(path: FilePath, content: FileContent): Result<void, FileWriteError> {
        try {
            fs.writeFileSync(path.value(), content.value());
            return Ok(undefined);
        } catch (err) {
            return Err(FileWriteError.io(path, err));
        }
    }
}
```

```typescript
// consumer
export class ReportPublisher {
    constructor(private readonly writer: IFileWriterPort) {}
}
```

The consumer depends only on the port interface, not on concrete infrastructure.

---

## Decision Tree

```text
Found reusable code in infrastructure?
  │
  ├─ Does it know adapter-specific or infrastructure-specific details?
  │   └─ YES → keep as private helper in Block 3
  │
  ├─ Does it need this or class state?
  │   └─ YES → keep as helper/method in Block 3
  │
  ├─ Does it perform I/O or side effects?
  │   └─ YES → keep in infrastructure, not utility
  │
  └─ Is it stateless, pure, domain-agnostic, and reusable?
      └─ YES → extract to shared taxonomy utility
```

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `npx tsc --noEmit` or AST-based tooling.

```bash
# List classes in infrastructure files
grep -n "^export class " packages/*/src/infrastructure_*.ts

# List port interface implementations
grep -n "implements I[A-Za-z0-9_]*Port" packages/*/src/infrastructure_*.ts

# Check possible business logic keywords
grep -n "is_orphan\|analyze\|validate\|calculate\|compute\|business" packages/*/src/infrastructure_*.ts

# Check forbidden imports
grep -n "^\s*from\s+.*capabilities_\|^\s*from\s+.*agent_" packages/*/src/infrastructure_*.ts

# Find error swallowing patterns
grep -n "?? ''\|?? \"\"\||| 0\||| ''\||| \"\"" packages/*/src/infrastructure_*.ts

# Find possible magic numbers
grep -n "[0-9]\+\.[0-9]\+" packages/*/src/infrastructure_*.ts | grep -v "//\|const\|import" | head -20

# Check TypeScript
npx tsc --noEmit
```

---

### Check Wrong Block Order

```bash
awk '
    /^    (toString|toJSON|valueOf|equals)\(/ { if (!util_line) util_line = NR }
    /^    [a-z][a-zA-Z]*\(/ && !/^    (toString|toJSON|valueOf|equals|constructor)\(/ { if (!port_line) port_line = NR }
    END { if (util_line && port_line && util_line < port_line) print "VIOLATION: utility method (line " util_line ") before port method (line " port_line ")" }
' packages/*/src/infrastructure_*.ts
```

---

## Common Mistakes

- Putting business logic in infrastructure.
- Putting domain calculations in infrastructure.
- Putting domain validation in infrastructure.
- Defining domain data interfaces/types in infrastructure files.
- Using concrete service types as constructor fields.
- Using raw primitives for domain value fields.
- Exposing raw primitives in public port contracts when a VO exists.
- Putting private helpers in the port interface.
- Putting constructors in the port interface.
- Placing utility methods before the port interface methods.
- Mixing Block 2 and Block 3 responsibilities.
- Keeping reusable, domain-agnostic utility functions inside Block 3.
- Extracting adapter-specific helpers to shared utility too early.
- Creating god ports with too many unrelated methods.
- Multiple implementation classes in one file.
- Direct dependency on concrete capabilities implementations.
- Direct dependency on concrete agent implementations.
- Silent error swallowing with `?? ''` or `|| 0`.
- Magic constants in infrastructure logic.
- Infrastructure returning lint results directly instead of returning data/errors to capabilities.
