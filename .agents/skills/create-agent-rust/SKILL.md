---
name: create-agent-rust
description: "Create and validate Rust agent layer files following AES rules: orchestration-only, zero I/O, zero business logic, zero domain computation, 3-block structure, one impl struct per file, aggregate contracts, DI for service dependencies, and shared VOs for domain data."
version: 1.3.0
category: refactoring
tags:
  [
    rust,
    aes,
    agent,
    aggregate,
    structure,
    3-block-structure,
    di,
    orchestration,
    vo,
    error-handling,
    magic-constant,
    imports,
  ]
triggers:
  - "create agent rust"
  - "add agent rust"
  - "fix agent structure rust"
  - "create aggregate rust"
  - "agent missing aggregate rust"
  - "validate agent logic rust"
  - "audit agent rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-infrastructure-rust
  - create-taxonomy-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - module_logic_validator-rust
  - fix-agent-di
---

# create-agent-rust

## Purpose

Create and validate Rust **agent layer** files following clean architecture / AES rules.

An agent file must contain **orchestration / pipeline execution only**.

Agents coordinate:

- capabilities,
- infrastructure ports,
- shared taxonomy types,

but agents MUST NOT contain:

- I/O,
- business logic,
- domain rules,
- domain computation,
- domain data definitions.

Agents implement aggregate traits, follow the 3-Block Structure, and depend on abstractions via DI.

---

## Definition of Done

An agent file is considered valid when:

1. It contains exactly **ONE implementation struct**.
2. The struct implements exactly **ONE domain aggregate trait** in Block 2.
3. Block 2 contains **ONLY** the aggregate trait implementation.
4. Constructors, std trait impls, and private helpers are placed in Block 3.
5. The file contains **zero I/O**.
6. The file contains **zero business logic**.
7. The file contains **zero domain computation**.
8. The file does **not** define domain data structures locally.
9. Service dependencies use DI via `Arc<dyn Trait>`.
10. Value/configuration fields use shared VOs, not raw primitives.
11. Aggregate contract signatures use shared VOs for domain data.
12. Reusable, stateless, domain-agnostic functions are extracted to `*_utility.rs`.
13. Agent-specific helpers may remain inside the implementation file.
14. `cargo check -p <crate-name>` passes.

---

## Rules

### Layer Boundaries (AES)

#### Agent Layer (`agent_*.rs`)

| Allowed                                             | Forbidden                                             |
| --------------------------------------------------- | ----------------------------------------------------- |
| Orchestration flow (`for`, `while`, `loop`)         | Domain computation                                    |
| Control flow (`if/else`, `match`)                   | Arithmetic or analytics calculations                  |
| Sequential pipeline statements                      | Data transformation logic                             |
| Calling injected protocol/port traits               | Business rules                                        |
| Error propagation (`?`, `match`, `if let`)          | Domain validation                                     |
| Collecting results/violations into shared VO types  | File I/O (`std::fs`, `File::open`)                    |
| Async coordination (`select!`, `join!`)             | Network calls (`reqwest`, `hyper`)                    |
| Aggregate trait implementation                      | Database operations (`sqlx`, `rusqlite`)              |
| Private helpers supporting orchestration            | Direct stdout/stderr printing                         |
|                                                     | Direct environment/system-clock/global-state mutation |
|                                                     | Direct import from concrete `capabilities_*` modules  |
|                                                     | Direct import from concrete `infrastructure_*` modules |
|                                                     | Direct import from concrete `surface_*` modules       |
|                                                     | Locally defined domain data structures                |
|                                                     | Raw primitives for domain values in aggregate contracts |

Agents may depend on:

- `shared/*`
- taxonomy VOs
- taxonomy constants
- taxonomy utilities
- protocol traits
- port traits
- aggregate traits

Agents must not depend on concrete implementations from lower layers.

---

### What Agents May Do

Agents are allowed to perform technical orchestration.

Examples:

```rust
for file in request.files() {
    match self.analyzer.analyze(file) {
        Ok(result) => violations.extend(result.into_violations()),
        Err(err) => violations.push(LintResult::from_analysis_error(file, err)),
    }
}
```

Allowed:

- iterate over input collections,
- call injected dependencies,
- route success/failure,
- continue or stop pipeline,
- collect results into shared VO collections,
- propagate errors,
- coordinate async tasks.

---

### What Agents Must Not Do

Agents must not perform domain computation or business decisions.

Bad:

```rust
let total = files.len();
let score = passed * 100 / total;
let is_orphan = content.contains("orphan");
```

These belong in capabilities.

---

### Structural Rules

#### 1. One implementation struct per file

Each agent file contains exactly ONE main implementation struct.

```rust
pub struct OrphanOrchestrator {
    analyzer: Arc<dyn IOrphanAnalyzerProtocol>,
}
```

Do not define multiple service structs in the same file.

---

#### 2. Only the implementation struct may be defined in the layer file

An agent file may define the implementation struct only.

Domain data structures MUST live in `shared/<domain>/taxonomy_*`.

Forbidden in agent files:

```rust
pub struct OrphanReport {
    results: Vec<String>,
}
```

Allowed:

```rust
use shared::orphan_detector::taxonomy_orphan_report_vo::OrphanReport;
```

---

#### 3. Service dependencies must use DI

Fields that represent collaborators, capabilities, ports, or services MUST use trait objects.

```rust
pub struct ImportOrchestrator {
    checker: Arc<dyn IImportCheckerProtocol>,
    parser: Arc<dyn IParserPort>,
}
```

Do not use concrete service types:

```rust
pub struct ImportOrchestrator {
    checker: MandatoryChecker, // BAD: concrete dependency
}
```

---

#### 4. Value/configuration fields must use shared VOs

Fields that represent domain values, configuration, identifiers, policies, etc. should use shared VOs.

```rust
pub struct LintExecutionOrchestrator {
    executor: Arc<dyn ILintExecutorProtocol>,
    policy: ExecutionPolicy,
}
```

Avoid raw primitives for domain values:

```rust
pub struct LintExecutionOrchestrator {
    executor: Arc<dyn ILintExecutorProtocol>,
    max_retries: u32, // BAD
}
```

---

### Helper vs Utility Decision

The boundary is not only about `&self`.

The real question is:

> Does this function know about agent-specific/domain-specific rules, or is it just a blind reusable tool?
>
> AND
>
> Is it used only by this struct, or by multiple modules?

---

### When to Keep as Private Helper (Block 3)

Keep the function inside the agent file if ANY of these is true:

1. It accesses `self.field` or instance state.
2. It is tightly coupled to this orchestrator only.
3. It is a factory method such as `new()` or builder method.
4. It contains agent-specific pipeline knowledge.
5. It contains domain knowledge.
6. It is stateless but only used by this one struct.

Example:

```rust
impl OrphanOrchestrator {
    fn should_skip_file(&self, file: &FilePath) -> bool {
        self.policy.is_excluded(file)
    }
}
```

This helper is specific to this orchestrator and may remain in Block 3.

---

### When to Extract to Utility (`*_utility.rs`)

Extract the function to shared taxonomy utility ONLY if ALL of these are true:

1. Stateless: no `&self`, no struct field access.
2. Pure: input A always produces output B.
3. No side effects: no I/O, no network, no database, no global mutation.
4. Domain-agnostic: does not know business or orchestration rules.
5. Reusable: useful for multiple modules/layers.

Example:

```rust
// shared/common/taxonomy_collection_utility.rs
pub fn chunk_count(total_items: usize, chunk_size: usize) -> usize {
    if chunk_size == 0 {
        return 0;
    }

    total_items.div_ceil(chunk_size)
}
```

If a function is stateless but domain-specific, keep it as a helper in the consuming layer.

---

### I/O Blocker

A function may be stateless, but if it performs I/O, it MUST NOT become a taxonomy utility.

It also MUST NOT stay in agent.

```text
Stateless + I/O = infrastructure/port implementation
NOT taxonomy utility
NOT agent layer
```

---

## The 3-Block Structure

Every implementation file MUST follow this order:

1. **Block 1 — Struct Definition**
2. **Block 2 — Aggregate Trait Implementation**
3. **Block 3 — Constructors, Std Traits, and Private Helpers**

---

### Block 1 — Struct Definition

```rust
pub struct OrphanOrchestrator {
    analyzer: Arc<dyn IOrphanAnalyzerProtocol>,
}
```

---

### Block 2 — Public Contract

Block 2 is RESERVED for the domain aggregate trait ONLY.

```rust
impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        // orchestration only
    }
}
```

Do NOT put these in Block 2:

```rust
impl Default for OrphanOrchestrator
impl Clone for OrphanOrchestrator
impl Debug for OrphanOrchestrator
impl Display for OrphanOrchestrator
impl From<...> for OrphanOrchestrator
```

Those belong in Block 3.

---

### Block 3 — Constructors, Std Traits, and Helpers

Block 3 contains:

- `new()`
- builders
- `Default`
- `Clone`
- `Debug`
- `Display`
- other std trait impls
- private helper methods
- agent-specific associated functions used only by this struct

```rust
impl OrphanOrchestrator {
    pub fn new(analyzer: Arc<dyn IOrphanAnalyzerProtocol>) -> Self {
        Self { analyzer }
    }

    fn should_skip_file(&self, file: &FilePath) -> bool {
        self.policy.is_excluded(file)
    }
}
```

---

### Utility Functions Do Not Belong in Block 3

If a function is:

- stateless,
- pure,
- domain-agnostic,
- and reusable across multiple modules,

then extract it to shared utility.

But if the function is agent-specific or orchestration-specific, it may remain in Block 3.

---

## Trait Placement Decision Rule

```text
Trait impl found in an agent file?
  │
  ├─ Is it the domain aggregate? (I<Name>Aggregate)
  │   └─ YES → Block 2
  │
  └─ Is it a std/derive/utility trait? (Default, Clone, Debug, Display, From, etc.)
      └─ YES → Block 3
```

---

## Example: Correct 3-Block Order

```rust
use std::sync::Arc;

use shared::code_analysis::taxonomy_file_path_vo::FilePath;
use shared::code_analysis::taxonomy_lint_result_vo::LintResult;
use shared::orphan_detector::taxonomy_orphan_analyzer_protocol::IOrphanAnalyzerProtocol;
use shared::orphan_detector::taxonomy_orphan_orchestrator_aggregate::IOrphanOrchestratorAggregate;
use shared::scan::taxonomy_scan_request_vo::ScanRequest;

// ─── Block 1: Struct Definition ───────────────────────────
pub struct OrphanOrchestrator {
    analyzer: Arc<dyn IOrphanAnalyzerProtocol>,
}

// ─── Block 2: Public Contract (domain aggregate ONLY) ─────
impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        let mut violations = Vec::new();

        for file in request.files() {
            match self.analyzer.analyze(file) {
                Ok(result) => violations.extend(result.into_violations()),
                Err(err) => violations.push(LintResult::from_analysis_error(file, err)),
            }
        }

        violations
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl OrphanOrchestrator {
    pub fn new(analyzer: Arc<dyn IOrphanAnalyzerProtocol>) -> Self {
        Self { analyzer }
    }
}
```

---

## Aggregate Rules

### Every agent struct MUST implement an aggregate trait

```rust
impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    // public contract
}
```

---

### Aggregate file naming

| Layer          | File Pattern            | Trait File                       | Trait Name           |
| -------------- | ----------------------- | -------------------------------- | -------------------- |
| Capabilities   | `capabilities_*.rs`   | `contract_<name>_protocol.rs`  | `I<Name>Protocol`  |
| Infrastructure | `infrastructure_*.rs` | `contract_<name>_port.rs`      | `I<Name>Port`      |
| Agents         | `agent_*.rs`          | `contract_<name>_aggregate.rs` | `I<Name>Aggregate` |

---

### Aggregate content rules

The aggregate trait MUST contain only public orchestration contract methods.

Good:

```rust
pub trait IOrphanOrchestratorAggregate: Send + Sync {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult>;
}
```

Bad:

```rust
pub trait IOrphanOrchestratorAggregate: Send + Sync {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult>;

    fn private_helper(&self); // BAD
}
```

---

### Constructors are not aggregate methods

`new()` and builders MUST stay in Block 3.

Bad:

```rust
pub trait IOrphanOrchestratorAggregate {
    fn new() -> Self; // BAD
}
```

Good:

```rust
impl OrphanOrchestrator {
    pub fn new(analyzer: Arc<dyn IOrphanAnalyzerProtocol>) -> Self {
        Self { analyzer }
    }
}
```

---

### Aggregate methods should use shared VOs

Aggregate contracts should avoid raw primitives for domain values.

Bad:

```rust
pub trait IOrphanOrchestratorAggregate {
    fn execute(&self, files: Vec<String>) -> Vec<String>;
}
```

Good:

```rust
pub trait IOrphanOrchestratorAggregate {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult>;
}
```

---

### Object safety

Aggregate traits intended for `Arc<dyn Trait>` MUST be object-safe.

Avoid generic methods in dyn-compatible traits unless bounded properly.

If a generic method is required, add:

```rust
where
    Self: Sized,
```

or split the trait into:

- object-safe aggregate trait
- generic extension trait

---

## The Fundamental Question

> **“Is this file orchestration / pipeline execution only?”**

If yes → **`agent_*.rs` + implement aggregate trait**

If no:

- has I/O → move to infrastructure
- has business/domain logic → move to capabilities
- has domain data → move to taxonomy

---

## Agent Layer Purpose

Agents are the orchestration layer.

They coordinate between capabilities and infrastructure through injected traits.

Agents contain:

- pipeline flow,
- dependency calls,
- error routing,
- result collection,
- async coordination.

Agents do NOT contain:

- arithmetic/domain calculations,
- parsing/domain normalization,
- business rules,
- domain validation,
- I/O,
- local domain data definitions.

---

## Detection Patterns

### BAD: Computation in Agent

```rust
impl OrphanOrchestrator {
    fn process(&self, files: &[FilePath]) {
        let total = files.len(); // BAD: domain/technical computation in agent

        let sum: usize = files
            .iter()
            .map(|f| f.size())
            .sum(); // BAD: computation/transformation
    }
}
```

Fix:

Move computation to capabilities.

```rust
impl IOrphanAnalyzerProtocol for OrphanAnalyzer {
    fn analyze_summary(&self, files: &[FilePath]) -> FileSummary {
        // computation belongs here
    }
}
```

Agent only calls it:

```rust
let summary = self.analyzer.analyze_summary(files);
```

---

### BAD: Business Logic in Agent

```rust
impl OrphanOrchestrator {
    fn analyze(&self, content: &FileContent) -> bool {
        content.value().contains("orphan") // BAD: business rule
    }
}
```

Fix:

Move to capabilities.

```rust
impl IOrphanAnalyzerProtocol for OrphanAnalyzer {
    fn analyze(&self, content: &FileContent) -> OrphanAnalysisResult {
        // domain logic here
    }
}
```

---

### BAD: I/O in Agent

```rust
impl OrphanOrchestrator {
    fn execute(&self, path: &FilePath) {
        let content = std::fs::read_to_string(path.value()); // BAD
    }
}
```

Fix:

Use an injected port.

```rust
impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        let mut violations = Vec::new();

        for file in request.files() {
            match self.reader.read(file) {
                Ok(content) => {
                    // delegate analysis to capabilities
                }
                Err(err) => {
                    violations.push(LintResult::from_read_error(file, err));
                }
            }
        }

        violations
    }
}
```

The agent does not perform I/O directly.

---

### BAD: Dataclass Defined in Agent File

```rust
pub struct OrphanReport {
    results: Vec<String>,
}
```

Fix:

Move to taxonomy.

```rust
// shared/orphan_detector/taxonomy_orphan_report_vo.rs
pub struct OrphanReport {
    results: OrphanResultList,
}
```

Then import:

```rust
use shared::orphan_detector::taxonomy_orphan_report_vo::OrphanReport;
```

---

### BAD: Concrete Service Field

```rust
pub struct OrphanOrchestrator {
    analyzer: OrphanAnalyzer, // BAD
}
```

Fix:

```rust
pub struct OrphanOrchestrator {
    analyzer: Arc<dyn IOrphanAnalyzerProtocol>,
}
```

---

### BAD: Std Trait in Block 2

```rust
pub struct OrphanOrchestrator;

impl Default for OrphanOrchestrator {
    fn default() -> Self {
        Self
    }
}

impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        Vec::new()
    }
}
```

Fix:

```rust
pub struct OrphanOrchestrator;

impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        Vec::new()
    }
}

impl Default for OrphanOrchestrator {
    fn default() -> Self {
        Self
    }
}
```

---

### GOOD: Implementor with Shared Data and DI

```rust
use std::sync::Arc;

use shared::import_rules::taxonomy_import_checker_protocol::IImportCheckerProtocol;
use shared::parser::taxonomy_parser_port::IParserPort;
use shared::import_rules::taxonomy_import_orchestrator_aggregate::IImportOrchestratorAggregate;

pub struct ImportOrchestrator {
    checker: Arc<dyn IImportCheckerProtocol>,
    parser: Arc<dyn IParserPort>,
}

impl IImportOrchestratorAggregate for ImportOrchestrator {
    // public aggregate methods only
}
```

---

### GOOD: Correct 3-Block with Std Traits

```rust
pub struct OrphanOrchestrator;

impl IOrphanOrchestratorAggregate for OrphanOrchestrator {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
        Vec::new()
    }
}

impl Default for OrphanOrchestrator {
    fn default() -> Self {
        Self
    }
}

impl OrphanOrchestrator {
    pub fn new() -> Self {
        Self
    }
}
```

---

## Workflow

### Step 1: Analyze File

Read the file and ask:

> Is this orchestration only?

If yes → keep as agent.

If it contains:

- computation → move to capabilities
- business logic → move to capabilities
- I/O → move to infrastructure
- domain data → move to taxonomy

---

### Step 2: Check for Missing Aggregate

Does the agent struct implement an aggregate trait?

If no:

1. create `contract_<name>_aggregate.rs`
2. define `I<Name>Aggregate`
3. move public orchestration method signatures into the trait
4. implement the trait for the struct

---

### Step 3: Create Aggregate File if Missing

Create aggregate file in the appropriate shared domain folder.

Examples:

| Crate           | Aggregate Path                                                |
| --------------- | ------------------------------------------------------------- |
| import-rules    | `crates/shared/src/import_rules/contract_*_aggregate.rs`    |
| code-analysis   | `crates/shared/src/code_analysis/contract_*_aggregate.rs`   |
| orphan-detector | `crates/shared/src/orphan_detector/contract_*_aggregate.rs` |

Register the module in the relevant `mod.rs`.

---

### Step 4: Enforce 3-Block Structure

Reorganize the file into:

1. struct definition
2. aggregate trait implementation
3. constructors, std traits, private helpers

---

### Step 5: Verify Struct Discipline

Check:

- exactly one implementation struct
- no local domain data structs
- no local enums/VOs/DTOs/constants
- service fields use `Arc<dyn Trait>`
- value fields use shared VOs

---

### Step 6: Verify Helper vs Utility Boundary

For each helper/function:

```text
Does it know agent-specific or domain-specific details?
├─ YES → keep as helper in Block 3
└─ NO
   Is it stateless, pure, and reusable by multiple modules?
   ├─ YES → extract to *_utility.rs
   └─ NO → keep as helper in Block 3
```

---

### Step 7: Verify Layer Compliance

Ensure:

- no forbidden imports from concrete capabilities
- no forbidden imports from concrete infrastructure
- no forbidden imports from surface
- no I/O
- no business logic
- no domain computation

---

### Step 8: Verify Error Handling, VO, and Constants

Check:

- no silent `unwrap_or_default()`
- aggregate contracts use shared VOs
- no raw primitives for domain values
- no magic constants

---

### Step 9: Verify Compilation

Run:

```bash
cargo check -p <crate-name>
```

---

## Verification Checklist

- [ ] File follows the 3-Block Structure.
- [ ] Block 1 contains exactly one implementation struct.
- [ ] Block 2 contains ONLY the aggregate trait implementation.
- [ ] Block 3 contains constructors, std traits, and private helpers.
- [ ] Agent struct implements an aggregate trait.
- [ ] Aggregate contains only public contract methods.
- [ ] Private helpers are not declared in the aggregate.
- [ ] Constructors are not declared in the aggregate.
- [ ] Std trait impls are in Block 3.
- [ ] Agent-specific helpers may remain in Block 3.
- [ ] Reusable, stateless, domain-agnostic functions are extracted to `*_utility.rs`.
- [ ] No reusable utility-like functions remain inside Block 3.
- [ ] Generic aggregate methods are object-safe or bounded with `where Self: Sized`.
- [ ] One file contains exactly one implementation struct.
- [ ] No domain data structures are defined locally.
- [ ] All domain data structures are imported from shared/taxonomy.
- [ ] Service dependencies use `Arc<dyn Trait>`.
- [ ] Value/configuration fields use shared VOs.
- [ ] Aggregate signatures use shared VOs for domain data.
- [ ] Agent contains zero I/O.
- [ ] Agent contains zero business logic.
- [ ] Agent contains zero domain computation.
- [ ] No forbidden imports from concrete `capabilities_*`.
- [ ] No forbidden imports from concrete `infrastructure_*`.
- [ ] No forbidden imports from concrete `surface_*`.
- [ ] Aggregate module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes.

---

## Quick Commands

These commands are rough heuristic checks. Final validation should use `cargo check`, clippy, or AST-based tooling.

```bash
# List structs in agent files
rg -n "^\s*pub struct" crates/<crate>/src/agent_*.rs

# List aggregate trait implementations
rg -n "impl\s+I[A-Za-z0-9_]+Aggregate\s+for" crates/<crate>/src/agent_*.rs

# Check possible computation/transformation patterns
rg "\.sum\(\)|\.len\(\)|\.map\(|\.fold\(|\.collect\(" crates/<crate>/src/agent_*.rs

# Check possible I/O in agents
rg "std::fs|File::open|reqwest|hyper|sqlx|rusqlite" crates/<crate>/src/agent_*.rs

# Check forbidden imports
rg "^\s*use\s+.*(capabilities_|infrastructure_|surface_)" crates/<crate>/src/agent_*.rs

# Find unwrap_or_default usage
rg "unwrap_or_default\(\)" crates/<crate>/src/agent_*.rs

# Find possible magic numbers
rg "[0-9]+\.[0-9]+" crates/<crate>/src/agent_*.rs

# Check object safety issues
cargo check -p <crate-name> 2>&1 | rg "cannot be made into an object"
```

Note:

Patterns like `.iter()` or `.map()` can appear in harmless technical code. Always inspect context. The real violation is **domain computation/transformation**, not merely using iterator control flow.

---

### Check Wrong Block Order

```bash
for file in crates/<crate>/src/agent_*.rs; do
  awk '
    FNR == 1 { std = 0; proto = 0 }

    /^impl (Default|Clone|Debug|Display)/ {
      if (!std) std = FNR
    }

    /^impl I[A-Z].*Aggregate/ {
      if (!proto) proto = FNR
    }

    END {
      if (std && proto && std < proto) {
        print "VIOLATION: " FILENAME " std trait (line " std ") before aggregate (line " proto ")"
      }
    }
  ' "$file"
done
```

---

## Computation Detection Rules

Agent layer must not contain domain computation.

Forbidden:

- arithmetic,
- totals,
- averages,
- counts used as domain decisions,
- sum,
- fold,
- parsing,
- normalization,
- deriving domain meaning from data.

Allowed:

- iterating to call dependencies,
- routing results,
- pushing results into a collection,
- propagating errors,
- continuing or stopping pipeline.

Bad:

```rust
let total = files.len();
let average = total_score / total;
```

Good:

```rust
let summary = self.analyzer.summarize(files);
```

---

## Error Handling Rules

Agent error handling must be explicit.

### Rule 1: Do not silently discard errors

Forbidden:

```rust
let result = checker.check().unwrap_or_default();
```

---

### Rule 2: Agent may return `Vec<LintResult>` for analysis orchestration

For linting/analysis pipelines, violations are expected domain outcomes.

```rust
fn execute(&self, request: &ScanRequest) -> Vec<LintResult> {
    let mut violations = Vec::new();

    for file in request.files() {
        match self.analyzer.analyze(file) {
            Ok(result) => violations.extend(result.into_violations()),
            Err(err) => violations.push(LintResult::from_analysis_error(file, err)),
        }
    }

    violations
}
```

---

### Rule 3: Agent may return `Result` for execution orchestration

If the aggregate represents a fallible execution operation, return `Result`.

```rust
fn run(&self, request: &ScanRequest) -> Result<ExecutionReport, AgentExecutionError>;
```

---

### Rule 4: Agent must not perform I/O error handling directly

Bad:

```rust
let content = match std::fs::read_to_string(path.value()) {
    Ok(c) => c,
    Err(_) => String::new(),
};
```

Good:

```rust
match self.reader.read(file) {
    Ok(content) => {
        // delegate to capability
    }
    Err(err) => {
        violations.push(LintResult::from_read_error(file, err));
    }
}
```

The agent calls a port. The port implementation lives in infrastructure.

---

## Primitive and VO Rules

Aggregate contracts should use shared VOs for domain data.

Bad:

```rust
pub trait IOrphanOrchestratorAggregate {
    fn execute(&self, files: Vec<String>) -> Vec<String>;
}
```

Good:

```rust
pub trait IOrphanOrchestratorAggregate {
    fn execute(&self, request: &ScanRequest) -> Vec<LintResult>;
}
```

### Primitive Policy

| Primitive            | Rule                                                                                |
| -------------------- | ----------------------------------------------------------------------------------- |
| `String`           | Forbidden for domain fields and public contract values. Use VO.                     |
| `i32`, `i64`     | Forbidden for domain values. Use VO.                                                |
| `u32`, `u64`     | Forbidden for domain values. Use VO.                                                |
| `usize`, `isize` | Forbidden for domain values. Use VO.                                                |
| `f32`, `f64`     | Forbidden for domain values. Use VO.                                                |
| `char`             | Forbidden for domain values. Use VO.                                                |
| `bool`             | Allowed for semantic toggles when no richer VO is needed.                           |
| `&str`             | May be allowed for borrowed low-level input, but domain identifiers should use VOs. |

Prefer VOs for:

- requests,
- reports,
- file paths,
- identifiers,
- execution results,
- violations,
- policies,
- thresholds.

---

## Magic Constant Extraction Rules

No hardcoded domain literals in agent layer.

Bad:

```rust
let fps = Fps::new(24.0).map_err(AgentExecutionError::invalid_fps)?;
```

Good:

```rust
use shared::animator::taxonomy_animator_constant::FPS_DEFAULT;

let fps = Fps::new(FPS_DEFAULT).map_err(AgentExecutionError::invalid_fps)?;
```

Constants MUST live in:

```text
taxonomy_*_constant.rs
```

---

## Import Strategy

### Agent Import Rules

Allowed:

```text
shared taxonomy types
shared contract traits
protocol traits
port traits
aggregate traits
```

Forbidden:

```text
concrete capabilities_* structs
concrete infrastructure_* structs
concrete surface_* structs
```

Bad:

```rust
use crate::capabilities_mandatory_checker::MandatoryChecker;
use crate::infrastructure_parser_adapter::ParserAdapter;
```

Good:

```rust
use shared::import_rules::taxonomy_mandatory_checker_protocol::IMandatoryCheckerProtocol;
use shared::parser::taxonomy_parser_port::IParserPort;

pub struct ImportOrchestrator {
    checker: Arc<dyn IMandatoryCheckerProtocol>,
    parser: Arc<dyn IParserPort>,
}
```

---

## Circular Dependencies

If a circular dependency appears between agent and lower layers, do not import concrete modules back and forth.

Bad:

```rust
// agent_runner.rs
use crate::capabilities_analyzer::Analyzer;

// capabilities_analyzer.rs
use crate::agent_runner::Runner;
```

Fix by moving abstractions to shared contracts and wiring them in the composition root.

Prefer:

```rust
// shared/scan/taxonomy_scan_progress_port.rs
pub trait IScanProgressPort: Send + Sync {
    fn progress(&self, event: ScanProgressEvent);
}
```

Lower layers may depend on shared abstractions, not on concrete agent structs.

If a lower layer needs to notify orchestration, use:

- events,
- callbacks,
- ports,
- shared contracts,
- composition root wiring.

Do not create direct lower-layer → agent concrete dependencies.

---

## Common Mistakes

- ❌ Putting domain computation in agents.
- ❌ Putting arithmetic or analytics calculations in agents.
- ❌ Putting business logic in agents.
- ❌ Putting I/O in agents.
- ❌ Defining domain data structs in agent files.
- ❌ Using concrete service types as struct fields.
- ❌ Using raw primitives for domain value fields.
- ❌ Exposing raw primitives in aggregate contracts.
- ❌ Putting private helpers in the aggregate trait.
- ❌ Putting constructors in the aggregate trait.
- ❌ Placing std trait impls before the aggregate trait.
- ❌ Mixing Block 2 and Block 3 responsibilities.
- ❌ Keeping reusable, domain-agnostic utility functions inside Block 3.
- ❌ Extracting agent-specific helpers to shared utility too early.
- ❌ Creating god aggregates with too many unrelated methods.
- ❌ Forgetting object safety for `Arc<dyn Trait>` usage.
- ❌ Multiple implementation structs in one file.
- ❌ Direct dependency on concrete capabilities implementations.
- ❌ Direct dependency on concrete infrastructure implementations.
- ❌ Silent error swallowing with `unwrap_or_default()`.
- ❌ Magic constants in agent logic.
- ❌ Treating simple iteration as computation violation without inspecting context.

```

