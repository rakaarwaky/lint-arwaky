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
│   │       │   ├── contract_system_port.rs
│   │       │   ├── taxonomy_path_vo.rs
│   │       │   └── ...
│   │       ├── tui/             ← shared types for tui feature
│   │       │   ├── mod.rs
│   │       │   ├── taxonomy_state_vo.rs
│   │       │   ├── taxonomy_tui_event.rs
│   │       │   └── ...
│   │       ├── import-rules/    ← shared types for import-rules feature
│   │       │   ├── mod.rs
│   │       │   ├── taxonomy_import_rule_vo.rs
│   │       │   └── ...
│   │       ├── code-analysis/   ← shared types for code-analysis feature
│   │       └── ...              ← one subfolder per feature crate
│   │
│   ├── user/               ← feature crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── taxonomy_user_vo.rs
│   │       ├── contract_user_port.rs
│   │       ├── capabilities_user_checker.rs
│   │       ├── infrastructure_user_adapter.rs
│   │       ├── agent_user_orchestrator.rs
│   │       ├── surface_user_command.rs
│   │       ├── root_user_container.rs
│   │       └── lib.rs
│   ├── root_cli_main_entry.rs   ← CLI binary (file, NOT directory)
│   ├── root_mcp_main_entry.rs   ← MCP server binary
│   ├── root_tui_main_entry.rs   ← TUI binary
│   └── lib.rs
└── Cargo.lock
```

**Key rules:**

- All 7 layers coexist in each feature crate, differentiated by file prefix.
- Entry points (`root_*_entry.rs`) are files inside `crates/`, not separate directories.
- `crates/shared/` has **subfolders matching each feature crate** (e.g. `shared/src/tui/` for tui types, `shared/src/import-rules/` for import-rules types).
- `crates/shared/src/common/` holds types shared across ALL features (path VOs, system ports, error types).

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
lint-arwaky-cli scan your-project/ | grep AES205
```

- Violations < 10 → full migration in one session
- Violations 10-50 → phased migration
- Violations > 50 → start with taxonomy only

---

## Phase 1: Taxonomy Layer

### Step 1.1: Identify Domain Types

```bash
grep -rn "pub struct\|pub enum" your-project/crates/*/src/ | grep -v test | grep -v mod.rs
```

### Step 1.2: Create Value Objects

**Before:** `crates/user/src/user.rs` (struct + logic mixed)
**After:** `crates/user/src/taxonomy_user_vo.rs` (pure data)

```rust
// crates/user/src/taxonomy_user_vo.rs
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
// crates/user/src/taxonomy_user_constant.rs
pub const MAX_RETRY_COUNT: u32 = 3;
pub const DEFAULT_TIMEOUT_MS: u64 = 5000;
```

### Step 1.4: Create Error Types

```rust
// crates/user/src/taxonomy_user_error.rs
#[derive(Debug, thiserror::Error)]
pub enum UserError {
    #[error("User not found: {0}")]
    NotFound(String),
    #[error("Invalid email: {0}")]
    InvalidEmail(String),
}
```

### Step 1.5: Register in lib.rs

```rust
// crates/user/src/lib.rs
pub mod taxonomy_user_vo;
pub mod taxonomy_user_error;
pub mod taxonomy_user_constant;
```

---

## Phase 2: Contract Layer

### Step 2.1: Identify Outbound Dependencies

```bash
grep -rn "std::fs\|std::net\|reqwest\|tokio::fs\|sqlx\|redis" your-project/crates/*/src/
```

### Step 2.2: Create Ports (outbound interfaces)

```rust
// crates/user/src/contract_user_port.rs
use crate::taxonomy_user_vo::UserVO;

pub trait IUserPort {
    fn find_by_id(&self, id: &str) -> Result<Option<UserVO>, Box<dyn std::error::Error>>;
    fn save(&self, user: &UserVO) -> Result<(), Box<dyn std::error::Error>>;
    fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>>;
}
```

### Step 2.3: Create Protocols (inbound interfaces)

```rust
// crates/user/src/contract_user_protocol.rs
use crate::taxonomy_user_vo::UserVO;

pub trait IUserProtocol {
    fn get_user(&self, id: &str) -> Result<UserVO, String>;
    fn create_user(&self, name: &str, email: &str) -> Result<UserVO, String>;
}
```

### Step 2.4: Create Aggregates (facades)

```rust
// crates/user/src/contract_user_aggregate.rs
use crate::taxonomy_user_vo::UserVO;

pub trait IUserAggregate {
    fn get_user(&self, id: &str) -> Result<UserVO, String>;
    fn create_user(&self, name: &str, email: &str) -> Result<UserVO, String>;
    fn delete_user(&self, id: &str) -> Result<(), String>;
}
```

### Step 2.5: Register in lib.rs

```rust
pub mod contract_user_port;
pub mod contract_user_protocol;
pub mod contract_user_aggregate;
```

---

## Phase 3: Capabilities Layer

Business logic only. No infrastructure imports.

```bash
grep -rn "if \|match \|calc\|validate\|check" your-project/crates/*/src/
```

```rust
// crates/user/src/capabilities_user_checker.rs
use crate::contract_user_protocol::IUserProtocol;
use crate::taxonomy_user_vo::UserVO;

pub struct UserChecker {
    user_protocol: Box<dyn IUserProtocol>,
}

impl UserChecker {
    pub fn new(user_protocol: Box<dyn IUserProtocol>) -> Self {
        Self { user_protocol }
    }

    pub fn validate_email(&self, email: &str) -> bool {
        email.contains('@') && email.contains('.')
    }
}
```

---

## Phase 4: Infrastructure Layer

Each adapter implements a port.

```rust
// crates/user/src/infrastructure_user_adapter.rs
use crate::contract_user_port::IUserPort;
use crate::taxonomy_user_vo::UserVO;

pub struct UserAdapter {
    db_path: String,
}

impl UserAdapter {
    pub fn new(db_path: String) -> Self {
        Self { db_path }
    }
}

impl IUserPort for UserAdapter {
    fn find_by_id(&self, id: &str) -> Result<Option<UserVO>, Box<dyn std::error::Error>> {
        todo!("Implement database query")
    }
    fn save(&self, user: &UserVO) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Implement database insert/update")
    }
    fn delete(&self, id: &str) -> Result<(), Box<dyn std::error::Error>> {
        todo!("Implement database delete")
    }
}
```

---

## Phase 5: Agent Layer

Orchestrator coordinates capabilities and infrastructure.

```rust
// crates/user/src/agent_user_orchestrator.rs
use crate::contract_user_aggregate::IUserAggregate;
use crate::contract_user_port::IUserPort;
use crate::capabilities_user_checker::UserChecker;
use crate::taxonomy_user_vo::UserVO;

pub struct UserOrchestrator {
    checker: UserChecker,
    port: Box<dyn IUserPort>,
}

impl UserOrchestrator {
    pub fn new(checker: UserChecker, port: Box<dyn IUserPort>) -> Self {
        Self { checker, port }
    }
}

impl IUserAggregate for UserOrchestrator {
    fn get_user(&self, id: &str) -> Result<UserVO, String> {
        self.port.find_by_id(id).map_err(|e| e.to_string())?
            .ok_or_else(|| format!("User not found: {}", id))
    }

    fn create_user(&self, name: &str, email: &str) -> Result<UserVO, String> {
        if !self.checker.validate_email(email) {
            return Err("Invalid email".to_string());
        }
        let user = UserVO::new(
            uuid::Uuid::new_v4().to_string(),
            name.to_string(),
            email.to_string(),
        );
        self.port.save(&user).map_err(|e| e.to_string())?;
        Ok(user)
    }

    fn delete_user(&self, id: &str) -> Result<(), String> {
        self.port.delete(id).map_err(|e| e.to_string())
    }
}
```

---

## Phase 6: Surface Layer

CLI commands, API handlers. Delegates to orchestrator.

```rust
// crates/user/src/surface_user_command.rs
use crate::contract_user_aggregate::IUserAggregate;

pub struct UserCommand {
    orchestrator: Box<dyn IUserAggregate>,
}

impl UserCommand {
    pub fn new(orchestrator: Box<dyn IUserAggregate>) -> Self {
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
                Ok(format!("Created: {}", user.id))
            }
            _ => Err("Usage: user <get|create> [args...]".to_string()),
        }
    }
}
```

---

## Phase 7: Root Layer

DI container wires everything. Entry point bootstraps.

### Container (inside feature crate)

```rust
// crates/user/src/root_user_container.rs
use crate::infrastructure_user_adapter::UserAdapter;
use crate::capabilities_user_checker::UserChecker;
use crate::agent_user_orchestrator::UserOrchestrator;

pub struct UserContainer {
    orchestrator: UserOrchestrator,
}

impl UserContainer {
    pub fn new(db_path: &str) -> Self {
        let port = Box::new(UserAdapter::new(db_path.to_string()));
        let checker = UserChecker::new(/* inject protocol */);
        let orchestrator = UserOrchestrator::new(checker, port);
        Self { orchestrator }
    }

    pub fn orchestrator(&self) -> &UserOrchestrator {
        &self.orchestrator
    }
}
```

### Entry Point (inside crates/)

```rust
// crates/root_cli_main_entry.rs
use user::root_user_container::UserContainer;
use user::surface_user_command::UserCommand;

fn main() {
    let container = UserContainer::new("data.db");
    let command = UserCommand::new(Box::new(container.orchestrator().clone()));

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

| Layer          | Pattern                                | Example                          |
| -------------- | -------------------------------------- | -------------------------------- |
| taxonomy       | `taxonomy_<concept>_<suffix>.rs`       | `taxonomy_user_vo.rs`            |
| contract       | `contract_<concept>_<suffix>.rs`       | `contract_user_port.rs`          |
| capabilities   | `capabilities_<concept>_<suffix>.rs`   | `capabilities_user_checker.rs`   |
| infrastructure | `infrastructure_<concept>_<suffix>.rs` | `infrastructure_user_adapter.rs` |
| agent          | `agent_<concept>_orchestrator.rs`      | `agent_user_orchestrator.rs`     |
| surface        | `surface_<concept>_<suffix>.rs`        | `surface_user_command.rs`        |
| root           | `root_<concept>_<suffix>.rs`           | `root_user_container.rs`         |

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
| AES303     | Add struct/enum/trait definition                  |
| AES304     | Remove `#[allow]`, `unwrap()`, `panic!`           |
| AES401     | Move primitives to VO, constants to `_constant`   |
| AES402     | Replace primitive types with VO types in contract |
| AES403     | Implement protocol trait in capability            |
| AES404     | Implement port trait in infrastructure            |
| AES501-506 | Wire in container or remove dead code             |
