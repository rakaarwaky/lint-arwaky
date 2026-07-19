---
name: create-contract-typescript
description: "Create and validate contract layer files (contract_*.ts) — port, protocol, aggregate interfaces that decouple layers without implementing any logic."
version: 1.0.0
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

Create and validate TypeScript **contract layer** files in `packages/shared/src/<domain>/`. Contracts are pure interface definitions — they decouple layers by defining interfaces without implementing any logic. Three suffix types serve different roles: `_port` (infrastructure), `_protocol` (capabilities), `_aggregate` (agent).

## Rules

### The Fundamental Question

> **"Is this a pure interface definition or does it contain implementation?"**

- **Contract (interface only)** → **MUST be in shared/taxonomy as `contract_*.ts`**. No method bodies, no logic.
- **Class** (that implements protocol) → belongs in layer file (`capabilities_*.ts`, `infrastructure_*.ts`, `agent_*.ts`).

### Contract Layer Structure

```
packages/shared/src/<domain>/
├── index.ts                 # Module exports for this domain
├── contract_*_port.ts       # Outbound interfaces — implemented by Infrastructure
├── contract_*_protocol.ts   # Inbound interfaces — implemented by Capabilities
└── contract_*_aggregate.ts  # Composition facades — implemented by Agents
```

### Three Suffix Types and Their Roles

| Suffix       | Role               | Implemented By       | Example                                                                        |
| ------------ | ------------------ | -------------------- | ------------------------------------------------------------------------------ |
| `_port`      | Outbound interface | Infrastructure layer | `contract_system_port.ts`, `contract_import_parser_port.ts`                    |
| `_protocol`  | Inbound interface  | Capabilities layer   | `contract_import_forbidden_protocol.ts`, `contract_naming_checker_protocol.ts` |
| `_aggregate` | Composition facade | Agent layer          | `contract_import_runner_aggregate.ts`, `contract_tui_aggregate.ts`             |

**CRITICAL:** These suffixes are **strict** — only `_port`, `_protocol`, `_aggregate` are allowed for `contract_` prefixed files. No other suffixes.

### Naming Convention

`contract_<concept_word(s)>_<role_suffix>.ts`

| Concept                     | File Name                               | Interface Name           | Implemented By          |
| --------------------------- | --------------------------------------- | ------------------------ | ----------------------- |
| System operations           | `contract_system_port.ts`               | `IFileSystemPort`        | Infrastructure adapters |
| Forbidden import checking   | `contract_import_forbidden_protocol.ts` | `IImportForbiddenProtocol`| Capabilities checkers   |
| Import runner orchestration | `contract_import_runner_aggregate.ts`   | `IImportRunnerAggregate` | Agent orchestrators     |

### Import Restrictions (AES201)

Contract files must remain **completely pure**:

| Can Import From          | Cannot Import From                                           |
| ------------------------ | ------------------------------------------------------------ |
| `taxonomy_*` files       | capabilities, infrastructure, agents, surfaces               |
| Other `contract_*` files | Any layer files (*.ts without contract_ or taxonomy_ prefix) |

**Contracts define interfaces only — zero implementation logic.**

### Interface Structure

Every contract interface follows the protocol pattern:

```typescript
// contract_system_port.ts
export interface IFileSystemPort {
    /** Read file contents. */
    readFile(path: string): Promise<string>;

    /** Write content to file. */
    writeFile(path: string, content: string): Promise<void>;

    /** Glob files matching pattern. */
    globFiles(pattern: string, callback: (file: string) => void): number;
}
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

### Step 2: Identify Public Methods

List all methods that other layers need to call. These become interface method signatures.

```bash
# Find methods used across layers
grep -rn "function\|method\|=>" packages/*/src/ | grep -v "shared/" | head -50
```

### Step 3: Create Contract File

Create `contract_<concept>_<suffix>.ts` in the appropriate domain under `packages/shared/src/<domain>/`.

**Rules:**

- Interface must be exported with `export interface`
- Methods must have type annotations
- Use `;` (semicolon) for method signatures — no implementation logic
- Import only `taxonomy_*` and other `contract_*` files

```typescript
// contract_<name>_<suffix>.ts
import { FilePath } from '../common/taxonomy_path';

export interface I<Name><Suffix> {
    publicMethod(input: FilePath): string;
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

class FileAdapter implements IFileSystemPort {
    async readFile(path: FilePath): Promise<string> {
        return fs.promises.readFile(path.value(), 'utf-8');
    }

    async writeFile(path: FilePath, content: string): Promise<void> {
        await fs.promises.writeFile(path.value(), content, 'utf-8');
    }
}
```

### Step 6: Verify

Run TypeScript compiler to confirm no violations.

## Verification Checklist

- [ ] Contract file uses correct suffix (`_port`, `_protocol`, `_aggregate`).
- [ ] Contract contains **only interface definitions** — no method bodies, no implementation logic.
- [ ] Interface is exported with `export interface`.
- [ ] Methods have type annotations.
- [ ] Contract imports only `taxonomy_*` and other `contract_*` files.
- [ ] No capabilities, infrastructure, agents, or surface imports in contract files.
- [ ] Domain's `index.ts` exports new contract module — `export { ... } from './contract_<name>_<suffix>'`.
- [ ] Layer file implements the interface (infrastructure for _port, capabilities for _protocol, agent for _aggregate).
- [ ] `tsc --noEmit` passes without errors.

## Quick Commands

```bash
# Find contracts without implementations
grep -rn "^export interface " packages/shared/src/*/contract_*.ts | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    interface=$(echo "$line" | grep -oP 'export interface \K[a-zA-Z_]+')
    grep -q "implements.*$interface" packages/*/src/*.ts || echo "UNIMPLEMENTED: $interface in $file"
done

# Check for forbidden imports in contract files
grep -n "from.*capabilities_\|from.*infrastructure_\|from.*agent_" packages/shared/src/*/contract_*.ts

# Find contracts that don't use export interface
grep -rn "^interface " packages/shared/src/*/contract_*.ts | grep -v "export"

# Verify contract module exports are registered
grep -n "^export.*from.*contract_" packages/shared/src/*/index.ts

# Check for unregistered contract files (exist on disk but not in index.ts)
find packages/shared/src/<domain>/ -name "contract_*.ts" | while read f; do
    name=$(basename "$f" .ts)
    grep -q "from.*$name" packages/shared/src/<domain>/index.ts || echo "UNREGISTERED: $name"
done

# Check TypeScript
npx tsc --noEmit
```

## Common Mistakes (AVOID)

- ❌ **Putting implementation logic in contract files**: Contracts must contain ONLY interface definitions. Implementors belong in layer files.
- ❌ **Importing non-taxonomy types into contracts**: Contracts can only import `taxonomy_*` and other `contract_*` files.
- ❌ **Using wrong suffix for contract files**: Only `_port`, `_protocol`, `_aggregate` are allowed. No other suffixes.
- ❌ **Forgetting to register new contract modules in index.ts**: Every `contract_*.ts` file must have a corresponding `export { ... } from './contract_<name>_<suffix>'` in the domain's `index.ts`.
- ❌ **Missing type annotations on methods**: All contract methods MUST have proper TypeScript type annotations.
- ❌ **Placing method bodies in contract files**: Even thin wrapper methods belong in layer files, not contracts.
- ❌ **Duplicating contract definitions across domains**: If a contract belongs to multiple domains, put it in `common/` and import from there.
