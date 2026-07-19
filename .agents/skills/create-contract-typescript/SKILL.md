---
name: create-contract-typescript
description: "Create and validate contract layer files (contract_*.ts) — pure interface definitions that decouple layers without leaking implementation details."
version: 1.1.0
category: refactoring
tags:
  [
    typescript,
    aes,
    contract,
    protocol,
    port,
    aggregate,
    interface,
    shared,
    structure,
  ]
triggers:
  - "create contract typescript"
  - "add contract typescript"
  - "create protocol typescript"
  - "create port typescript"
  - "create aggregate typescript"
  - "missing contract typescript"
  - "fix god interface typescript"
dependencies: []
related:
  - create-capabilities-typescript
  - create-infrastructure-typescript
  - create-agent-typescript
  - create-taxonomy-typescript
  - interface-consolidation-typescript
  - enforce-1-class-per-file-typescript
  - create-missing-protocols-typescript
---

# create-contract-typescript

## Purpose

Create and validate TypeScript **contract layer** files in `packages/shared/src/<domain>/`. Contracts are **pure interface definitions** — they decouple layers by defining the "WHAT" (public promise) without implementing any "HOW" (logic) or leaking internal stepping stones. 

Three suffix types serve different roles: `_port` (infrastructure), `_protocol` (capabilities), `_aggregate` (agent).

## Rules

### The Fundamental Question (The Golden Rule)

> **"Is this a public promise needed by an outer layer, or just an internal stepping stone?"**
> 
> - **Public Promise (WHAT)**: Outer layers need to call this, or it requires polymorphism (multiple implementations). → **Put in Contract (`contract_*.ts`)**.
> - **Internal Stepping Stone (HOW)**: Helper methods, highly specific algorithms (e.g., specific regex), or logic that only serves other methods in the same class. → **Keep as `private` method in Implementation Class**. **NEVER put this in the contract.**

### Contract Layer Structure

```text
packages/shared/src/<domain>/
├── index.ts                    # Module exports for this domain
├── contract_*_port.ts          # Outbound interfaces — implemented by Infrastructure
├── contract_*_protocol.ts      # Inbound interfaces — implemented by Capabilities
└── contract_*_aggregate.ts     # Composition facades — implemented by Agents
```

**CRITICAL:** These suffixes are **strict** — only `_port`, `_protocol`, `_aggregate` are allowed for `contract_` prefixed files. No other suffixes.

### Three Suffix Types and Their Roles

| Suffix       | Role               | Implemented By       | Example                                                                        |
| ------------ | ------------------ | -------------------- | ------------------------------------------------------------------------------ |
| `_port`      | Outbound interface | Infrastructure layer | `contract_system_port.ts`, `contract_import_parser_port.ts`                    |
| `_protocol`  | Inbound interface  | Capabilities layer   | `contract_import_forbidden_protocol.ts`, `contract_naming_checker_protocol.ts` |
| `_aggregate` | Composition facade | Agent layer          | `contract_import_runner_aggregate.ts`, `contract_tui_aggregate.ts`             |

### Naming Convention

Pattern: `contract_<concept_word(s)>_<role_suffix>.ts`

| Concept                     | File Name                               | Interface Name           | Implemented By          |
| --------------------------- | --------------------------------------- | ------------------------ | ----------------------- |
| System operations           | `contract_system_port.ts`               | `IFileSystemPort`        | Infrastructure adapters |
| Forbidden import checking   | `contract_import_forbidden_protocol.ts` | `IImportForbiddenProtocol`| Capabilities checkers   |
| Import runner orchestration | `contract_import_runner_aggregate.ts`   | `IImportRunnerAggregate` | Agent orchestrators     |

### Import Restrictions (AES201)

Contract files must remain **completely pure**:

| Can Import From          | Cannot Import From                                           |
| ------------------------ | ------------------------------------------------------------ |
| `taxonomy_*` files       | `capabilities_*`, `infrastructure_*`, `agent_*`, `surface_*` |
| Other `contract_*` files | Any layer files (`*.ts` without `contract_` or `taxonomy_` prefix) |

**Contracts define interfaces only — zero implementation logic.**

### Interface Structure Rules

Every contract interface must follow these structural rules:

1. **Export**: Must be exported with `export interface`.
2. **Signatures**: Methods must have proper TypeScript type annotations and end with `;` (semicolon).
3. **No Bodies**: Absolutely no implementation logic or method bodies (no `{ ... }`).
4. **No Helpers**: Do NOT include private helper signatures (e.g., `extractSpecificRegex`) or highly specific algorithmic steps in the interface.
5. **No Primitives**: ALL primitive types are FORBIDDEN in contract method signatures:
   - `string` → use domain-specific VO (e.g., `FilePath`, `SymbolName`)
   - `number` → use domain-specific VO (e.g., `LineNumber`, `Count`)
   - `boolean` → use `BooleanVO`
   - `string[]` → use domain-specific list VO (e.g., `PatternList`)
   - `Record<string, T>` → use domain-specific VO

```typescript
// contract_system_port.ts — Complete interface structure example
import { FilePath } from '../common/taxonomy_path';

export interface IFileSystemPort {
    /** Read file contents. */
    readFile(path: FilePath): Promise<ContentString>;

    /** Write content to file. */
    writeFile(path: FilePath, content: ContentString): Promise<void>;

    /** Glob files matching pattern. */
    globFiles(pattern: PatternList, callback: (file: FilePath) => void): Count;
}

// NOTE: Implementation belongs in infrastructure_adapter.ts — NOT here.
```

## Detection Patterns

### BAD: Contract Contains Implementation

```typescript
// BAD: Contract file contains method bodies with logic
export interface IFileSystemPort {
    readFile(path: string): string {
        // ← IMPLEMENTATION belongs in infrastructure_*.ts
        return fs.readFileSync(path).toString();  // ← I/O in contract!
    }
}
```

### BAD: Contract Imports Non-Taxonomy Types

```typescript
// BAD: Contract imports capability types
import { MyChecker } from '../capabilities/my_checker';  // ← FORBIDDEN

export interface IMyProtocol {
    check(checker: MyChecker): void;  // ← Should use taxonomy types only
}
```

### BAD: Leaking Implementation Details (God Interface)

```typescript
// BAD: Contract contains highly specific helper methods that force all implementors to write boilerplate
export interface IFileParserPort {
    parseFile(path: FilePath): Promise<ParsedData>;
    
    // BAD: LEAKING IMPLEMENTATION DETAIL. 
    // A Python parser doesn't need Rust regex. This belongs in the Rust parser class as a private method.
    extractRustSpecificRegex(content: string): string[]; 
}
```

### GOOD: Pure Protocol Interface

```typescript
// contract_system_port.ts — pure interface definition
import { FilePath } from '../common/taxonomy_path';

export interface IFileSystemPort {
    readFile(path: FilePath): Promise<string>;
    writeFile(path: FilePath, content: string): Promise<void>;
}

// Implementation belongs in infrastructure_adapter.ts — NOT here
```

## Workflow

### Step 1: Determine the Contract Role
Ask: **"Which layer will implement this interface?"**
- Infrastructure implements → `_port` (outbound)
- Capabilities implements → `_protocol` (inbound)
- Agent implements → `_aggregate` (composition facade)

### Step 2: Identify Public Methods (The Filter)
List all methods. Apply the Golden Rule:
- Does an outer layer call this? → **Keep in Contract**.
- Is it just a stepping stone / internal helper? → **Discard from Contract** (it will be a `private` method in the impl class).

### Step 3: Create Contract File
Create `contract_<concept>_<suffix>.ts` in the appropriate domain under `packages/shared/src/<domain>/`.
- Use `export interface`.
- Add proper type annotations.
- Use `;` for method signatures (no bodies).
- Import **only** `taxonomy_*` and other `contract_*` files.

```typescript
// contract_<name>_<suffix>.ts
import { FilePath } from '../common/taxonomy_path';

export interface I<Name><Suffix> {
    /** Public method description. */
    publicMethod(input: FilePath): string;
    
    /** Async method description. */
    asyncMethod(id: number): Promise<void>;
}
```

### Step 4: Register Module
Update the domain's `index.ts` to export the new contract module:
```typescript
// shared/src/<domain>/index.ts
export { I<Name><Suffix> } from './contract_<name>_<suffix>';  // ← Add this line
export { SomeVO } from './taxonomy_<name>_vo';
```

### Step 5: Implement in Layer File
The implementing layer file imports and implements the interface:
```typescript
// Infrastructure layer implements _port
import { IFileSystemPort } from '../shared/domain/contract_system_port';
import { FilePath } from '../common/taxonomy_path';
import * as fs from 'fs/promises';

export class FileAdapter implements IFileSystemPort {
    async readFile(path: FilePath): Promise<string> {
        return fs.readFile(path.value(), 'utf-8');
    }

    async writeFile(path: FilePath, content: string): Promise<void> {
        await fs.writeFile(path.value(), content, 'utf-8');
    }
    
    // Private helpers stay in the class, NOT in the interface above.
    private sanitizePath(path: string): string {
        return path.trim();
    }
}
```

### Step 6: Verify
Run TypeScript compiler to confirm no violations.

## Verification Checklist

- [ ] Contract file uses correct suffix (`_port`, `_protocol`, `_aggregate`).
- [ ] Contract contains **only interface definitions** — no method bodies, no implementation logic.
- [ ] **No leaking implementation details**: Contract does not contain highly specific helper methods (e.g., specific regex, internal parsing steps) that belong in the impl class as `private` methods.
- [ ] Interface is exported with `export interface`.
- [ ] Methods have proper TypeScript type annotations.
- [ ] Contract imports **only** `taxonomy_*` and other `contract_*` files.
- [ ] No `capabilities_*`, `infrastructure_*`, `agent_*`, or `surface_*` imports in contract files.
- [ ] Domain's `index.ts` exports new contract module (`export { ... } from './contract_<name>_<suffix>'`).
- [ ] Layer file correctly implements the interface.
- [ ] `npx tsc --noEmit` passes without errors.

## Quick Commands

```bash
# 1. Find contracts without implementations
grep -rn "^export interface " packages/shared/src/*/contract_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    interface=$(echo "$line" | grep -oP 'export interface \K[a-zA-Z_]+')
    grep -q "implements.*$interface" packages/*/src/*.ts || echo "UNIMPLEMENTED: $interface in $file"
done

# 2. Check for forbidden imports in contract files
grep -rn "from.*capabilities_\|from.*infrastructure_\|from.*agent_\|from.*surface_" packages/shared/src/*/contract_*.ts

# 3. Find contracts that don't use export interface
grep -rn "^interface " packages/shared/src/*/contract_*.ts | grep -v "export"

# 4. Detect potential "God Interfaces" (Interfaces with > 10 methods — likely leaking helpers)
awk '/^export interface/ {iface=$0; count=0} /^\s+[a-zA-Z_]+\(/ {count++} /^}/ {if(count > 10) print "WARNING: Potential God Interface?", iface, "has", count, "methods"}' packages/shared/src/*/contract_*.ts

# 5. Verify contract module exports are registered
find packages/shared/src/<domain>/ -name "contract_*.ts" | while read f; do
    name=$(basename "$f" .ts)
    grep -q "from.*'$name'" packages/shared/src/<domain>/index.ts || echo "UNREGISTERED: $name in index.ts"
done

# 6. Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Putting implementation logic in contract files**: Contracts must contain ONLY interface definitions. Implementors belong in layer files.
- ❌ **Importing non-taxonomy types into contracts**: Contracts can only import `taxonomy_*` and other `contract_*` files.
- ❌ **Using wrong suffix for contract files**: Only `_port`, `_protocol`, `_aggregate` are allowed. No other suffixes.
- ❌ **Leaking implementation details (God Interface)**: Do not put private helpers, specific regex logic, or internal stepping stones in the contract. They belong in the implementation class as `private` methods.
- ❌ **Forgetting to register new contract modules in `index.ts`**: Every `contract_*.ts` file must have a corresponding `export { ... } from './contract_<name>_<suffix>'` in the domain's `index.ts`.
- ❌ **Missing type annotations on methods**: All contract methods MUST have proper TypeScript type annotations.
- ❌ **Duplicating contract definitions across domains**: If a contract belongs to multiple domains, put it in `common/` and import from there.
```

---