# AES Migration Guide — TypeScript

> Step-by-step guide for migrating a TypeScript/JavaScript project to AES architecture.
> Workspace structure: `packages/` with npm/pnpm workspaces.

See [ARCHITECTURE.md](ARCHITECTURE.md) for layer rules and [README.md](README.md) for project usage.

## Workspace Structure

```
project-root/
├── package.json             ← workspace root config
├── pnpm-workspace.yaml      ← pnpm workspace definition (if using pnpm)
├── packages/
│   ├── shared/              ← shared taxonomy + contract + utility types
│   │   ├── package.json
│   │   └── src/
│   │       ├── index.ts
│   │       ├── common/          ← truly shared across ALL features
│   │       │   ├── index.ts
│   │       │   ├── taxonomy_common_vo.ts
│   │       │   ├── taxonomy_path_vo.ts
│   │       │   └── ...
│   │       └── user/            ← shared types for user feature (domain folder)
│   │           ├── index.ts
│   │           ├── taxonomy_user_vo.ts
│   │           ├── taxonomy_user_error.ts
│   │           ├── taxonomy_user_constant.ts
│   │           ├── contract_user_protocol.ts
│   │           ├── contract_user_aggregate.ts
│   │           └── utility_user_hasher.ts
│   │
│   ├── user/                ← feature package
│   │   ├── package.json
│   │   └── src/
│   │       ├── index.ts
│   │       ├── capabilities_user_checker.ts     ← business logic capability
│   │       ├── capabilities_user_repository.ts  ← external adaptation capability
│   │       ├── agent_user_orchestrator.ts       ← agent layer (orchestrator)
│   │       ├── surface_user_command.ts          ← surfaces layer
│   │       └── root_user_container.ts           ← root container
│   └── order/
│       └── src/
│           └── ...
└── src/
    └── root_cli_main_entry.ts   ← CLI entry point (at workspace root)
```

**Key rules:**

- All 7 layers coexist in each feature slice. Stable domain taxonomy, contracts, and utilities live under `packages/shared/src/<feature>/`. Orchestration, capabilities, and surfaces live in the feature package.
- Entry points (`root_*_entry.ts`) live at workspace root or `src/`.
- Shared types go in `packages/shared/`.

---

## Prerequisites

```bash
npm install -g lint-arwaky-cli
lint-arwaky-cli version
lint-arwaky-cli scan your-project/
```

---

## Phase 0: Audit

```bash
lint-arwaky-cli scan your-project/
find your-project/packages -name "*.ts" | wc -l
```

- Violations < 10 → full migration in one session
- Violations 10-50 → phased migration
- Violations > 50 → start with taxonomy only

---

## Phase 1: Taxonomy Layer

Define Value Objects, Errors, Events, and compile-time Constants under the `shared` member.

### Step 1.1: Identify Domain Types

```bash
grep -rn "^export interface\|^export type\|^export enum\|^export class" your-project/packages/*/src/ | grep -v test | grep -v node_modules
```

### Step 1.2: Create Value Objects

```typescript
// packages/shared/src/user/taxonomy_user_vo.ts
/** User value object — immutable domain data container. */

export interface UserVO {
  readonly id: string;
  readonly name: string;
  readonly email: string;
}

export function createUserVO(id: string, name: string, email: string): UserVO {
  return { id, name, email };
}
```

### Step 1.3: Create Constants

```typescript
// packages/shared/src/user/taxonomy_user_constant.ts
/** User constants — compile-time literal values. */

export const MAX_RETRY_COUNT = 3;
export const DEFAULT_TIMEOUT_MS = 5000;
```

### Step 1.4: Create Error Types

```typescript
// packages/shared/src/user/taxonomy_user_error.ts
/** User domain-level errors. */

export class UserError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "UserError";
  }
}

export class UserNotFoundError extends UserError {
  constructor(userId: string) {
    super(`User not found: ${userId}`);
    this.name = "UserNotFoundError";
  }
}

export class InvalidEmailError extends UserError {
  constructor(email: string) {
    super(`Invalid email: ${email}`);
    this.name = "InvalidEmailError";
  }
}
```

---

## Phase 2: Contract Layer

Contracts define public interfaces (Protocols and Aggregates) without exposing implementation.

### Step 2.1: Create Protocols (inbound/outbound interfaces)

Define protocol interfaces implemented by Capabilities (both business calculation and external adapters) and consumed by the Agent.

```typescript
// packages/shared/src/user/contract_user_protocol.ts
/** User contract protocols. */

import type { UserVO } from "./taxonomy_user_vo";

export interface IUserProtocol {
  checkValidEmail(email: string): boolean;
}

export interface IUserRepositoryProtocol {
  findById(userId: string): Promise<UserVO | null>;
  save(user: UserVO): Promise<void>;
  delete(userId: string): Promise<void>;
}
```

### Step 2.2: Create Aggregates (facades)

Define aggregate facades implemented by the Agent and consumed by Surfaces.

```typescript
// packages/shared/src/user/contract_user_aggregate.ts
/** User contract aggregate facade. */

import type { UserVO } from "./taxonomy_user_vo";

export interface IUserAggregate {
  getUser(userId: string): Promise<UserVO>;
  createUser(name: string, email: string): Promise<UserVO>;
  deleteUser(userId: string): Promise<void>;
}
```

---

## Phase 3: Utility Layer

Utility contains low-level technical mechanics. It must contain only **stateless standalone functions** (no stateful objects, no behavior, no contract implementation, and no business decisions).

### Step 3.1: Create Technical Utilities

Extract reusable technical actions (e.g. parsing, hash computation, formatting) into the Utility layer inside the `shared` member.

```typescript
// packages/shared/src/user/utility_user_hasher.ts
/** User utility functions. */

export function hashUserToken(inputStr: string): string {
  // stateless technical operation
  return `hash_${inputStr}`;
}
```

---

## Phase 4: Capabilities Layer

Capabilities contain concrete behavior implementations. This includes business logic (validations, computations) and external adaptation (database repositories, network integration, third-party clients).

- Must implement one domain protocol interface defined in Contract.
- Must use dependency injection for collaborator services.
- Must not import or depend on other Capabilities.

### Step 4.1: Create Business Logic Capability

```typescript
// packages/user/src/capabilities_user_checker.ts
/** Validates user domain rules — pure business logic. */

import type { IUserProtocol } from "shared/user/contract_user_protocol";

export class UserChecker implements IUserProtocol {
  checkValidEmail(email: string): boolean {
    return email.includes("@") && email.includes(".");
  }
}
```

### Step 4.2: Create External Adaptation Capability (formerly Infrastructure)

```typescript
// packages/user/src/capabilities_user_repository.ts
/** User persistence repository — implements IUserRepositoryProtocol for database. */

import type { IUserRepositoryProtocol } from "shared/user/contract_user_protocol";
import type { UserVO } from "shared/user/taxonomy_user_vo";

export class UserRepository implements IUserRepositoryProtocol {
  constructor(private readonly dbPath: string) {}

  async findById(userId: string): Promise<UserVO | null> {
    // Actual database call here using local state or shared utilities
    throw new Error("Query DB");
  }

  async save(user: UserVO): Promise<void> {
    throw new Error("Insert/update user");
  }

  async delete(userId: string): Promise<void> {
    throw new Error("Delete user");
  }
}
```

---

## Phase 5: Agent Layer

Orchestrates sequential execution, branching, looping, and error handling. Ignorant of concrete capability and utility implementations (coordinates only via contract protocols injected at constructor time).

```typescript
// packages/user/src/agent_user_orchestrator.ts
/** User orchestration — coordinates user-related operations. */

import { v4 as uuidv4 } from "uuid";
import type { IUserAggregate } from "shared/user/contract_user_aggregate";
import type {
  IUserProtocol,
  IUserRepositoryProtocol,
} from "shared/user/contract_user_protocol";
import type { UserVO } from "shared/user/taxonomy_user_vo";
import {
  InvalidEmailError,
  UserNotFoundError,
} from "shared/user/taxonomy_user_error";

export class UserOrchestrator implements IUserAggregate {
  constructor(
    private readonly checker: IUserProtocol,
    private readonly repository: IUserRepositoryProtocol,
  ) {}

  async getUser(userId: string): Promise<UserVO> {
    const user = await this.repository.findById(userId);
    if (user === null) {
      throw new UserNotFoundError(userId);
    }
    return user;
  }

  async createUser(name: string, email: string): Promise<UserVO> {
    if (!this.checker.checkValidEmail(email)) {
      throw new InvalidEmailError(email);
    }
    const user: UserVO = { id: uuidv4(), name, email };
    await this.repository.save(user);
    return user;
  }

  async deleteUser(userId: string): Promise<void> {
    await this.repository.delete(userId);
  }
}
```

---

## Phase 6: Surface Layer

Translates user-facing inputs into actions, delegating execution to the Agent orchestrator.

```typescript
// packages/user/src/surface_user_command.ts
/** CLI command surface for user operations. */

import type { IUserAggregate } from "shared/user/contract_user_aggregate";

export class UserCommand {
  constructor(private readonly orchestrator: IUserAggregate) {}

  async run(args: string[]): Promise<string> {
    if (args.length === 0) {
      return "Usage: user <get|create> [args...]";
    }

    const action = args[0];
    if (action === "get") {
      const id = args[1];
      if (!id) throw new Error("Missing user ID");
      const user = await this.orchestrator.getUser(id);
      return `User: ${user.name} <${user.email}>`;
    } else if (action === "create") {
      const name = args[1];
      const email = args[2];
      if (!name || !email) throw new Error("Missing name or email");
      const user = await this.orchestrator.createUser(name, email);
      return `Created user: ${user.id}`;
    } else {
      return "Usage: user <get|create> [args...]";
    }
  }
}
```

---

## Phase 7: Root Layer

Wires concrete implementations to contracts and bootstraps the system.

### Container

```typescript
// packages/user/src/root_user_container.ts
/** User feature DI container. */

import { UserOrchestrator } from "./agent_user_orchestrator";
import { UserChecker } from "./capabilities_user_checker";
import { UserRepository } from "./capabilities_user_repository";
import type { IUserAggregate } from "shared/user/contract_user_aggregate";

export class UserContainer {
  private readonly _orchestrator: IUserAggregate;

  constructor(dbPath: string) {
    const checker = new UserChecker();
    const repository = new UserRepository(dbPath);
    this._orchestrator = new UserOrchestrator(checker, repository);
  }

  get orchestrator(): IUserAggregate {
    return this._orchestrator;
  }
}
```

### Entry Point

```typescript
// src/root_cli_main_entry.ts
/** CLI Main entry point. */

import { UserContainer } from "../packages/user/src/root_user_container";
import { UserCommand } from "../packages/user/src/surface_user_command";

async function main() {
  const container = new UserContainer("data.db");
  const command = new UserCommand(container.orchestrator);

  const args = process.argv.slice(2);
  try {
    const output = await command.run(args);
    console.log(output);
  } catch (error: any) {
    console.error(`Error: ${error.message}`);
    process.exit(1);
  }
}

main();
```

---

## Phase 8: Verify

```bash
lint-arwaky-cli scan your-project/
vitest run
npm run lint && npm run format
```

---

## File Naming Reference

| Layer        | Pattern                              | Example                        |
| ------------ | ------------------------------------ | ------------------------------ |
| taxonomy     | `taxonomy_<concept>_<suffix>.ts`     | `taxonomy_user_vo.ts`          |
| contract     | `contract_<concept>_<suffix>.ts`     | `contract_user_protocol.ts`    |
| utility      | `utility_<concept>_<suffix>.ts`      | `utility_user_hasher.ts`       |
| capabilities | `capabilities_<concept>_<suffix>.py` | `capabilities_user_checker.ts` |
| agent        | `agent_<concept>_orchestrator.ts`    | `agent_user_orchestrator.ts`   |
| surface      | `surface_<concept>_<suffix>.ts`      | `surface_user_command.ts`      |
| root         | `root_<concept>_<suffix>.ts`         | `root_user_container.ts`       |

---

## Import Rules

```
taxonomy_     → taxonomy_*
contract_     → taxonomy_*
utility_      → taxonomy_*
capabilities_ → taxonomy_*, contract_*, utility_*
agent_        → taxonomy_*, contract_*
surface_      → taxonomy_*, contract_*, utility_*
root_         → ALL layers
```

**NEVER:** capabilities → agent, agent → surface, surface → capabilities, capability → capability.

---

## Troubleshooting

| Violation  | Fix                                               |
| ---------- | ------------------------------------------------- |
| AES101     | Rename to `layer_concept_suffix`                  |
| AES102     | Change suffix to match layer's allowed list       |
| AES201     | Remove forbidden import, use contract interface   |
| AES202     | Add missing import per layer requirements         |
| AES303     | Add struct/enum/trait definition                  |
| AES304     | Remove `#[allow]`, `unwrap()`, `panic!`           |
| AES401     | Move primitives to VO, constants to `_constant`   |
| AES402     | Replace primitive types with VO types in contract |
| AES403     | Implement protocol trait in capability            |
| AES404     | Move stateless helper functions to Utility        |
| AES501-506 | Wire in container or remove dead code             |
