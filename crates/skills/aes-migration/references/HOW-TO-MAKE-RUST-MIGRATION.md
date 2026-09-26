# HOW TO MAKE MIGRATION RUST

> **Purpose**: Guide phased migration of legacy Rust projects into AES layered architecture — taxonomy → contract → utility → capabilities → agent → surface → root.
>
> **Audience**: Agents and engineers executing a migration to AES.
>
> **Scope**: Rust `crates/` workspaces; all 7 AES layers; 9 migration phases (0–8).
>
> **Location**: Project root; each phase operates on a layer directory under `crates/`.
>
> **Length**: 9 phases; duration depends on violation count.

---

## Rules

1. **Phase 0 first — audit before touching anything.** Run `lint-arwaky-cli scan .`; record baseline violations to choose strategy.
2. **Taxonomy before contract before capabilities.** VOs must exist before protocols can reference them.
3. **Protocol = one method per feature.** Never a mega `execute(op, …)` that dispatches multiple features.
4. **Aggregate = many methods, one per export.** Rich consumer surface; surface/root call specific verbs.
5. **Utility is stateless free functions only.** No struct, no impl, no upward imports (AES404).
6. **Capabilities implement protocol; agent implements aggregate.** No cross-layer imports (AES201).
7. **Surface calls aggregate; never imports agent or capabilities.** Dependency arrow points down (AES201 purpose).
8. **Root wires everything; never contains business logic.** Container constructs; entry bootstraps.
9. **Verify at every phase.** `lint-arwaky-cli scan <layer-dir>` → 0 before moving to next phase.

---

## Workflow

1. **Phase 0 — Audit** — run `lint-arwaky-cli scan .`; record baseline.
2. **Phase 1 — Taxonomy** — extract VOs, errors, constants.
3. **Phase 2 — Contract** — create protocol (1 method) + aggregate (many exports).
4. **Phase 3 — Utility** — extract stateless helpers to shared.
5. **Phase 4 — Capabilities** — implement protocols with business logic.
6. **Phase 5 — Agent** — implement aggregate, delegate to capabilities.
7. **Phase 6 — Surface** — map I/O, call aggregate.
8. **Phase 7 — Root** — wire containers, bootstrap entry.
9. **Phase 8 — Verify** — full scan → 0 violations; compile clean.

---

## AES Dependency Model

Layers do not import each other directly — they import from **contract** and receive dependencies via `Arc<dyn Trait>`:

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

         utility ←── imports taxonomy only ──┘
```

**Key principles:**
- Agent does **not** import capabilities — it receives them via `Arc<dyn Trait>`.
- Surface does **not** import agent — it receives the orchestrator via `Arc<dyn Trait>`.
- Capabilities **implements** protocol traits. Agent **implements** aggregate traits.
- Utility is flexible — imports taxonomy only, imported by capabilities/agent/surface.
- All import rules are enforced by `lint-arwaky-cli` (AES201–AES205).

---

## Workspace Structure

```
project-root/
├── Cargo.toml              ← workspace manifest (members = ["crates/*"])
├── lint_arwaky.config.yaml  ← AES config (created in Phase 0)
├── crates/
│   ├── shared/             ← shared types (subfolders per feature + common/)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs           ← re-exports all subfolders
│   │       ├── common/          ← truly shared across ALL features
│   │       └── <feature>/       ← shared types per feature domain
│   │           ├── mod.rs
│   │           ├── taxonomy_<concept>_vo.rs
│   │           ├── taxonomy_<concept>_error.rs
│   │           ├── taxonomy_<concept>_constant.rs
│   │           ├── contract_<concept>_protocol.rs
│   │           ├── contract_<concept>_aggregate.rs
│   │           └── utility_<concept>_<role>.rs
│   │
│   ├── <feature>/          ← feature crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── capabilities_<concept>_<role>.rs
│   │       ├── agent_<concept>_orchestrator.rs
│   │       ├── surface_<concept>_<role>.rs
│   │       └── root_<concept>_container.rs
│   │
│   └── root_<name>_entry.rs   ← binary entry point (file, NOT directory)
│
└── Cargo.lock
```

**Key rules:**
- All 7 layers coexist in each feature slice.
- Taxonomy, contracts, and utilities live under `crates/shared/<feature>/`.
- Capabilities, agent, surface, and root live in the feature crate.
- Entry points (`root_*_entry.rs`) are files inside `crates/`, not separate directories.
- `crates/shared/src/common/` holds types shared across ALL features.

---

## Prerequisites

```bash
# Install lint-arwaky
cargo install lint-arwaky-cli

# Verify installation
lint-arwaky-cli version
# Expected: Lint Arwaky v2.0.0+

# Install external linters (optional, for external lint checks)
lint-arwaky-cli install
```

---

## Phase 0: Audit & Config Setup

> **Skill:** `aes-lint-arwaky` — load for audit commands and violation analysis.

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
find crates -name "*.rs" | wc -l
```

---

## Phase 1: Taxonomy Layer

> **Skill:** `aes-taxonomy` — load for VOs, errors, constants, entities, events.

Define Value Objects, Errors, Events, and compile-time Constants under
`crates/shared/<feature>/`.

### Steps

1. Identify domain types:
   ```bash
   grep -rn "pub struct\|pub enum" crates/*/src/ | grep -v test | grep -v mod.rs
   ```
2. Load `aes-taxonomy` skill.
3. Create taxonomy files following skill templates.
4. Register in domain `mod.rs`.
5. Verify: `cargo check -p shared`.

### Example

```rust
// crates/shared/src/user/taxonomy_user_vo.rs

/// User identifier value object.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct UserId(String);

impl UserId {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Email value object with validation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Email(String);
```

### Rules Enforced

- **AES101**: Filename must be `taxonomy_<concept>_<suffix>.rs` (snake_case, 3+ words).
- **AES102**: Suffix must be `vo`, `entity`, `error`, `event`, or `constant`.
- **AES401**: No raw primitives (`String`, `i32`, `bool`) in type annotations — wrap in VOs.
- **AES401**: `_constant` files must contain only `pub const` / `pub static` — no `fn`, `struct`, `enum`.

---

## Phase 2: Contract Layer

> **Skill:** `aes-contract` — load for protocol and aggregate traits.

Contracts define public interfaces (Protocols and Aggregates) without
exposing implementation.

### Steps

1. Load `aes-contract` skill.
2. Create protocol traits (inbound/outbound) under `crates/shared/<feature>/`.
3. Create aggregate facade traits under `crates/shared/<feature>/`.
4. Register in domain `mod.rs`.
5. Verify: `cargo check -p shared`.

### Example

```rust
// crates/shared/src/user/contract_user_protocol.rs

use crate::user::taxonomy_user_vo::{UserId, Email};

/// Protocol for user repository operations.
/// Implemented by capabilities layer.
pub trait IUserRepositoryProtocol: Send + Sync {
    fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError>;
    fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserError>;
    fn save(&self, user: &User) -> Result<(), UserError>;
}
```

```rust
// crates/shared/src/user/contract_user_aggregate.rs

use crate::user::taxonomy_user_vo::UserId;

/// Aggregate facade for user operations.
/// Implemented by agent layer.
pub trait IUserAggregate: Send + Sync {
    fn get_user(&self, id: &UserId) -> Result<UserResponse, UserError>;
    fn register_user(&self, cmd: &RegisterCommand) -> Result<UserResponse, UserError>;
}
```

### Rules Enforced

- **AES102**: Suffix must be `protocol` or `aggregate`.
- **AES402**: No raw primitives in method signatures — use VOs.
- **AES201**: Protocol must not import aggregate. Aggregate may import protocol.

---

## Phase 3: Utility Layer

> **Skill:** `aes-utility` — load for stateless standalone functions.

Utility contains low-level technical mechanics — **stateless standalone
functions only**. No structs, no enums, no traits.

### Steps

1. Identify reusable stateless functions across modules.
2. Load `aes-utility` skill.
3. Create utility files under `crates/shared/<feature>/`.
4. Register in domain `mod.rs`.
5. Verify: `cargo check -p shared`.

### Example

```rust
// crates/shared/src/user/utility_user_validator.rs

use crate::user::taxonomy_user_vo::Email;

/// Validate email format. Stateless — no struct, no state.
pub fn validate_email(email: &Email) -> bool {
    let s = email.as_str();
    s.contains('@') && s.contains('.')
}

/// Normalize email to lowercase.
pub fn normalize_email(email: &Email) -> Email {
    Email::new(email.as_str().to_lowercase())
}
```

### Rules Enforced

- **AES102**: Suffix is flexible, but forbidden suffixes apply (`vo`, `entity`, `protocol`, `aggregate`, etc.).
- **AES404**: No `struct`, `enum`, `trait`, `type` definitions — functions and constants only.
- **AES201**: Utility may import taxonomy only. Must not import contract, capabilities, agent, surface.

---

## Phase 4: Capabilities Layer

> **Skill:** `aes-capabilities` — load for business logic and external adaptation.

Capabilities contain concrete behavior implementations. They **implement
protocol traits** defined in the contract layer.

### Steps

1. Load `aes-capabilities` skill.
2. Create business logic capabilities (implement protocol traits).
3. Create external adaptation capabilities (repositories, clients).
4. Follow **3-Block Structure**: Struct → Trait Impl → Constructors.
5. Use `Arc<dyn Trait>` for DI.
6. Verify: `cargo check -p <feature>`.

### Example

```rust
// crates/user/src/capabilities_user_repository.rs

use std::sync::Arc;
use shared::user::contract_user_protocol::IUserRepositoryProtocol;
use shared::user::taxonomy_user_vo::{UserId, Email, User, UserError};

// ─── Block 1: Struct ───
pub struct UserRepository {
    db_pool: Arc<ConnectionPool>,
}

// ─── Block 2: Trait Impl ───
impl IUserRepositoryProtocol for UserRepository {
    fn find_by_id(&self, id: &UserId) -> Result<Option<User>, UserError> {
        // concrete implementation
    }

    fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserError> {
        // concrete implementation
    }

    fn save(&self, user: &User) -> Result<(), UserError> {
        // concrete implementation
    }
}

// ─── Block 3: Constructors ───
impl UserRepository {
    pub fn new(db_pool: Arc<ConnectionPool>) -> Self {
        Self { db_pool }
    }
}
```

### Rules Enforced

- **AES102**: Suffix is flexible (forbidden: `vo`, `entity`, `protocol`, `aggregate`, `utility`).
- **AES201**: Capabilities may import taxonomy, contract, utility. Must not import agent, surface, other capabilities.
- **AES202**: Must import taxonomy and contract(protocol).
- **AES403**: At least 1 struct must implement a protocol trait. Max 3 type declarations per file.
- **AES201 purpose**: contract(protocol) imports must be used for `impl` (implement), not just function calls.

---

## Phase 5: Agent Layer

> **Skill:** `aes-agent` — load for orchestration logic.

Orchestrates sequential execution, branching, looping, and error handling.
**Implements aggregate traits** defined in the contract layer.

### Steps

1. Load `aes-agent` skill.
2. Create orchestrator struct implementing aggregate trait.
3. Inject protocol dependencies via `Arc<dyn Trait>`.
4. Verify: `cargo check -p <feature>`.

### Example

```rust
// crates/user/src/agent_user_orchestrator.rs

use std::sync::Arc;
use shared::user::contract_user_aggregate::IUserAggregate;
use shared::user::contract_user_protocol::IUserRepositoryProtocol;
use shared::user::taxonomy_user_vo::{UserId, UserResponse, UserError};

pub struct UserOrchestrator {
    repository: Arc<dyn IUserRepositoryProtocol>,
}

impl IUserAggregate for UserOrchestrator {
    fn get_user(&self, id: &UserId) -> Result<UserResponse, UserError> {
        let user = self.repository.find_by_id(id)?
            .ok_or(UserError::NotFound)?;
        Ok(UserResponse::from(user))
    }

    fn register_user(&self, cmd: &RegisterCommand) -> Result<UserResponse, UserError> {
        // orchestration logic: validate → save → return
    }
}

impl UserOrchestrator {
    pub fn new(repository: Arc<dyn IUserRepositoryProtocol>) -> Self {
        Self { repository }
    }
}
```

### Rules Enforced

- **AES102**: Suffix must be `orchestrator`.
- **AES201**: Agent may import taxonomy, contract(aggregate), contract(protocol), utility. Must not import capabilities, surface.
- **AES202**: Must import taxonomy and contract(aggregate).
- **AES405**: At least 1 struct must implement an aggregate trait. Max 3 type declarations.
- **AES201 purpose**: contract(aggregate) imports must be used for `impl` (implement).

---

## Phase 6: Surface Layer

> **Skill:** `aes-surface` — load for user-facing input translation.

Translates user-facing inputs into actions, delegating to the Agent
orchestrator via aggregate trait.

### Surface Classification

| Category    | Suffixes                                      | Rules                                                                                                 |
| ------------- | ----------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| **Smart**   | `_command`, `_controller`, `_page`, `_router` | May contain orchestration logic. Global limit: 15 functions.                                          |
| **Utility** | `_hook`, `_store`, `_action`, `_screen`       | Supports smart surfaces. Max 10 methods, 80 lines/method, 3 nesting depth, 3 control-flow statements. |
| **Passive** | `_component`, `_view`, `_layout`, others      | Presentation only. Same limits as Utility.                                                            |

### Steps

1. Load `aes-surface` skill.
2. Create surface structs (commands, handlers, endpoints).
3. Inject aggregate trait via `Arc<dyn Trait>`.
4. Verify: `cargo check -p <feature>`.

### Example

```rust
// crates/user/src/surface_user_command.rs

use std::sync::Arc;
use shared::user::contract_user_aggregate::IUserAggregate;
use shared::user::taxonomy_user_vo::{UserId, UserResponse, UserError};

pub struct GetUserCommand {
    aggregate: Arc<dyn IUserAggregate>,
}

impl GetUserCommand {
    pub fn new(aggregate: Arc<dyn IUserAggregate>) -> Self {
        Self { aggregate }
    }

    pub fn execute(&self, id: &UserId) -> Result<UserResponse, UserError> {
        self.aggregate.get_user(id)
    }
}
```

### Rules Enforced

- **AES102**: Suffix must be in the surface allow-list.
- **AES201**: Surface(command) may import taxonomy, contract(aggregate), utility. Must not import agent, capabilities, contract(protocol).
- **AES406**: Function count, method count, method length, nesting depth, and control-flow limits apply per surface category.
- **AES201 purpose**: contract(aggregate) imports must be used for function calls (`call`), not `impl`.

---

## Phase 7: Root Layer

> **Skill:** `aes-root` — load for DI container and entry point wiring.

Wires concrete implementations to contracts and bootstraps the system.
Root is the **only layer** allowed to import all other layers.

### Steps

1. Load `aes-root` skill.
2. Create DI container wiring: capabilities → orchestrator → surface.
3. Create entry point file at `crates/root_<name>_entry.rs`.
4. Verify: `cargo check -p <feature>`.

### Example

```rust
// crates/user/src/root_user_container.rs

use std::sync::Arc;
use crate::capabilities_user_repository::UserRepository;
use crate::agent_user_orchestrator::UserOrchestrator;
use crate::surface_user_command::GetUserCommand;
use shared::user::contract_user_protocol::IUserRepositoryProtocol;
use shared::user::contract_user_aggregate::IUserAggregate;

pub struct UserContainer {
    pub get_user_command: Arc<GetUserCommand>,
}

impl UserContainer {
    pub fn new(db_pool: Arc<ConnectionPool>) -> Self {
        // Wire: capabilities → agent → surface
        let repository: Arc<dyn IUserRepositoryProtocol> =
            Arc::new(UserRepository::new(db_pool));

        let orchestrator: Arc<dyn IUserAggregate> =
            Arc::new(UserOrchestrator::new(repository));

        let get_user_command =
            Arc::new(GetUserCommand::new(orchestrator));

        Self { get_user_command }
    }
}
```

### Rules Enforced

- **AES102**: Suffix must be `entry` or `container`.
- **AES201**: Root may import all layers. No forbidden imports.
- Root layer files are **skipped** by role-rules (AES401–406) and orphan-detector.

---

## Phase 8: Verify & CI Gate

> **Skill:** `aes-lint-arwaky` — load for final build verification.

### Step 1: Full AES Scan

```bash
lint-arwaky-cli scan .
```

**Target: 0 violations.**

### Step 2: Run Tests

```bash
cargo test --workspace
```

### Step 3: Format & Clippy

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

### Step 4: CI Gate

```bash
lint-arwaky-cli ci . --threshold 0
```

**Exit code 0** = all checks pass. **Exit code 1** = violations found.

### Step 5: External Lint (optional)

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

| Import                             | Expected Purpose                    |
| ------------------------------------ | ------------------------------------- |
| capabilities → contract(protocol) | `implement` (impl Trait for Struct) |
| agent → contract(aggregate)       | `implement` (impl Trait for Struct) |
| surface → contract(aggregate)     | `call` (function invocation)        |
| capabilities → utility            | `call` (function invocation)        |
| agent → utility                   | `call` (function invocation)        |

---

## Supplementary Skills (Post-Migration)

| Skill                      | When to Use                                             |
| ---------------------------- | --------------------------------------------------------- |
| `aes-docs`            | Add doc comments, type annotations after migration      |
| `aes-testing-suite`          | Generate test suites                                    |

---

## File Naming Reference

| Layer        | Pattern                              | Allowed Suffixes                                                                                              |
| -------------- | -------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| taxonomy     | `taxonomy_<concept>_<suffix>.rs`     | `vo`, `entity`, `error`, `event`, `constant`                                                                  |
| contract     | `contract_<concept>_<suffix>.rs`     | `protocol`, `aggregate`                                                                                       |
| utility      | `utility_<concept>_<suffix>.rs`      | flexible (forbidden: `vo`, `entity`, `protocol`, `aggregate`)                                                  |
| capabilities | `capabilities_<concept>_<suffix>.rs` | flexible (forbidden: `vo`, `entity`, `protocol`, `aggregate`, `utility`)                                       |
| agent        | `agent_<concept>_orchestrator.rs`    | `orchestrator`                                                                                                |
| surface      | `surface_<concept>_<suffix>.rs`      | `command`, `controller`, `page`, `router`, `hook`, `store`, `action`, `screen`, `component`, `view`, `layout` |
| root         | `root_<concept>_<suffix>.rs`         | `entry`, `container`                                                                                          |

---

## Troubleshooting

### Common Violations and Fixes

| Code        | Violation                            | Fix                                               |
| ------------- | -------------------------------------- | --------------------------------------------------- |
| AES101      | Filename not snake_case or < 3 words | Rename to `prefix_concept_suffix.rs`               |
| AES102      | Wrong suffix for layer               | Change suffix to match layer's allow-list         |
| AES201      | Forbidden cross-layer import         | Route through contract layer; use `Arc<dyn Trait>` |
| AES202      | Missing mandatory import             | Add required taxonomy/contract import             |
| AES203      | Unused import                        | Remove the import                                 |
| AES204      | Dummy function (`_use_*`, `dummy_*`) | Remove dummy function and the import it fakes     |
| AES205      | Circular dependency                  | Break cycle via contract layer abstraction        |
| AES301      | File > 1000 lines                    | Split into smaller files                          |
| AES304      | `unwrap()`, `#[allow(...)]`, `FIXME` | Use `match`/`expect` with context; fix root cause  |
| AES401      | Raw primitive in taxonomy            | Wrap in Value Object                              |
| AES403      | Capability missing protocol impl     | Add `impl IProtocol for Struct`                    |
| AES404      | Struct/enum in utility file          | Move type to taxonomy; keep only functions        |
| AES405      | Agent missing aggregate impl         | Add `impl IAggregate for Orchestrator`             |
| AES406      | Too many functions in surface        | Split into smaller surface files                  |
| AES501–506 | Orphan file                          | Wire into container or remove                     |

### Parse Errors

If `lint-arwaky-cli` reports `PARSE_WARN` for a file, the file has a syntax
error that prevents AST parsing. Fix the syntax error first, then re-scan.

### Config Not Found

If no config file is found, lint-arwaky uses embedded defaults. Run
`lint-arwaky-cli init` to create an explicit config file.

---

## Section Contract

| Section | Why it belongs here |
| ------- | ------------------- |
| Rules (numbered list) | Each rule prevents one specific migration failure mode. |
| Workflow (phases 0–8) | Explicit phase order; agents know which layer they are on. |
| Dependency model diagram | Grounds every import decision in the diagram. |
| Workspace structure | Shows where every layer's files live in a Rust workspace. |
| Phase examples | Copy-paste-ready skeletons so the agent never guesses the structure. |
| Rules Enforced per phase | Exact AES codes violated by each layer anti-pattern. |
| Import matrix | Quick lookup for which layer may import which. |
| Troubleshooting table | Maps violation codes to specific fixes. |

---

## Verify

```bash
# Phase 0 — baseline
lint-arwaky-cli scan .

# Per phase (replace <dir> with the layer directory being migrated)
lint-arwaky-cli scan crates/shared/src/<domain>  # taxonomy + contract + utility
lint-arwaky-cli scan crates/<domain>/src  # capabilities → agent → surface → root

# Final gate
lint-arwaky-cli scan .   # must be 0
cargo check --workspace
```

A pass means naming, layer imports, primitives, and roles are clean.
Migration strategy (phase order, skip conditions) is **manual** — see Rules §1–2.

Related: [HOW-TO-MAKE-PYTHON-MIGRATION.md](HOW-TO-MAKE-PYTHON-MIGRATION.md), [HOW-TO-MAKE-TYPESCRIPT-MIGRATION.md](HOW-TO-MAKE-TYPESCRIPT-MIGRATION.md)
