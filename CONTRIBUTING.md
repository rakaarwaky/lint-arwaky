# Contributing to Lint Arwaky

> This guide covers everything you need to start contributing to the Rust implementation in `crates/`.

## Why Contribute

| Aspect                     | Benefit                                                        |
| -------------------------- | -------------------------------------------------------------- |
| **Real-world impact**      | Your code powers the same rule engine that audits this project |
| **Skill development**      | Practice Rust, async/tokio, MCP, and 6-layer architecture      |
| **Open-source experience** | Build portfolio with a self-auditing codebase                  |
| **Community**              | Join a project where every PR is checked by the rules it adds  |
| **Learning opportunity**   | Study a codebase that passes its own architecture linter       |

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Setup](#setup)
- [Architecture](#architecture)
- [How to Add an Adapter](#how-to-add-an-adapter)
- [How to Add a CLI Command](#how-to-add-a-cli-command)
- [How to Add an MCP Tool](#how-to-add-an-mcp-tool)
- [Testing](#testing)
- [Code Style](#code-style)
- [Pull Request Process](#pull-request-process)

---

## Prerequisites

- **Rust** >= 1.70 (edition 2021)
- **Cargo** (bundled with Rust)
- **Git**
- Familiarity with:
  - Rust async/await and `tokio`
  - `clap` derive macros
  - `mcp-sdk-rs` (or willingness to read the JSON-RPC 2.0 spec)

> Optional: `rustup` for toolchain management, `cargo-watch` for development.

---

## Setup

```bash
# Clone
git clone https://github.com/rakaarwaky/lint-arwaky.git
cd lint-arwaky

# Build everything
cargo build --release

# Run the CLI
./target/release/lint-arwaky-cli version
# Expected: Lint Arwaky v1.10.14 (AES Semantic Builder)

# Run the MCP server in a separate terminal
./target/release/lint-arwaky-mcp
# Expected: "Listening on stdin/stdout (JSON-RPC 2.0)"

# Self-lint the project
./target/release/lint-arwaky-cli check .
# Scans `crates/` under the same AES rules the project enforces.
```

For development without the release profile:

```bash
cargo run --bin lint-arwaky-cli -- check .
cargo run --bin lint-arwaky-mcp
```

---

## Architecture

### 6-Layer Model

The codebase is organized into **feature crates** (vertical slicing) in the `crates/` directory. Within each crate, files from different layers coexist and are identified strictly by their **file prefix**:

- `taxonomy_` — Value Objects (VOs), entities, constants, errors, events, utilities, helpers
- `contract_` — Interface layer: ports (I\*), protocols, aggregates
- `capabilities_` — Thinking layer: checkers, analyzers, processors, evaluators, resolvers, etc.
- `infrastructure_` — Toolbox layer: linter adapters, providers, scanners
- `agent_` — Wiring layer: orchestrators
- `surface_` — Interface layer: CLI commands, MCP handlers
- `root_` — Composition containers, main entries (root layer)

### Dependency Rules

The import boundaries are enforced by the `arch_import_checker` capability and AES201:

```
agent          --> taxonomy, contract, infrastructure, capabilities  OK
surfaces       --> taxonomy, contract(io)                            OK
surfaces       --> infrastructure, capabilities                      NO  (AES201)
capabilities   --> taxonomy, contract(protocol)                      OK
capabilities   --> infrastructure, surfaces, agent                   NO  (AES201)
infrastructure --> taxonomy, contract(port)                          OK
infrastructure --> capabilities, surfaces, agent                     NO  (AES201)
contract       --> taxonomy                                          OK
contract       --> agent, capabilities, infrastructure, surfaces     NO  (AES201)
taxonomy       --> taxonomy                                          OK
taxonomy       --> agent, capabilities, infrastructure, surfaces, contract  NO  (AES201)
```

The Surface → Agent edge is indirect: surfaces hold `Arc<dyn ServiceContainerAggregate>` and call into it via the trait only.

### Key Interfaces & Mandatory Inheritance

To prevent architectural bypasses, every logic file **must** define a struct that implements the appropriate contract (AES303):

| Layer              | Allowed Suffixes                                                                                                                                                                                                                                                                                                                                                                                                                                                      | Trait / Example File                         |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------- |
| **Agent**          | `_orchestrator`                                                                                                                                                                                                                                                                                                                                                                                                                                                       | `analysis_execution_orchestrator.rs`         |
| **Capabilities**   | `_analyzer`, `_checker`, `_processor`, `_evaluator`, `_resolver`, `_validator`, `_formatter`, `_executor`, `_transformer`, `_calculator`, `_builder`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor`, `_actions`                                                                                                                          | `architecture_compliance_analyzer.rs`        |
| **Infrastructure** | `_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_util`, `_system`, `_repository`, `_cache`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer` | `python_ruff_adapter.rs`                     |
| **Surfaces**       | `_command`, `_controller`, `_page`, `_view`, `_component`, `_router`, `_layout`, `_hook`, `_store`, `_action`, `_screen`                                                                                                                                                                                                                                                                                                                                              | `cli_check_command.rs`                       |
| **Taxonomy**       | `_vo`, `_entity`, `_event`, `_error`, `_constant`, `_utility`, `_helper`                                                                                                                                                                                                                                                                                                                                                                                              | `lint_score_vo.rs`, `lint_score_constant.rs` |

**AES401 (taxonomy role)**: A `_constant` file may contain only `pub const` / `pub static` declarations. Any `struct`, `enum`, `fn`, `impl`, `mod`, or `pub use` in such a file triggers AES401.

---

## How to Add an Adapter

### 1. Pick the right layer

Linter adapters live in `crates/external-lint/src/` and follow the `infrastructure_[language]_[tool]_adapter.rs` naming pattern (or `_scanner.rs` for AST-level tools).

### 2. Create the adapter file

For example, `crates/external-lint/src/infrastructure_rs_clippy_adapter.rs`:

```rust
//! Adapter wrapping the Clippy linter.
use async_trait::async_trait;
use std::path::Path;
use std::process::Command;

use code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_result_vo::{LintResult, LintResultList};

pub struct RustClippyAdapter;

impl RustClippyAdapter {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl ILinterAdapterPort for RustClippyAdapter {
    fn name(&self) -> AdapterName { AdapterName::new("clippy") }

    async fn scan(&self, path: &Path) -> LintResultList {
        let output = Command::new("cargo")
            .args(["clippy", "--message-format=json", "--manifest-path"])
            .arg(path)
            .output();
        // parse output into LintResultList...
        LintResultList::default()
    }
}
```

### 3. Register in the DI container

Edit `crates/composition_root.rs` (or `crates/root_composition_container.rs`) and add the adapter registration:

```rust
use crate::external_lint::infrastructure_rs_clippy_adapter::RustClippyAdapter;

// inside CompositionRoot / container building logic
let clippy = Arc::new(RustClippyAdapter::new());
```

### 4. Add a test

Create `#[cfg(test)] mod tests` inside the same file (Rust convention: keep unit tests in-file) or a sibling test in `tests/`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clippy_name() {
        let a = RustClippyAdapter::new();
        assert_eq!(a.name().value, "clippy");
    }
}
```

````

### 5. Run the tests

```bash
cargo test --bin lint-arwaky-cli
cargo test --lib
````

---

## How to Add a CLI Command

### 1. Pick the right module

CLI surface modules live in `crates/cli-commands/src/` and follow the `surface_<group>_command.rs` pattern:

| Module                           | Purpose                                                       |
| -------------------------------- | ------------------------------------------------------------- |
| `surface_core_command.rs`        | `check`, `scan`, `fix`, `ci`, `version`, `adapters`, `config` |
| `surface_setup_command.rs`       | `setup init/doctor/install/mcp-config/hermes`                 |
| `surface_watch_command.rs`       | `watch`                                                       |
| `surface_maintenance_command.rs` | `install-hook`, `uninstall-hook`                              |
| `surface_git_command.rs`         | `git-diff`, `multi-project`                                   |

### 2. Add the subcommand

In `crates/cli-commands/src/surface_core_command.rs`:

```rust
#[derive(Subcommand, Debug)]
pub enum Commands {
    // ... existing variants ...
    /// My new command
    MyCommand {
        /// Optional path argument
        path: Option<String>,
    },
}
```

In `crates/root_cli_main_entry.rs` add a `match` arm:

```rust
Commands::MyCommand { path } => {
    let target = path.unwrap_or_else(|| ".".to_string());
    // delegate to an agent orchestrator using CompositionRoot
    let container = Arc::new(CompositionRoot::new(default_aes_config()));
    container.get_my_orchestrator().run(&target);
    ExitCode::SUCCESS
}
```

### 3. Register in COMMAND_CATALOG

`crates/shared/src/cli-commands/taxonomy_catalog_constant.rs` is the single source of truth. Add a row:

```rust
pub static COMMAND_CATALOG: &[(&str, &str, &str)] = &[
    // ... existing rows ...
    (
        "my-command",
        "Brief description for list_commands",
        "lint-arwaky-cli my-command /path",
    ),
];
```

### 4. Run and verify

```bash
cargo run --bin lint-arwaky-cli -- my-command .
cargo run --bin lint-arwaky-cli -- list_commands core
```

---

## How to Add an MCP Tool

### 1. Implement the tool

In `crates/mcp-server/src/surface_tools_command.rs`, add a new function:

```rust
pub async fn my_tool_handler(args: Option<serde_json::Value>) -> serde_json::Value {
    serde_json::json!({ "ok": true, "args": args })
}
```

### 2. Register the tool schema

In `crates/root_mcp_main_entry.rs` (or `crates/mcp-server/src/surface_tools_controller.rs` as appropriate), add an entry to the `tools/list` array:

```rust
{
    "name": "my_tool",
    "description": "What the tool does (non-empty string).",
    "inputSchema": {
        "type": "object",
        "properties": {
            "arg": { "type": "string" }
        }
    }
}
```

### 3. Wire the dispatch arm

In the `match tool_name` block:

```rust
"my_tool" => surface_tools_command::my_tool_handler(arguments).await,
```

### 4. Smoke-test

```bash
echo '{"jsonrpc":"2.0","id":1,"method":"tools/list"}' \
  | ./target/release/lint-arwaky-mcp

echo '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"my_tool","arguments":{"arg":"hello"}}}' \
  | ./target/release/lint-arwaky-mcp
```

---

## Testing

### Run all tests

```bash
cargo test --workspace
```

### Run a single test by name

```bash
cargo test --lib arch_compliance
```

### Test organization

Tests live next to the code they exercise (Rust convention) within each feature crate:

```
crates/
  code-analysis/src/capabilities_cycle_analyzer.rs
    └── #[cfg(test)] mod tests { ... }
  contract/...
  ...
```

For integration tests, use the `tests/` directory at the workspace root.

### Rules

- Every public function should have at least one test.
- Mock external processes (`std::process::Command`) at the trait boundary.
- Test both success and failure paths.
- Use `#[tokio::test]` for async tests.

---

## Code Style

### Formatting

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

### Conventions

- **Naming**: Strict layer prefix snake_case filename with a layer-role suffix (e.g., `capabilities_naming_checker.rs`).
- **Structs**: Mandatory struct definitions in all logic modules. Free-standing functions are forbidden in the capability/agent/infrastructure layers (AES303).
- **Lines**: Files must stay within the AES301/AES302 bounds (configurable; default 10-500 lines).
- **Score**: `cargo run --bin lint-arwaky-cli -- check .` should report 0 critical findings on a clean PR.
- **Bypasses**: `#[allow(...)]` on lint rules, `noqa`-style comments, and `type: ignore`-style suppressions are forbidden by AES304 and will fail CI.

### AES401 Taxonomy Role — Primitive Type Policy

The project applies a **layer-granular** primitive enforcement strategy:

| Layer                            | `no_primitives` | Policy                                                                       |
| -------------------------------- | --------------- | ---------------------------------------------------------------------------- |
| `contract`                       | `true`          | All port/protocol/aggregate signatures must use taxonomy Value Objects       |
| `taxonomy(entity\|error\|event)` | `true`          | All entity/error/event attributes must use Value Objects                     |
| `taxonomy(vo)`                   | `false`         | VO internals may use primitives as underlying storage                        |
| `taxonomy(constant)`             | `false`         | Constants are primitives by definition; must contain ONLY constants (AES401) |
| `infrastructure`                 | `false`         | Adapters may use primitive types as supporting/local types                   |
| `capabilities`                   | `false`         | Capability implementations may use primitive types internally                |
| `surfaces`                       | `false`         | Surface/CLI handlers may use primitive types for I/O parsing                 |

**Rationale**: Strict Value Objects in implementation/adapter layers create unnecessary boxing overhead and conflict with third-party APIs (`clap`, `tokio`, `mcp-sdk-rs`). Domain contracts and taxonomy definitions remain strictly typed to prevent boundary leakage.

---

## Pull Request Process

### Before Submitting

1. **Run tests**: `cargo test --workspace`
2. **Run self-lint**: `cargo run --bin lint-arwaky-cli -- check .` — no CRITICAL findings.
3. **Format & lint**: `cargo fmt --all && cargo clippy --all-targets -- -D warnings`
4. **Update docs**: Ensure `README.md`, `SKILL.md`, `PRD.md`, and `CHANGELOG.md` reflect your changes.

### PR Description Template

```markdown
## What

Brief description of what this PR does.

## Why

Why is this change needed?

## How

How does it work? Any design decisions?

## Testing

How was it tested? What test cases were added?

## Checklist

- [ ] `cargo test --workspace` passes
- [ ] `cargo run --bin lint-arwaky-cli -- check .` reports 0 CRITICAL findings
- [ ] `cargo fmt --all` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] Docs updated if needed
```

### Review Criteria

- Code follows AES rules (no cross-layer violations)
- Tests cover both happy path and error cases
- No hardcoded paths or environment assumptions
- Subprocess calls use absolute paths to executables
- Error messages are actionable (tell the user what to do)

---

## Questions?

Open an issue on GitHub or contact the maintainer.
