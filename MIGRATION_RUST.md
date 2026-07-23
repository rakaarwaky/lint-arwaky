# AES Migration Guide — Rust

> Step-by-step guide for migrating a Rust project to AES architecture.
> Workspace structure: `crates/` with Cargo workspace.

See [ARCHITECTURE.md](ARCHITECTURE.md) for layer rules and [README.md](README.md) for project usage.

## Workspace Structure

```
project-root/
├── Cargo.toml              ← workspace manifest (members = ["crates/*"])
├── crates/
│   ├── shared/             ← shared types (subfolders per feature + common/)
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs           ← re-exports all subfolders
│   │       ├── common/          ← truly shared across ALL features
│   │       │   ├── mod.rs
│   │       │   ├── taxonomy_common_vo.rs
│   │       │   ├── taxonomy_path_vo.rs
│   │       │   └── ...
│   │       ├── user/            ← shared types for user feature (domain folder)
│   │       │   ├── mod.rs
│   │       │   ├── taxonomy_user_vo.rs
│   │       │   ├── taxonomy_user_error.rs
│   │       │   ├── taxonomy_user_constant.rs
│   │       │   ├── contract_user_protocol.rs
│   │       │   ├── contract_user_aggregate.rs
│   │       │   ├── utility_user_hasher.rs
│   │       │   └── ...
│   │       └── ...              ← one subfolder per feature crate
│   │
│   ├── user/               ← feature crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── capabilities_user_checker.rs     ← business logic capability
│   │       ├── capabilities_user_repository.rs  ← external adaptation capability
│   │       ├── agent_user_orchestrator.rs       ← agent layer (orchestrator)
│   │       ├── surface_user_command.rs          ← surfaces layer
│   │       ├── root_user_container.rs           ← root container
│   │       └── lib.rs
│   ├── root_cli_main_entry.rs   ← CLI binary (file, NOT directory)
│   ├── root_mcp_main_entry.rs   ← MCP server binary
│   ├── root_tui_main_entry.rs   ← TUI binary
│   └── lib.rs
└── Cargo.lock
```

**Key rules:**

- All 7 layers coexist in each feature slice. Stable domain taxonomy, contracts, and utilities live under `crates/shared/<feature>/`. Orchestration, capabilities, and surfaces live in the feature crate.
- Entry points (`root_*_entry.rs`) are files inside `crates/`, not separate directories.
- `crates/shared/src/common/` holds types shared across ALL features (path VOs, common errors, etc.).

---

## Prerequisites

```bash
cargo install lint-arwaky-cli
lint-arwaky-cli version
lint-arwaky-cli check your-project/
```

---

## Phase 0: Audit

```bash
lint-arwaky-cli check your-project/
find your-project/crates -name "*.rs" | wc -l
```

- Violations < 10 → full migration in one session
- Violations 10-50 → phased migration
- Violations > 50 → start with taxonomy only

---

## Phase 1: Taxonomy Layer

Define Value Objects, Errors, Events, and compile-time Constants under the `shared` member.

### Step 1.1: Identify Domain Types

```bash
grep -rn "pub struct\|pub enum" your-project/crates/*/src/ | grep -v test | grep -v mod.rs
```

### Step 1.2: Create Value Objects

```rust
// crates/shared/src/user/taxonomy_user_vo.rs
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UserVO {
    pub id: String,
    pub name: String,
    pub email: String,
}

impl UserVO {
    pub fn new(id: String, name: String, email: String) -> Self {
        Self { id, name, email }
    }
}
```

### Step 1.3: Create Constants

```rust
// crates/shared/src/user/taxonomy_user_constant.rs
pub const MAX_RETRY_COUNT: u32 = 3;
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;
```

### Step 1.4: Create Error Types

```rust
// crates/shared/src/user/taxonomy_user_error.rs
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("User not found: {0}")]
    NotFound(String),
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
}
```

### Step 1.5: Register in mod.rs

```rust
// crates/shared/src/user/mod.rs
pub mod taxonomy_user_vo;
pub mod taxonomy_user_error;
pub mod taxonomy_user_constant;
```

---

## Phase 2: Contract Layer

Contracts define public interfaces (Protocols and Aggregates) without exposing implementation.

### Step 2.1: Create Protocols (inbound/outbound interfaces)

Define protocol traits implemented by Capabilities (both business calculation and external adapters) and consumed by the Agent.

```rust
// crates/shared/src/user/contract_user_protocol.rs
use crate::user::taxonomy_user_vo::UserVO;

pub trait IUserProtocol {
    fn check_valid_email(&self, email: &str) -> bool;
}

pub trait IUserRepositoryProtocol {
    fn find_by_id(&self, id: &str) -> Result<Option<UserVO>, Box<dyn std::error::Error>>;
    fn save(&self, user: &UserVO) -> Result<(), Box<dyn std::error::Error>>;
    fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
}
```

### Step 2.2: Create Aggregates (facades)

Define aggregate facades implemented by the Agent and consumed by Surfaces.

```rust
// crates/shared/src/user/contract_user_aggregate.rs
use crate::user::taxonomy_user_vo::UserVO;

pub trait IUserAggregate {
    fn get_user(&self, id: &str) -> Result<UserVO, String>;
    fn create_user(&self, name: &str, email: &str) -> Result<UserVO, String>;
    fn delete_user(&self, id: &str) -> Result<(), String>;
}
```

### Step 2.3: Register in mod.rs

```rust
// crates/shared/src/user/mod.rs
pub mod contract_user_protocol;
pub mod contract_user_aggregate;
```

---

## Phase 3: Utility Layer

Utility contains low-level technical mechanics. It must contain only **stateless standalone functions** (no stateful objects, no behavior, no contract implementation, and no business decisions).

### Step 3.1: Create Technical Utilities

Extract reusable technical actions (e.g. parsing, hash computation, formatting) into the Utility layer inside the `shared` member.

```rust
// crates/shared/src/user/utility_user_hasher.rs
pub fn hash_user_token(input: &str) -> String {
    // stateless technical operation
    format!("hash_{}", input)
}
```

### Step 3.2: Register in mod.rs

```rust
// crates/shared/src/user/mod.rs
pub mod utility_user_hasher;
```

---

## Phase 4: Capabilities Layer

Capabilities contain concrete behavior implementations. This includes business logic (validations, computations) and external adaptation (database repositories, network integration, third-party clients).

- Must implement one domain protocol trait defined in Contract.
- Must follow strict **3-Block Structure** separated by block comments.
- Must use dependency injection for collaborator services via `Arc<dyn Trait>`.
- Must not import or depend on other Capabilities.

### Step 4.1: Create Business Logic Capability

```rust
// crates/user/src/capabilities_user_checker.rs
use shared::user::contract_user_protocol::IUserProtocol;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct UserChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IUserProtocol for UserChecker {
    fn check_valid_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl Default for UserChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl UserChecker {
    pub fn new() -> Self {
        Self
    }
}
```

### Step 4.2: Create External Adaptation Capability (formerly Infrastructure)

```rust
// crates/user/src/capabilities_user_repository.rs
use shared::user::contract_user_protocol::IUserRepositoryProtocol;
use shared::user::taxonomy_user_vo::UserVO;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct UserRepository {
    db_path: String,
}

// ─── Block 2: Protocol Trait Implementation ───────────────
impl IUserRepositoryProtocol for UserRepository {
    fn find_by_id(&self, id: &str) -> Result<Option<UserVO>, Box<dyn std::error::Error>> {
        // Concrete database query using local state and/or shared utilities
        todo!("Query DB at {}", self.db_path)
    }

    fn save(&self, user: &UserVO) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Insert/update user")
    }

    fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Delete user")
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl UserRepository {
    pub fn new(db_path: String) -> Self {
        Self { db_path }
    }
}
```

---

## Phase 5: Agent Layer

Orchestrates sequential execution, branching, looping, and error handling. Ignorant of concrete capability and utility implementations (coordinates only via contract protocols injected via `Arc`).

```rust
// crates/user/src/agent_user_orchestrator.rs
use shared::user::contract_user_aggregate::IUserAggregate;
use shared::user::contract_user_protocol::{IUserProtocol, IUserRepositoryProtocol};
use shared::user::taxonomy_user_vo::UserVO;
use std::sync::Arc;

pub struct UserOrchestrator {
    checker: Arc<dyn IUserProtocol>,
    repository: Arc<dyn IUserRepositoryProtocol>,
}

impl UserOrchestrator {
    pub fn new(
        checker: Arc<dyn IUserProtocol>,
        repository: Arc<dyn IUserRepositoryProtocol>,
    ) -> Self {
        Self { checker, repository }
    }
}

impl IUserAggregate for UserOrchestrator {
    fn get_user(&self, id: &str) -> Result<UserVO, String> {
        self.repository.find_by_id(id)
            .map_err(|e| e.to_string())?
            .ok_or_else(|| format!("User not found: {}", id))
    }

    fn create_user(&self, name: &str, email: &str) -> Result<UserVO, String> {
        if !self.checker.check_valid_email(email) {
            return Err("Invalid email".to_string());
        }
        let user = UserVO::new(
            uuid::Uuid::new_v4().to_string(),
            name.to_string(),
            email.to_string(),
        );
        self.repository.save(&user).map_err(|e| e.to_string())?;
        Ok(user)
    }

    fn delete_user(&self, id: &str) -> Result<(), String> {
        self.repository.delete(id).map_err(|e| e.to_string())
    }
}
```

---

## Phase 6: Surface Layer

Translates user-facing inputs into actions, delegating execution to the Agent orchestrator.

```rust
// crates/user/src/surface_user_command.rs
use shared::user::contract_user_aggregate::IUserAggregate;
use std::sync::Arc;

pub struct UserCommand {
    orchestrator: Arc<dyn IUserAggregate>,
}

impl UserCommand {
    pub fn new(orchestrator: Arc<dyn IUserAggregate>) -> Self {
        Self { orchestrator }
    }

    pub fn run(&self, args: &[String]) -> Result<String, String> {
        match args.first().map(|s| s.as_str()) {
            Some("get") => {
                let id = args.get(1).ok_or("Missing user ID")?;
                let user = self.orchestrator.get_user(id)?;
                Ok(format!("User: {} <{}>", user.name, user.email))
            }
            Some("create") => {
                let name = args.get(1).ok_or("Missing name")?;
                let email = args.get(2).ok_or("Missing email")?;
                let user = self.orchestrator.create_user(name, email)?;
                Ok(format!("Created user: {}", user.id))
            }
            _ => Err("Usage: user <get|create> [args...]".to_string()),
        }
    }
}
```

---

## Phase 7: Root Layer

Wires concrete implementations to contracts and bootstraps the system.

### Container

```rust
// crates/user/src/root_user_container.rs
use crate::agent_user_orchestrator::UserOrchestrator;
use crate::capabilities_user_checker::UserChecker;
use crate::capabilities_user_repository::UserRepository;
use shared::user::contract_user_aggregate::IUserAggregate;
use std::sync::Arc;

pub struct UserContainer {
    orchestrator: Arc<dyn IUserAggregate>,
}

impl UserContainer {
    pub fn new(db_path: &str) -> Self {
        let checker = Arc::new(UserChecker::new());
        let repository = Arc::new(UserRepository::new(db_path.to_string()));
        let orchestrator = Arc::new(UserOrchestrator::new(checker, repository));
        Self { orchestrator }
    }

    pub fn orchestrator(&self) -> Arc<dyn IUserAggregate> {
        self.orchestrator.clone()
    }
}
```

### Entry Point

```rust
// crates/root_cli_main_entry.rs
use user::root_user_container::UserContainer;
use user::surface_user_command::UserCommand;

fn main() {
    let container = UserContainer::new("data.db");
    let command = UserCommand::new(container.orchestrator());

    let args: Vec<String> = std::env::args().skip(1).collect();
    match command.run(&args) {
        Ok(output) => println!("{}", output),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

---

## Phase 8: Verify

```bash
lint-arwaky-cli check your-project/
cargo test --workspace
cargo fmt --all && cargo clippy --all-targets -- -D warnings
```

---

## File Naming Reference

| Layer        | Pattern                              | Example                        |
| ------------ | ------------------------------------ | ------------------------------ |
| taxonomy     | `taxonomy_<concept>_<suffix>.rs`     | `taxonomy_user_vo.rs`          |
| contract     | `contract_<concept>_<suffix>.rs`     | `contract_user_protocol.rs`    |
| utility      | `utility_<concept>_<suffix>.rs`      | `utility_user_hasher.rs`       |
| capabilities | `capabilities_<concept>_<suffix>.rs` | `capabilities_user_checker.rs` |
| agent        | `agent_<concept>_orchestrator.rs`    | `agent_user_orchestrator.rs`   |
| surface      | `surface_<concept>_<suffix>.rs`      | `surface_user_command.rs`      |
| root         | `root_<concept>_<suffix>.rs`         | `root_user_container.rs`       |

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
