# AES Migration Guide — TypeScript (v1.1.0)

> Skill-driven migration workflow for TypeScript/JavaScript projects to AES
> architecture. Each phase delegates to a dedicated skill in `.agents/skills/`.

See [ARCHITECTURE.md](ARCHITECTURE.md) for layer rules and
[README.md](README.md) for project usage.

---

## Table of Contents

- [AES Dependency Model](#aes-dependency-model)
- [Workspace Structure](#workspace-structure)
- [Prerequisites](#prerequisites)
- [Phase 0: Audit & Config Setup](#phase-0-audit--config-setup)
- [Phase 1: Taxonomy Layer](#phase-1-taxonomy-layer)
- [Phase 2: Contract Layer](#phase-2-contract-layer)
- [Phase 3: Utility Layer](#phase-3-utility-layer)
- [Phase 4: Capabilities Layer](#phase-4-capabilities-layer)
- [Phase 5: Agent Layer](#phase-5-agent-layer)
- [Phase 6: Surface Layer](#phase-6-surface-layer)
- [Phase 7: Root Layer](#phase-7-root-layer)
- [Phase 8: Verify & CI Gate](#phase-8-verify--ci-gate)
- [Import Rules Quick Reference](#import-rules-quick-reference)
- [Supplementary Skills](#supplementary-skills-post-migration)
- [File Naming Reference](#file-naming-reference)
- [Troubleshooting](#troubleshooting)

---

## AES Dependency Model

AES uses **dependency injection** as the inter-layer wiring mechanism.
Layers do not import each other directly — they import from **contract**
and receive dependencies via constructor injection:

```
                    ┌──────────────────────────────────┐
                    │             root                  │
                    │  (DI wiring — wires everything)   │
                    └──────┬───────────────────────────┘
                           │
              ┌────────────┼─────────────┐
              ▼            ▼             ▼
         ┌────────┐  ┌─────────┐  ┌──────────────┐
         │surface │  │  agent  │  │ capabilities │
         └───┬────┘  └────┬────┘  └──────┬───────┘
             │            │              │
             ▼            ▼              ▼
        ┌──────────────────────────────────────────┐
        │      contract (protocol / aggregate)      │
        └──────────────────┬───────────────────────┘
                           ▼
                  ┌──────────────────┐
                  │    taxonomy       │
                  └──────────────────┘

         utility ←── flexible, imports taxonomy only
```

**Key principles:**

- Agent does **not** import capabilities — it receives them via constructor injection.
- Surface does **not** import agent — it receives the orchestrator via constructor injection.
- Capabilities **implements** protocol interfaces. Agent **implements** aggregate interfaces.
- Utility is flexible — imports taxonomy only, imported by capabilities/agent/surface.
- TypeScript DI pattern: pass interface-typed instances via `constructor` parameters.
- All import rules are enforced by `lint-arwaky-cli` (AES201–AES205).

---

## Workspace Structure

```
project-root/
├── package.json             ← workspace root config
├── pnpm-workspace.yaml      ← pnpm workspace definition (if using pnpm)
├── tsconfig.json            ← TypeScript config
├── lint_arwaky.config.yaml  ← AES config (created in Phase 0)
├── packages/
│   ├── shared/              ← shared taxonomy + contract + utility types
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── src/
│   │       ├── index.ts
│   │       ├── common/          ← truly shared across ALL features
│   │       │   └── index.ts
│   │       └── <feature>/       ← shared types per feature domain
│   │           ├── index.ts
│   │           ├── taxonomy_<concept>_vo.ts
│   │           ├── taxonomy_<concept>_error.ts
│   │           ├── taxonomy_<concept>_constant.ts
│   │           ├── contract_<concept>_protocol.ts
│   │           ├── contract_<concept>_aggregate.ts
│   │           └── utility_<concept>_<role>.ts
│   │
│   ├── <feature>/           ← feature package
│   │   ├── package.json
│   │   ├── tsconfig.json
│   │   └── src/
│   │       ├── index.ts
│   │       ├── capabilities_<concept>_<role>.ts
│   │       ├── agent_<concept>_orchestrator.ts
│   │       ├── surface_<concept>_<role>.ts
│   │       └── root_<concept>_container.ts
│   │
│   └── root_<name>_entry.ts   ← entry point (file inside packages/)
│
└── tests/
```

**Key rules:**

- All 7 layers coexist in each feature slice.
- Taxonomy, contracts, and utilities live under `packages/shared/src/<feature>/`.
- Capabilities, agent, surface, and root live in the feature package.
- Entry points (`root_*_entry.ts`) live directly under `packages/` (file, NOT directory).
- `packages/shared/src/common/` holds types shared across ALL features.
- Every package directory should have `index.ts` (barrel file — skipped by lint).

---

## Prerequisites

```bash
# Install lint-arwaky
npm install -g lint-arwaky-cli

# Verify installation
lint-arwaky-cli version
# Expected: Lint Arwaky v1.1.0

# Install external linters (optional, for external lint checks)
npm install -D eslint prettier typescript
lint-arwaky-cli install
```

---

## Phase 0: Audit & Config Setup

> **Skill:** `lint-arwaky-typescript` — load for audit commands and violation analysis.

### Step 1: Initialize Config

```bash
cd your-project/
lint-arwaky-cli init
```

This creates `lint_arwaky.config.yaml` with default AES rules.

### Step 2: Run Initial Audit

```bash
lint-arwaky-cli scan .
```

### Step 3: Assess Migration Scope


| Violations | Strategy                                                    |
| ------------ | ------------------------------------------------------------- |
| < 10       | Full migration in one session                               |
| 10–50     | Phased migration (Phase 1 → 8)                             |
| > 50       | Start with taxonomy only (Phase 1), re-audit, then continue |

### Step 4: Count Files

```bash
find packages -name "*.ts" -not -name "*.d.ts" -not -path "*/node_modules/*" | wc -l
```

---

## Phase 1: Taxonomy Layer

> **Skill:** `create-taxonomy-typescript` — load for VOs, errors, constants, entities, events.

Define Value Objects, Errors, Events, and Constants under
`packages/shared/src/<feature>/`.

### Steps

1. Identify domain types:
   ```bash
   grep -rn "^export interface\|^export type\|^export enum\|^export class" packages/*/src/
   ```
2. Load `create-taxonomy-typescript` skill.
3. Create taxonomy files following skill templates.
4. Register in domain `index.ts`.
5. Verify: `npx tsc --noEmit`.

### Example

```typescript
// packages/shared/src/user/taxonomy_user_vo.ts

/** User identifier value object. */
export class UserId {
  constructor(readonly value: string) {
    if (!value) throw new Error("UserId cannot be empty");
  }

  equals(other: UserId): boolean {
    return this.value === other.value;
  }
}

/** Email value object with validation. */
export class Email {
  constructor(readonly value: string) {
    if (!value.includes("@")) throw new Error(`Invalid email: ${value}`);
  }

  normalized(): Email {
    return new Email(this.value.toLowerCase());
  }
}

/** User entity. */
export interface User {
  readonly id: UserId;
  readonly email: Email;
  readonly name: string;
}
```

```typescript
// packages/shared/src/user/taxonomy_user_error.ts

/** Base error for user domain. */
export class UserError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "UserError";
  }
}

/** Raised when a user is not found. */
export class UserNotFoundError extends UserError {
  constructor(userId: UserId) {
    super(`User not found: ${userId.value}`);
    this.name = "UserNotFoundError";
  }
}
```

```typescript
// packages/shared/src/user/taxonomy_user_constant.ts

export const MAX_USERNAME_LENGTH = 128;
export const MIN_PASSWORD_LENGTH = 8;
export const DEFAULT_PAGE_SIZE = 50;
```

### Rules Enforced

- **AES101**: Filename must be `taxonomy_<concept>_<suffix>.ts` (snake_case, 3+ words).
- **AES102**: Suffix must be `vo`, `entity`, `error`, `event`, or `constant`.
- **AES401**: No raw primitives (`string`, `number`, `boolean`, `any`) in type annotations — wrap in VOs.
- **AES401**: `_constant` files must contain only `export const` — no `class`, `interface`, `function`, `type`.

---

## Phase 2: Contract Layer

> **Skill:** `create-contract-typescript` — load for protocol and aggregate interfaces.

Contracts define public interfaces (Protocols and Aggregates) using
TypeScript `interface` without exposing implementation.

### Steps

1. Load `create-contract-typescript` skill.
2. Create protocol interfaces (inbound/outbound) under `packages/shared/src/<feature>/`.
3. Create aggregate facade interfaces under `packages/shared/src/<feature>/`.
4. Register in domain `index.ts`.
5. Verify: `npx tsc --noEmit`.

### Example

```typescript
// packages/shared/src/user/contract_user_protocol.ts

import { UserId, Email, User } from "./taxonomy_user_vo";

/**
 * Protocol for user repository operations.
 * Implemented by capabilities layer.
 */
export interface IUserRepositoryProtocol {
  findById(userId: UserId): Promise<User | null>;
  findByEmail(email: Email): Promise<User | null>;
  save(user: User): Promise<void>;
}
```

```typescript
// packages/shared/src/user/contract_user_aggregate.ts

import { UserId } from "./taxonomy_user_vo";
import { UserResponse } from "./taxonomy_user_vo";

/**
 * Aggregate facade for user operations.
 * Implemented by agent layer.
 */
export interface IUserAggregate {
  getUser(userId: UserId): Promise<UserResponse>;
  registerUser(command: RegisterCommand): Promise<UserResponse>;
}
```

### Rules Enforced

- **AES102**: Suffix must be `protocol` or `aggregate`.
- **AES402**: No raw primitives in method signatures — use VOs.
- **AES201**: Protocol must not import aggregate. Aggregate may import protocol.

---

## Phase 3: Utility Layer

> **Skill:** `create-utility-typescript` — load for stateless standalone functions.

Utility contains low-level technical mechanics — **stateless standalone
functions only**. No classes, no interfaces, no enums, no type aliases.

### Steps

1. Identify reusable stateless functions across packages.
2. Load `create-utility-typescript` skill.
3. Create utility files under `packages/shared/src/<feature>/`.
4. Register in domain `index.ts`.
5. Verify: `npx tsc --noEmit`.

### Example

```typescript
// packages/shared/src/user/utility_user_validator.ts

import { Email } from "./taxonomy_user_vo";

const EMAIL_PATTERN = /^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;

/** Validate email format. Stateless — no class, no state. */
export function validateEmail(email: Email): boolean {
  return EMAIL_PATTERN.test(email.value);
}

/** Normalize email to lowercase. */
export function normalizeEmail(email: Email): Email {
  return new Email(email.value.toLowerCase());
}

/** Generate a UUID-based user identifier. */
export function generateUserId(): string {
  return crypto.randomUUID();
}
```

### Rules Enforced

- **AES102**: Suffix is flexible, but forbidden suffixes apply (`vo`, `entity`, `protocol`, `aggregate`, etc.).
- **AES404**: No `export class`, `export interface`, `export enum`, `export type`. Only `export function` and `export const`.
- **AES201**: Utility may import taxonomy only. Must not import contract, capabilities, agent, surface.

---

## Phase 4: Capabilities Layer

> **Skill:** `create-capabilities-typescript` — load for business logic and external adaptation.

Capabilities contain concrete behavior implementations. They **implement
protocol interfaces** defined in the contract layer via `implements`.

### Steps

1. Load `create-capabilities-typescript` skill.
2. Create business logic capabilities (implement protocol interfaces).
3. Create external adaptation capabilities (repositories, clients).
4. Verify: `npx tsc --noEmit`.

### Example

```typescript
// packages/user/src/capabilities_user_repository.ts

import { IUserRepositoryProtocol } from "@shared/user/contract_user_protocol";
import { UserId, Email, User } from "@shared/user/taxonomy_user_vo";

/** Concrete user repository backed by database. */
export class UserRepository implements IUserRepositoryProtocol {
  constructor(private readonly db: DatabaseConnection) {}

  async findById(userId: UserId): Promise<User | null> {
    const row = await this.db.query(
      "SELECT * FROM users WHERE id = $1",
      [userId.value],
    );
    return row ? User.fromRow(row) : null;
  }

  async findByEmail(email: Email): Promise<User | null> {
    const row = await this.db.query(
      "SELECT * FROM users WHERE email = $1",
      [email.value],
    );
    return row ? User.fromRow(row) : null;
  }

  async save(user: User): Promise<void> {
    await this.db.execute(
      "INSERT INTO users (id, email, name) VALUES ($1, $2, $3)",
      [user.id.value, user.email.value, user.name],
    );
  }
}
```

### Rules Enforced

- **AES102**: Suffix is flexible (forbidden: `vo`, `entity`, `protocol`, `aggregate`, `utility`).
- **AES201**: Capabilities may import taxonomy, contract, utility. Must not import agent, surface, other capabilities.
- **AES202**: Must import taxonomy and contract(protocol).
- **AES403**: At least 1 class must implement a protocol interface. Max 3 type declarations per file.
- **AES201 purpose**: contract(protocol) imports must be used for `implements` (implement), not just function calls.

---

## Phase 5: Agent Layer

> **Skill:** `create-agent-typescript` — load for orchestration logic.

Orchestrates sequential execution, branching, looping, and error handling.
**Implements aggregate interfaces** defined in the contract layer.

### Steps

1. Load `create-agent-typescript` skill.
2. Create orchestrator class implementing aggregate interface.
3. Inject protocol dependencies via constructor.
4. Verify: `npx tsc --noEmit`.

### Example

```typescript
// packages/user/src/agent_user_orchestrator.ts

import { IUserAggregate } from "@shared/user/contract_user_aggregate";
import { IUserRepositoryProtocol } from "@shared/user/contract_user_protocol";
import { UserId, UserResponse } from "@shared/user/taxonomy_user_vo";
import { UserNotFoundError } from "@shared/user/taxonomy_user_error";

/** Orchestrates user operations via injected repository. */
export class UserOrchestrator implements IUserAggregate {
  constructor(
    private readonly repository: IUserRepositoryProtocol,
  ) {}

  async getUser(userId: UserId): Promise<UserResponse> {
    const user = await this.repository.findById(userId);
    if (!user) throw new UserNotFoundError(userId);
    return UserResponse.fromUser(user);
  }

  async registerUser(command: RegisterCommand): Promise<UserResponse> {
    const existing = await this.repository.findByEmail(command.email);
    if (existing) throw new UserAlreadyExistsError(command.email);
    const user = User.create(command);
    await this.repository.save(user);
    return UserResponse.fromUser(user);
  }
}
```

### Rules Enforced

- **AES102**: Suffix must be `orchestrator`.
- **AES201**: Agent may import taxonomy, contract(aggregate), contract(protocol), utility. Must not import capabilities, surface.
- **AES202**: Must import taxonomy and contract(aggregate).
- **AES405**: At least 1 class must implement an aggregate interface. Max 3 type declarations.
- **AES201 purpose**: contract(aggregate) imports must be used for `implements` (implement).

---

## Phase 6: Surface Layer

> **Skill:** `create-surface-typescript` — load for user-facing input translation.

Translates user-facing inputs into actions, delegating to the Agent
orchestrator via aggregate interface.

### Surface Classification


| Category    | Suffixes                                      | Rules                                                                                                 |
| ------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **Smart**   | `_command`, `_controller`, `_page`, `_router` | May contain orchestration logic. Global limit: 15 functions.                                          |
| **Utility** | `_hook`, `_store`, `_action`, `_screen`       | Supports smart surfaces. Max 10 methods, 80 lines/method, 3 nesting depth, 3 control-flow statements. |
| **Passive** | `_component`, `_view`, `_layout`, others      | Presentation only. Same limits as Utility.                                                            |

### Steps

1. Load `create-surface-typescript` skill.
2. Create surface classes (commands, handlers, endpoints).
3. Inject aggregate interface via constructor.
4. Verify: `npx tsc --noEmit`.

### Example

```typescript
// packages/user/src/surface_user_command.ts

import { IUserAggregate } from "@shared/user/contract_user_aggregate";
import { UserId, UserResponse } from "@shared/user/taxonomy_user_vo";

/** Command to retrieve a user by ID. */
export class GetUserCommand {
  constructor(
    private readonly aggregate: IUserAggregate,
  ) {}

  async execute(userId: UserId): Promise<UserResponse> {
    return this.aggregate.getUser(userId);
  }
}
```

### Rules Enforced

- **AES102**: Suffix must be in the surface allow-list.
- **AES201**: Surface(command) may import taxonomy, contract(aggregate), utility. Must not import agent, capabilities, contract(protocol).
- **AES406**: Function count, method count, method length, nesting depth, and control-flow limits apply per surface category.
- **AES201 purpose**: contract(aggregate) imports must be used for method calls (`call`), not `implements`.

---

## Phase 7: Root Layer

> **Skill:** `create-root-typescript` — load for DI container and entry point wiring.

Wires concrete implementations to contracts and bootstraps the system.
Root is the **only layer** allowed to import all other layers.

### Steps

1. Load `create-root-typescript` skill.
2. Create DI container wiring: capabilities → orchestrator → surface.
3. Create entry point at `packages/root_<name>_entry.ts`.
4. Verify: `npx tsc --noEmit`.

### Example

```typescript
// packages/user/src/root_user_container.ts

import { IUserRepositoryProtocol } from "@shared/user/contract_user_protocol";
import { IUserAggregate } from "@shared/user/contract_user_aggregate";
import { UserRepository } from "./capabilities_user_repository";
import { UserOrchestrator } from "./agent_user_orchestrator";
import { GetUserCommand } from "./surface_user_command";

/** DI container for user feature. */
export class UserContainer {
  readonly getUserCommand: GetUserCommand;

  constructor(db: DatabaseConnection) {
    // Wire: capabilities → agent → surface
    const repository: IUserRepositoryProtocol = new UserRepository(db);
    const orchestrator: IUserAggregate = new UserOrchestrator(repository);
    this.getUserCommand = new GetUserCommand(orchestrator);
  }
}
```

```typescript
// packages/root_app_entry.ts

import { UserContainer } from "./user/src/root_user_container";

async function main(): Promise<void> {
  const db = await createDatabaseConnection();
  const container = new UserContainer(db);
  // start application...
}

main().catch(console.error);
```

### Rules Enforced

- **AES102**: Suffix must be `entry` or `container`.
- **AES201**: Root may import all layers. No forbidden imports.
- Root layer files are **skipped** by role-rules (AES401–406) and orphan-detector.

---

## Phase 8: Verify & CI Gate

> **Skill:** `build-verify-all` — load for final build verification.

### Step 1: Full AES Scan

```bash
lint-arwaky-cli scan .
```

**Target: 0 violations.**

### Step 2: Type Check

```bash
npx tsc --noEmit
```

### Step 3: Run Tests

```bash
npx vitest run
```

### Step 4: External Lint & Format

```bash
npx eslint packages/
npx prettier --check packages/
```

### Step 5: CI Gate

```bash
lint-arwaky-cli ci . --threshold 0
```

**Exit code 0** = all checks pass. **Exit code 1** = violations found.

### Step 6: External Lint via Lint Arwaky (optional)

```bash
lint-arwaky-cli external .
```

---

## Import Rules Quick Reference


| Source Layer   | May Import                             | Must NOT Import                                       |
| ---------------- | ---------------------------------------- | ------------------------------------------------------- |
| `taxonomy`     | taxonomy                               | contract, utility, capabilities, agent, surface, root |
| `contract`     | taxonomy, contract                     | utility, capabilities, agent, surface, root           |
| `utility`      | taxonomy                               | contract, capabilities, agent, surface, root          |
| `capabilities` | taxonomy, contract, utility            | capabilities, agent, surface, root                    |
| `agent`        | taxonomy, contract, utility            | capabilities, surface, root                           |
| `surface`      | taxonomy, contract(aggregate), utility | agent, capabilities, contract(protocol), root         |
| `root`         | ALL layers                             | —                                                    |

**Purpose enforcement** (AES201 sub-check):


| Import                             | Expected Purpose                                |
| ------------------------------------ | ------------------------------------------------- |
| capabilities → contract(protocol) | `implement` (`class Foo implements IProtocol`)  |
| agent → contract(aggregate)       | `implement` (`class Foo implements IAggregate`) |
| surface → contract(aggregate)     | `call` (method invocation)                      |
| capabilities → utility            | `call` (function invocation)                    |
| agent → utility                   | `call` (function invocation)                    |

---

## Supplementary Skills (Post-Migration)


| Skill                            | When to Use                                                               |
| ---------------------------------- | --------------------------------------------------------------------------- |
| `add-docs-typescript`            | Add JSDoc, type annotations after migration                               |
| `fix-bypass-typescript`          | Remove`@ts-ignore`, `@ts-expect-error`, `eslint-disable`, `FIXME`, `HACK` |
| `cleanup-consolidate-typescript` | Remove dead code, merge duplicates                                        |
| `create-test-typescript`         | Generate test suites (vitest)                                             |

---

## File Naming Reference


| Layer        | Pattern                              | Allowed Suffixes                                                                                              |
| -------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| taxonomy     | `taxonomy_<concept>_<suffix>.ts`     | `vo`, `entity`, `error`, `event`, `constant`                                                                  |
| contract     | `contract_<concept>_<suffix>.ts`     | `protocol`, `aggregate`                                                                                       |
| utility      | `utility_<concept>_<suffix>.ts`      | flexible (forbidden:`vo`, `entity`, `protocol`, `aggregate`)                                                  |
| capabilities | `capabilities_<concept>_<suffix>.ts` | flexible (forbidden:`vo`, `entity`, `protocol`, `aggregate`, `utility`)                                       |
| agent        | `agent_<concept>_orchestrator.ts`    | `orchestrator`                                                                                                |
| surface      | `surface_<concept>_<suffix>.ts`      | `command`, `controller`, `page`, `router`, `hook`, `store`, `action`, `screen`, `component`, `view`, `layout` |
| root         | `root_<concept>_<suffix>.ts`         | `entry`, `container`                                                                                          |

---

## Troubleshooting

### Common Violations and Fixes


| Code        | Violation                                          | Fix                                                                 |
| ------------- | ---------------------------------------------------- | --------------------------------------------------------------------- |
| AES101      | Filename not snake_case or < 3 words               | Rename to`prefix_concept_suffix.ts`                                 |
| AES102      | Wrong suffix for layer                             | Change suffix to match layer's allow-list                           |
| AES201      | Forbidden cross-layer import                       | Route through contract layer; use constructor injection             |
| AES202      | Missing mandatory import                           | Add required taxonomy/contract import                               |
| AES203      | Unused import                                      | Remove the import                                                   |
| AES204      | Dummy function (`_use_*`, `dummy_*`)               | Remove dummy function and the import it fakes                       |
| AES205      | Circular dependency                                | Break cycle via contract layer abstraction                          |
| AES301      | File > 1000 lines                                  | Split into smaller files                                            |
| AES304      | `@ts-ignore`, `@ts-expect-error`, `eslint-disable` | Fix the type error; remove the suppression                          |
| AES401      | Raw primitive in taxonomy                          | Wrap in Value Object class                                          |
| AES403      | Capability missing`implements`                     | Add`class Foo implements IProtocol`                                 |
| AES404      | Class/interface/enum/type in utility file          | Move types to taxonomy; keep only`export function` / `export const` |
| AES405      | Agent missing`implements`                          | Add`class Foo implements IAggregate`                                |
| AES406      | Too many functions in surface                      | Split into smaller surface files                                    |
| AES501–506 | Orphan file                                        | Wire into container or remove                                       |

### Parse Errors

If `lint-arwaky-cli` reports `PARSE_WARN` for a file, the file has a syntax
error that prevents AST parsing. Fix the syntax error first, then re-scan.

### Config Not Found

If no config file is found, lint-arwaky uses embedded defaults. Run
`lint-arwaky-cli init` to create an explicit config file.

### TypeScript-Specific: Path Aliases

AES TypeScript projects use **path aliases** for cross-package imports:

```json
// tsconfig.json
{
  "compilerOptions": {
    "paths": {
      "@shared/*": ["./packages/shared/src/*"]
    }
  }
}
```

```typescript
// ✅ Correct — path alias
import { UserId } from "@shared/user/taxonomy_user_vo";

// ❌ Wrong — deep relative import (fragile, breaks orphan detection)
import { UserId } from "../../../shared/src/user/taxonomy_user_vo";
```

### TypeScript-Specific: JSX/TSX Files

Surface layer files with JSX (`.tsx`) are fully supported. The tree-sitter
parser handles JSX syntax natively. AES naming and import rules apply
identically to `.ts` and `.tsx` files.

See [ARCHITECTURE.md](ARCHITECTURE.md) §12 for the full violation code reference.

---

## Reference

- Architecture: [ARCHITECTURE.md](ARCHITECTURE.md)
- CLI Reference: [README.md](README.md)
- PRD: [PRD.md](PRD.md)
- Test Plan: [TEST_PLAN.md](TEST_PLAN.md)
- Rust Migration Guide: [MIGRATION_RUST.md](MIGRATION_RUST.md)
- Python Migration Guide: [MIGRATION_PYTHON.md](MIGRATION_PYTHON.md)
