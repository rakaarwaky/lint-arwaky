# AES Migration Guide — TypeScript

> Step-by-step guide for migrating a TypeScript/JavaScript project to AES architecture.
> Workspace structure: `packages/` with npm/pnpm workspaces.

## Workspace Structure

```
project-root/
├── package.json             ← workspace root config
├── pnpm-workspace.yaml      ← pnpm workspace definition
├── packages/
│   ├── shared/              ← shared taxonomy + contract types
│   │   ├── package.json
│   │   └── src/
│   │       ├── index.ts
│   │       ├── taxonomy_common_vo.ts
│   │       ├── contract_common_port.ts
│   │       └── ...
│   ├── user/                ← feature package
│   │   ├── package.json
│   │   └── src/
│   │       ├── index.ts
│   │       ├── taxonomy_user_vo.ts
│   │       ├── taxonomy_user_error.ts
│   │       ├── taxonomy_user_constant.ts
│   │       ├── contract_user_port.ts
│   │       ├── contract_user_protocol.ts
│   │       ├── contract_user_aggregate.ts
│   │       ├── capabilities_user_checker.ts
│   │       ├── infrastructure_user_adapter.ts
│   │       ├── agent_user_orchestrator.ts
│   │       ├── surface_user_command.ts
│   │       └── root_user_container.ts
│   └── order/
│       └── src/
│           └── ...
└── src/
    └── root_cli_main_entry.ts   ← CLI entry point (at workspace root)
```

**Key rules:**

- All 7 layers coexist in each feature package, differentiated by filename prefix.
- Entry points (`root_*_entry.ts`) live at workspace root or `src/`.
- Shared types go in `packages/shared/`.

---

## Prerequisites

```bash
npm install -g lint-arwaky-cli
lint-arwaky-cli version
lint-arwaky-cli check your-project/
```

---

## Phase 0: Audit

```bash
lint-arwaky-cli check your-project/
find your-project/packages -name "*.ts" | wc -l
```

---

## Phase 1: Taxonomy Layer

### Step 1.1: Identify Domain Types

```bash
grep -rn "^export interface\|^export type\|^export enum\|^export class" your-project/packages/*/src/ | grep -v test | grep -v node_modules
```

### Step 1.2: Create Value Objects

**Before:** `packages/user/src/user.ts` (interface + logic mixed)
**After:** `packages/user/src/taxonomy_user_vo.ts` (pure data)

```typescript
// packages/user/src/taxonomy_user_vo.ts
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
// packages/user/src/taxonomy_user_constant.ts
/** Application constants. */

export const MAX_RETRY_COUNT = 3;
export const DEFAULT_TIMEOUT_MS = 5000;
export const API_VERSION = "v1" as const;
```

### Step 1.4: Create Error Types

```typescript
// packages/user/src/taxonomy_user_error.ts
/** Domain error types. */

export class UserError extends Error {
  constructor(
    message: string,
    public readonly code?: string,
  ) {
    super(message);
    this.name = "UserError";
  }
}

export class UserNotFoundError extends UserError {
  constructor(userId: string) {
    super(`User not found: ${userId}`, "USER_NOT_FOUND");
    this.name = "UserNotFoundError";
  }
}

export class InvalidEmailError extends UserError {
  constructor(email: string) {
    super(`Invalid email: ${email}`, "INVALID_EMAIL");
    this.name = "InvalidEmailError";
  }
}
```

---

## Phase 2: Contract Layer

### Step 2.1: Identify Outbound Dependencies

```bash
grep -rn "fs\.\|http\.\|fetch\|axios\|mysql\|pg\|redis" your-project/packages/*/src/
```

### Step 2.2: Create Ports (outbound interfaces)

```typescript
// packages/user/src/contract_user_port.ts
/** Outbound port for user persistence — implemented by infrastructure. */

import type { UserVO } from "./taxonomy_user_vo";

export interface IUserPort {
  findById(id: string): Promise<UserVO | null>;
  save(user: UserVO): Promise<void>;
  delete(id: string): Promise<void>;
}
```

### Step 2.3: Create Protocols (inbound interfaces)

```typescript
// packages/user/src/contract_user_protocol.ts
/** Inbound protocol for user operations — implemented by capabilities. */

import type { UserVO } from "./taxonomy_user_vo";

export interface IUserProtocol {
  getUser(id: string): Promise<UserVO>;
  createUser(name: string, email: string): Promise<UserVO>;
}
```

### Step 2.4: Create Aggregates (facades)

```typescript
// packages/user/src/contract_user_aggregate.ts
/** Aggregate facade — combines user protocols for surface layer. */

import type { UserVO } from "./taxonomy_user_vo";

export interface IUserAggregate {
  getUser(id: string): Promise<UserVO>;
  createUser(name: string, email: string): Promise<UserVO>;
  deleteUser(id: string): Promise<void>;
}
```

---

## Phase 3: Capabilities Layer

Business logic only. No infrastructure imports.

```typescript
// packages/user/src/capabilities_user_checker.ts
/** Validates user domain rules — pure business logic. */

import type { IUserProtocol } from "./contract_user_protocol";

export class UserChecker {
  constructor(private readonly userProtocol: IUserProtocol) {}

  validateEmail(email: string): boolean {
    return email.includes("@") && email.includes(".");
  }

  async checkUniqueEmail(email: string): Promise<boolean> {
    const existing = await this.userProtocol.getByEmail(email);
    return existing === null;
  }
}
```

---

## Phase 4: Infrastructure Layer

Each adapter implements a port.

```typescript
// packages/user/src/infrastructure_user_adapter.ts
/** User persistence adapter — implements IUserPort for database. */

import type { IUserPort } from "./contract_user_port";
import type { UserVO } from "./taxonomy_user_vo";

export class UserAdapter implements IUserPort {
  constructor(private readonly dbPath: string) {}

  async findById(id: string): Promise<UserVO | null> {
    // Actual database call here
    throw new Error("Implement database query");
  }

  async save(user: UserVO): Promise<void> {
    throw new Error("Implement database insert/update");
  }

  async delete(id: string): Promise<void> {
    throw new Error("Implement database delete");
  }
}
```

---

## Phase 5: Agent Layer

Orchestrator coordinates capabilities and infrastructure.

```typescript
// packages/user/src/agent_user_orchestrator.ts
/** User orchestration — coordinates user-related operations. */

import { v4 as uuidv4 } from "uuid";
import type { IUserAggregate } from "./contract_user_aggregate";
import type { IUserPort } from "./contract_user_port";
import type { UserChecker } from "./capabilities_user_checker";
import type { UserVO } from "./taxonomy_user_vo";
import { InvalidEmailError, UserNotFoundError } from "./taxonomy_user_error";

export class UserOrchestrator implements IUserAggregate {
  constructor(
    private readonly checker: UserChecker,
    private readonly port: IUserPort,
  ) {}

  async getUser(id: string): Promise<UserVO> {
    const user = await this.port.findById(id);
    if (user === null) {
      throw new UserNotFoundError(id);
    }
    return user;
  }

  async createUser(name: string, email: string): Promise<UserVO> {
    if (!this.checker.validateEmail(email)) {
      throw new InvalidEmailError(email);
    }
    const user: UserVO = { id: uuidv4(), name, email };
    await this.port.save(user);
    return user;
  }

  async deleteUser(id: string): Promise<void> {
    await this.port.delete(id);
  }
}
```

---

## Phase 6: Surface Layer

CLI commands, API handlers. Delegates to orchestrator.

```typescript
// packages/user/src/surface_user_command.ts
/** CLI command for user operations. */

import type { IUserAggregate } from "./contract_user_aggregate";

export class UserCommand {
  constructor(private readonly orchestrator: IUserAggregate) {}

  async run(args: string[]): Promise<string> {
    if (args.length === 0) {
      return "Usage: user <get|create> [args...]";
    }

    const [action, ...rest] = args;

    switch (action) {
      case "get": {
        const id = rest[0];
        if (!id) return "Missing user ID";
        const user = await this.orchestrator.getUser(id);
        return `User: ${user.name} <${user.email}>`;
      }
      case "create": {
        const name = rest[0] ?? "";
        const email = rest[1] ?? "";
        const user = await this.orchestrator.createUser(name, email);
        return `Created: ${user.id}`;
      }
      default:
        return `Unknown action: ${action}`;
    }
  }
}
```

---

## Phase 7: Root Layer

DI container wires everything. Entry point bootstraps.

### Container (inside feature package)

```typescript
// packages/user/src/root_user_container.ts
/** DI container — wires all user-related dependencies. */

import { UserAdapter } from "./infrastructure_user_adapter";
import { UserChecker } from "./capabilities_user_checker";
import { UserOrchestrator } from "./agent_user_orchestrator";

export class UserContainer {
  readonly orchestrator: UserOrchestrator;

  constructor(dbPath: string) {
    const port = new UserAdapter(dbPath);
    const checker = new UserChecker(port);
    this.orchestrator = new UserOrchestrator(checker, port);
  }
}
```

### Entry Point (at workspace root)

```typescript
// src/root_cli_main_entry.ts
/** CLI entry point — bootstraps the application. */

import { UserContainer } from "../packages/user/src/root_user_container";
import { UserCommand } from "../packages/user/src/surface_user_command";

async function main(): Promise<void> {
  const container = new UserContainer("data.db");
  const command = new UserCommand(container.orchestrator);

  const args = process.argv.slice(2);
  try {
    const result = await command.run(args);
    console.log(result);
  } catch (e) {
    console.error(`Error: ${e}`);
    process.exit(1);
  }
}

main();
```

---

## Phase 8: Verify

```bash
lint-arwaky-cli check your-project/
npm test
npx tsc --noEmit
npx eslint .
```

---

## File Naming Reference

| Layer          | Pattern                                | Example                          |
| -------------- | -------------------------------------- | -------------------------------- |
| taxonomy       | `taxonomy_<concept>_<suffix>.ts`       | `taxonomy_user_vo.ts`            |
| contract       | `contract_<concept>_<suffix>.ts`       | `contract_user_port.ts`          |
| capabilities   | `capabilities_<concept>_<suffix>.ts`   | `capabilities_user_checker.ts`   |
| infrastructure | `infrastructure_<concept>_<suffix>.ts` | `infrastructure_user_adapter.ts` |
| agent          | `agent_<concept>_orchestrator.ts`      | `agent_user_orchestrator.ts`     |
| surface        | `surface_<concept>_<suffix>.ts`        | `surface_user_command.ts`        |
| root           | `root_<concept>_<suffix>.ts`           | `root_user_container.ts`         |

---

## Import Rules

```
taxonomy_       → taxonomy_*
contract_       → taxonomy_*, contract_*
capabilities_   → taxonomy_*, contract_*
infrastructure_ → taxonomy_*, contract_*
agent_          → taxonomy_*, contract_aggregate_*, contract_port_*, contract_protocol_*
surface_        → taxonomy_*, contract_aggregate_*
root_           → ALL layers
```

**NEVER:** capabilities → infrastructure, agent → surface, surface → capabilities.

---

## Troubleshooting

| Violation  | Fix                                               |
| ---------- | ------------------------------------------------- |
| AES101     | Rename to `layer_concept_suffix`                  |
| AES102     | Change suffix to match layer's allowed list       |
| AES201     | Remove forbidden import, use contract interface   |
| AES202     | Add missing import per layer requirements         |
| AES303     | Add interface/type/class definition               |
| AES304     | Remove `eslint-disable`                           |
| AES401     | Move primitives to VO, constants to `_constant`   |
| AES402     | Replace primitive types with VO types in contract |
| AES403     | Implement protocol interface in capability        |
| AES404     | Implement port interface in infrastructure        |
| AES501-506 | Wire in container or remove dead code             |
