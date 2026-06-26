# Developer Guide

> **For developers who want to understand, modify, or extend Lint Arwaky.**
> This guide explains **how** the code works and **why** it's designed this way.

---

## Table of Contents

1. [Architecture at a Glance](#1-architecture-at-a-glance)
2. [How a Lint Check Works (End-to-End)](#2-how-a-lint-check-works-end-to-end)
3. [The 7 Layers — File Prefix Convention](#3-the-7-layers--file-prefix-convention)
4. [Key Patterns Explained](#4-key-patterns-explained)
5. [Workspace Crate Map](#5-workspace-crate-map)
6. [Entry Points & Dispatch](#6-entry-points--dispatch)
7. [Adding a New AES Rule](#7-adding-a-new-aes-rule)
8. [Adding a New CLI Command](#8-adding-a-new-cli-command)
9. [Testing Guide](#9-testing-guide)
10. [Common Pitfalls](#10-common-pitfalls)

---

## 1. Architecture at a Glance

```
User / AI Agent
       │
       ▼
┌─────────────────────────────────────────┐
│  Surface Layer                          │
│  CLI (clap) │ MCP (rmcp) │ TUI (ratatui)│
└──────────────┬──────────────────────────┘
               │ calls
               ▼
┌─────────────────────────────────────────┐
│  Agent Layer                            │
│  Orchestrators (one per feature crate)  │
│  ImportOrchestrator, NamingOrchestrator │
└──────────────┬──────────────────────────┘
               │ delegates to
               ▼
┌─────────────────────────────────────────┐
│  Capabilities + Infrastructure (peers)  │
│  Checkers, Analyzers  ←→  Adapters      │
│  (business logic)        (tool wrappers)│
└──────────────┬──────────────────────────┘
               │ depends on
               ▼
┌─────────────────────────────────────────┐
│  Contract Layer                         │
│  Ports, Protocols, Aggregates (traits)  │
└──────────────┬──────────────────────────┘
               │ depends on
               ▼
┌─────────────────────────────────────────┐
│  Taxonomy Layer                         │
│  VOs, Entities, Errors, Constants       │
│  (pure data, zero dependencies)         │
└─────────────────────────────────────────┘
```

**Dependency rule:** Arrows only point **down**. A layer may only depend on layers **below** it. `Capabilities` and `Infrastructure` are **horizontal peers** — they never import from each other.

---

## 2. How a Lint Check Works (End-to-End)

Here's the exact code path when you run `lint-arwaky-cli check .`:

### Step 1 — CLI Entry (`root_cli_main_entry.rs`)

```rust
// main() parses args via clap, then:
let container = ImportContainer::new_default();
let orchestrator = container.orchestrator();  // Arc<dyn IImportRunnerAggregate>
let results = rt.block_on(orchestrator.run_audit(&FilePath::new(".")));
```

The CLI creates **all containers** upfront, then dispatches based on the subcommand.

### Step 2 — Orchestrator (`agent_import_orchestrator.rs`)

```rust
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let files = self.collect_files(target);          // walk + filter
        self.mandatory.check_mandatory_imports(...);     // AES202
        self.forbidden.check_forbidden_imports(...);     // AES201
        self.intent.check_mandatory_imports(...);        // AES204
        self.unused.check_unused_imports(...);           // AES203
        self.cycle.check_cycles(...);                    // AES205
    }
}
```

The orchestrator **composes** checkers — it never contains business logic itself.

### Step 3 — Checker (`capabilities_import_mandatory_checker.rs`)

```rust
impl IArchImportProtocol for ArchImportMandatoryChecker {
    fn check_mandatory_imports(&self, file, content, definition, violations) {
        if is_barrel_file(file) { return; }           // skip __init__.py, mod.rs
        if is_exception(file) { return; }             // skip configured exceptions
        for rule in definition.rules() {
            if !content.contains(rule.import) {
                violations.push(LintResult::new_arch(file, line, "AES202", ...));
            }
        }
    }
}
```

Checkers are **pure functions**: input → violations. No side effects, no I/O.

### Step 4 — Result (`taxonomy_lint_result_vo.rs`)

```rust
pub struct LintResult {
    pub file: String,
    pub line: usize,
    pub code: String,      // "AES202"
    pub severity: Severity,
    pub message: String,
}
```

`LintResult` is a **Value Object** — immutable, validated, serializable.

### Step 5 — Format & Display

Results flow back to the surface layer where they're formatted as:
- **CLI:** colored terminal output
- **MCP:** JSON for AI agents
- **TUI:** ratatui table widgets

---

## 3. The 7 Layers — File Prefix Convention

Every `.rs` file in a feature crate declares its layer through its **filename prefix**:

| Prefix | Layer | What it contains | Example |
|--------|-------|-----------------|---------|
| `taxonomy_` | Domain Models | Value objects, entities, errors, constants | `taxonomy_file_path_vo.rs` |
| `contract_` | Interfaces | Trait definitions (ports, protocols, aggregates) | `contract_import_runner_aggregate.rs` |
| `capabilities_` | Business Logic | Checkers, analyzers, processors | `capabilities_import_mandatory_checker.rs` |
| `infrastructure_` | Tool Wrappers | Adapters for external tools (Clippy, Ruff, etc.) | `infrastructure_rs_clippy_adapter.rs` |
| `agent_` | Orchestration | Coordinates capabilities + infrastructure | `agent_import_orchestrator.rs` |
| `surface_` | Entry Points | CLI commands, MCP handlers, TUI views | `surface_check_command.rs` |
| `root_` | Wiring | DI containers, binary entry points | `root_import_rules_container.rs` |

### Why file prefixes instead of directories?

The AES architecture uses **vertical slicing** — each feature crate contains all 7 layers. File prefixes enforce the layer boundaries **structurally**: the AES linter itself checks that `taxonomy_*` files don't import from `capabilities_*` files.

### Naming convention

```
{layer}_{feature}_{type}.rs

Examples:
  taxonomy_file_path_vo.rs         → taxonomy layer, file_path feature, value object
  contract_import_runner_aggregate → contract layer, import feature, aggregate trait
  capabilities_mandatory_checker   → capabilities layer, mandatory feature, checker impl
  infrastructure_clippy_adapter    → infrastructure layer, clippy feature, adapter impl
  agent_import_orchestrator        → agent layer, import feature, orchestrator
  surface_check_command            → surface layer, check feature, CLI command
  root_import_container            → root layer, import feature, DI container
```

---

## 4. Key Patterns Explained

### 4.1 Dependency Injection (DI) Containers

**File:** `root_*_container.rs` in each feature crate

**What:** Manual DI wiring using `Arc<dyn Trait>`. No framework.

**Why:** Keeps dependencies explicit and testable. Each container is a factory that creates concrete implementations and exposes them as trait objects.

```rust
// root_import_rules_container.rs
pub struct ImportContainer {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    // ...
}

impl ImportContainer {
    pub fn new_default() -> Self {
        Self {
            mandatory: Arc::new(ArchImportMandatoryChecker::new()),
            forbidden: Arc::new(ArchImportForbiddenChecker::new()),
            // ...
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        Arc::new(ImportOrchestrator::new(
            self.mandatory.clone(),
            self.forbidden.clone(),
            // ...
        ))
    }
}
```

### 4.2 Orchestrators (Agent Layer)

**File:** `agent_*_orchestrator.rs`

**What:** Coordinates checkers/adapters without containing business logic.

**Why:** Single responsibility — orchestration is the only job. Checkers remain independently testable.

```rust
pub struct ImportOrchestrator {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    // ...
}

impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        // 1. Collect files
        // 2. Run each checker
        // 3. Aggregate results
    }
}
```

### 4.3 Checkers (Capabilities Layer)

**File:** `capabilities_*_checker.rs`

**What:** Pure business logic — input in, violations out.

**Why:** Testable in isolation, no I/O, no side effects.

```rust
pub struct ArchImportMandatoryChecker;

impl IArchImportProtocol for ArchImportMandatoryChecker {
    fn check_mandatory_imports(
        &self,
        file: &str,
        content: &str,
        definition: &ImportDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        // Pure logic: scan content, push violations
    }
}
```

### 4.4 Value Objects (Taxonomy Layer)

**File:** `taxonomy_*_vo.rs`

**What:** Validated, immutable data types.

**Why:** Type safety. `FilePath` can't be a raw `String`. Validation happens at construction.

```rust
pub struct FilePath { value: String }

impl FilePath {
    pub fn new(path: &str) -> Result<Self, LintError> {
        // Validate non-empty
        // Normalize separators
        // Strip trailing slashes
        Ok(Self { value: normalized })
    }
}
```

### 4.5 Contracts (Contract Layer)

**File:** `contract_*_aggregate.rs`, `contract_*_protocol.rs`, `contract_*_port.rs`

**What:** Trait definitions that decouple layers.

**Why:** Upper layers depend on abstractions, not implementations. Enables swapping implementations without changing callers.

```rust
#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

### 4.6 Adapters (Infrastructure Layer)

**File:** `infrastructure_*_adapter.rs`

**What:** Wrappers around external tools (Clippy, Ruff, ESLint, etc.).

**Why:** Isolate third-party tool integration. If a tool's CLI changes, only the adapter changes.

```rust
pub struct ClippyAdapter;

impl IExternalLintProtocol for ClippyAdapter {
    fn run(&self, target: &str) -> Vec<LintResult> {
        // Execute `cargo clippy`, parse output
    }
}
```

---

## 5. Workspace Crate Map

```
lint-arwaky (workspace)
│
├── shared/                    ← Foundation: all taxonomy_* + contract_*
│   └── 168 files              ← VOs, entities, traits used everywhere
│
├── import-rules/              ← AES201–AES205: import compliance
├── naming-rules/              ← AES101–AES102: naming conventions
├── code-analysis/             ← AES301–AES305: code quality
├── role-rules/                ← AES401–AES406: layer-role violations
├── orphan-detector/           ← AES501–AES506: dead code detection
│
├── auto-fix/                  ← Automatic safe code fixes
├── config-system/             ← YAML config loading & multi-project
├── external-lint/             ← Clippy, Ruff, ESLint, etc. adapters
│
├── cli-commands/              ← CLI surface (check, scan, fix, ci, ...)
├── mcp-server/                ← MCP server surface (5 JSON-RPC tools)
├── tui/                       ← TUI surface (ratatui 3-panel browser)
│
├── file-watch/                ← inotify-based file watching
├── git-hooks/                 ← pre-commit hook install/uninstall
├── project-setup/             ← init, doctor, mcp-config
├── maintenance/               ← env doctor, security scans
│
├── lib.rs                     ← Re-exports all crates for binaries
├── root_cli_main_entry.rs     ← CLI binary entry
├── root_mcp_main_entry.rs     ← MCP binary entry
└── root_tui_main_entry.rs     ← TUI binary entry
```

### What goes where?

| You want to... | Put code in... |
|----------------|---------------|
| Add a new value type (VO) | `shared/src/common/taxonomy_*_vo.rs` |
| Add a new trait interface | `shared/src/{feature}/contract_*_aggregate.rs` |
| Add a new lint checker | `crates/{feature}/capabilities_*_checker.rs` |
| Add a new external tool | `crates/external-lint/infrastructure_*_adapter.rs` |
| Add a new CLI command | `crates/cli-commands/src/surface_*_command.rs` |
| Add a new MCP tool | `crates/mcp-server/src/` |
| Wire up new dependencies | `crates/{feature}/root_*_container.rs` |

---

## 6. Entry Points & Dispatch

### CLI (`root_cli_main_entry.rs`)

```
main()
  ├── parse CLI args (clap)
  ├── create all containers (ImportContainer, RoleContainer, ...)
  ├── match command:
  │     ├── Check   → handle_check()   → runs all linters
  │     ├── Scan    → handle_scan()    → multi-project orchestration
  │     ├── Fix     → handle_fix()     → auto-fix processor
  │     ├── CI      → handle_ci()      → CI exit codes
  │     ├── Orphan  → orphan check
  │     ├── Doctor  → maintenance doctor
  │     └── ...     → other commands
  └── exit
```

### MCP Server (`root_mcp_main_entry.rs`)

```
main() [async]
  ├── create McpContainer
  ├── create McpServerOrchestrator
  ├── create LintArwakyMcpServer
  └── serve via stdio (JSON-RPC 2.0)
        ├── execute_command  → dispatches to linters
        ├── list_commands    → returns available tools
        ├── command_schema   → returns JSON schema
        ├── read_skill       → returns SKILL.md content
        └── health_check     → returns server status
```

### TUI (`root_tui_main_entry.rs`)

```
main()
  └── TuiContainer::run()
        ├── initialize terminal (crossterm)
        ├── 3-panel layout (ratatui)
        │     ├── Left:   file browser
        │     ├── Center: file content
        │     └── Right:  lint results
        └── event loop (keyboard navigation)
```

---

## 7. Adding a New AES Rule

### Step 1: Define the rule in shared

Add the rule code and configuration VO in `shared/src/`:

```rust
// shared/src/{feature}/taxonomy_rule_vo.rs
pub struct MyNewRule {
    pub code: String,        // "AES306"
    pub severity: Severity,
    pub config: RuleConfig,
}
```

### Step 2: Add the contract trait

```rust
// shared/src/{feature}/contract_my_protocol.rs
pub trait IMyNewProtocol: Send + Sync {
    fn check(&self, file: &str, content: &str, violations: &mut Vec<LintResult>);
}
```

### Step 3: Implement the checker

```rust
// crates/{feature}/capabilities_my_checker.rs
pub struct MyNewChecker;

impl IMyNewProtocol for MyNewChecker {
    fn check(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // Your checking logic here
        if condition_violated {
            violations.push(LintResult::new_arch(
                file, line, "AES306", Severity::Warning,
                "Description of violation"
            ));
        }
    }
}
```

### Step 4: Wire it in the container

```rust
// crates/{feature}/root_*_container.rs
pub struct FeatureContainer {
    my_checker: Arc<dyn IMyNewProtocol>,
    // ...
}

impl FeatureContainer {
    pub fn new_default() -> Self {
        Self {
            my_checker: Arc::new(MyNewChecker),
            // ...
        }
    }
}
```

### Step 5: Call it from the orchestrator

```rust
// crates/{feature}/agent_*_orchestrator.rs
impl IMyAggregate for MyOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut violations = Vec::new();
        self.my_checker.check(file, content, &mut violations);
        // ...
    }
}
```

### Step 6: Add config

```yaml
# lint_arwaky.config.rust.yaml
rules:
  AES306:
    enabled: true
    severity: warning
    config:
      # rule-specific settings
```

### Step 7: Test

Add test workspace violations in `test-workspaces/` and verify they're caught.

---

## 8. Adding a New CLI Command

### Step 1: Add variant to `Commands` enum

```rust
// shared/src/cli-commands/taxonomy_commands_vo.rs
pub enum Commands {
    // ... existing commands
    MyNewCommand {
        #[arg(required = true)]
        path: String,
    },
}
```

### Step 2: Create the surface handler

```rust
// crates/cli-commands/src/surface_my_new_command.rs
pub fn handle_my_new_command(args: &MyNewCommandArgs, context: &CheckContext) {
    // Implement command logic
}
```

### Step 3: Dispatch in main entry

```rust
// crates/root_cli_main_entry.rs
Commands::MyNewCommand { path } => {
    handle_my_new_command(&path, &context);
}
```

---

## 9. Testing Guide

### Running tests

```bash
# All tests
cargo test --workspace

# Single crate
cargo test -p import_rules-lint-arwaky

# Single test by name
cargo test --lib import_mandatory

# With output
cargo test -- --nocapture
```

### Test workspaces

The `test-workspaces/` directory contains **intentional violations** for validation:

```bash
# Scan test workspaces
cargo run --bin lint-arwaky-cli -- scan test-workspaces

# Expected: >= 2000 violations, >= 24 unique AES codes per language
```

### Self-lint

The project lints itself:

```bash
cargo run --bin lint-arwaky-cli -- check .
```

---

## 10. Common Pitfalls

### 1. Importing from wrong layer

```rust
// WRONG: taxonomy importing from capabilities
use crate::capabilities_mandatory_checker::ArchImportMandatoryChecker;

// RIGHT: taxonomy should only import from taxonomy
use crate::taxonomy_file_path_vo::FilePath;
```

### 2. Putting business logic in orchestrator

```rust
// WRONG: orchestrator contains checking logic
impl ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        for line in content.lines() {
            if line.contains("import") { ... }  // Business logic in orchestrator!
        }
    }
}

// RIGHT: delegate to checker
impl ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        self.mandatory.check_mandatory_imports(file, content, &def, &mut violations);
    }
}
```

### 3. Using concrete types instead of trait objects

```rust
// WRONG: depends on concrete implementation
pub struct MyOrchestrator {
    checker: ArchImportMandatoryChecker,
}

// RIGHT: depends on contract trait
pub struct MyOrchestrator {
    checker: Arc<dyn IArchImportProtocol>,
}
```

### 4. Forgetting barrel file skip

```rust
// Always skip barrel files in checkers
fn check(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
    if file.ends_with("__init__.py") || file.ends_with("mod.rs") {
        return;
    }
    // ... actual checking
}
```

### 5. Not normalizing file paths

```rust
// Use FilePath VO instead of raw strings
let path = FilePath::new(raw_path)?;  // Normalizes separators, validates
```

---

## Quick Reference: File Locations

| What | Where |
|------|-------|
| All shared types | `crates/shared/src/` |
| Feature crate source | `crates/{feature}/src/` |
| CLI commands | `crates/cli-commands/src/surface_*_command.rs` |
| MCP tools | `crates/mcp-server/src/` |
| TUI views | `crates/tui/src/surface_*_view.rs` |
| Config files | `lint_arwaky.config.{rust,python,javascript}.yaml` |
| Test workspaces | `test-workspaces/` |
| CI workflows | `.github/workflows/` |
| Build scripts | `scripts/` |

---

## Further Reading

| Topic | Document |
|-------|----------|
| AES 7-layer architecture | [ARCHITECTURE.md](../ARCHITECTURE.md) |
| 24 AES rules catalog | [docs/rules/RULES_AES.md](rules/RULES_AES.md) |
| Product requirements | [PRD.md](../PRD.md) |
| AI agent skill | [SKILL.md](../SKILL.md) |
| Contributing guide | [CONTRIBUTING.md](../CONTRIBUTING.md) |
| Deployment guide | [DEPLOY.md](../DEPLOY.md) |
| Test plan | [TEST.md](../TEST.md) |
