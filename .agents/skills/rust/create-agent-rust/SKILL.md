---
name: create-agent-rust
description: "Create and validate agent layer files following AES rules: 3-block structure, one struct per file, aggregate contracts, zero computation/I/O/business logic."
version: 1.0.0
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
  ]
triggers:
  - "create agent rust"
  - "add agent rust"
  - "fix agent structure rust"
  - "create aggregate rust"
  - "agent missing aggregate rust"
  - "validate agent logic rust"
dependencies: []
related:
  - create-capabilities-rust
  - create-infrastructure-rust
  - enforce-1-struct-per-file-rust
  - trait-consolidation-rust
  - module_logic_validator-rust
  - fix-agent-di
---

# create-agent-rust

## Purpose

Create and validate Rust **agent layer** files following clean architecture rules. Ensures agents contain zero computation, zero I/O, and zero business logic — they are orchestration/pipeline execution only. Agents implement aggregate traits, follow the 3-Block Structure, and use DI for all fields.

**This skill consolidates rules from:** `module_logic_validator`, `enforce-1-struct-per-file`, `trait-consolidation`, and `fix-agent-di` — applied specifically to the agent layer.

## Rules

### Layer Boundaries (AES)

**Agent Layer (`agent_*.rs`)**

| Allowed                                          | Forbidden                                            |
| ------------------------------------------------ | ---------------------------------------------------- |
| `for`, `while`, `loop` (orchestration flow)      | Computation (arithmetic, `sum()`, `len()`)           |
| `if/else`, `match` (control flow for pipelines)  | Business rules, domain logic                         |
| `?`, `if let` (error propagation)                | File I/O (`std::fs`, `File::open`)                   |
| `tokio::select!`, `tokio::time::sleep` (async)   | Network (`reqwest`, `hyper`)                         |
| Sequential statements (orchestration)            | Database (`sqlx`, `rusqlite`)                        |
| Trait implementation                             | Domain model definition (`struct`)                   |
|                                                  | Direct import from `capabilities_*`                  |
|                                                  | Direct import from `infrastructure_*`                |

### Structural Rules (All Layers)

- **1 file = 1 impl struct** — each agent file contains exactly ONE main impl struct
- **All data classes in shared** — no structs/enums with data may be defined outside shared/taxonomy
- **Fields must use DI** — impl struct fields should be `Arc<dyn Trait>` objects, not concrete types
- **Helper functions stay in layer** — helper methods that support the impl struct remain in the file
- **Utility functions → extract to taxonomy** — truly stateless, domain-agnostic free functions (no `&self`) should be extracted to `*_utility.rs` modules in shared/taxonomy

### The 3-Block Structure

Every implementation file MUST follow this exact order:

1. `struct Definition`
2. `impl Aggregate for Struct` (Public Contract)
3. `impl Struct` (Constructors & Helpers)

**CRITICAL:** Utility functions extracted to standalone modules — Stateless, domain-agnostic free functions (no `&self`) MUST be extracted OUT of the impl block into their own `*_utility.rs` modules in shared/taxonomy. They do NOT belong in Block 3.

### Aggregate Rules

- **Every agent struct MUST implement an aggregate trait**
- **Aggregate MUST define methods for all public methods**
- **Aggregate contains ONLY public/contract methods** — no private helpers
- **Private helpers stay in Block 3** (`impl Struct`)
- **Constructors (`new`, builders) in Block 3**
- **Generic aggregate methods need `where Self: Sized`**

## The Fundamental Question

> **"Is this file orchestration/pipeline execution only?"**

If yes → **`agent_*.rs` + implement aggregate trait**
If no (has computation, I/O, or business logic) → **split into appropriate layer**

## Naming Convention

| Layer          | File Pattern             | Trait File                          | Trait Name         |
| -------------- | ------------------------ | ----------------------------------- | ------------------ |
| **Capabilities** | `capabilities_*.rs`      | `contract_<name>_protocol.rs`       | `I<Name>Protocol`  |
| **Infrastructure** | `infrastructure_*.rs`  | `contract_<name>_port.rs`           | `I<Name>Port`      |
| **Agents**       | `agent_*.rs`             | `contract_<name>_aggregate.rs`      | `I<Name>Aggregate` |

## Agent Layer Purpose

Agents are the **orchestration layer** — they coordinate between capabilities and infrastructure but contain:

- **NO computation** (no arithmetic, no data transformation)
- **NO business logic** (no domain rules, no validation)
- **NO I/O** (no file reads, no network calls, no database queries)

Their sole purpose is to orchestrate pipeline execution by calling into capabilities and infrastructure.

## Detection Patterns

### BAD: Computation in Agent

```rust
// BAD: Computation in agent layer
impl OrphanOrchestrator {
    fn process(&self) {
        let total = files.len();  // ← COMPUTATION — should be in capabilities
        let sum: usize = files.iter().map(|f| f.size()).sum();  // ← FORBIDDEN
    }
}
```

### BAD: Business Logic in Agent

```rust
// BAD: Domain logic in agent layer
impl OrphanOrchestrator {
    fn analyze(&self, content: &str) -> bool {
        return content.contains("orphan");  // ← BUSINESS RULE — should be in capabilities
    }
}
```

### BAD: Dataclass in Layer File

```rust
// BAD: Domain data defined in agent layer
pub struct OrphanReport {  // ← DATA CLASS — should be in shared/taxonomy
    results: Vec<String>,
    timestamp: u64,
}

pub struct OrphanOrchestrator {
    report: OrphanReport,  // ← concrete type, not DI
}
```

### GOOD: Implementor with Shared Data

```rust
// GOOD: All data from shared, fields use DI
use shared::orphan_detector::contract_orphan_protocol::ICapabilitiesOrphanProtocol;

pub struct OrphanOrchestrator {
    analyzer: Arc<dyn ICapabilitiesOrphanProtocol>,  // ← DI
}

impl IOrphanOrchestratorAggregate for OrphanOrchestrator { ... }
```

## Workflow

### Step 1: Analyze File

Read file and check for prohibited content. Ask: **"Is this orchestration only?"**

- If it has computation → **MOVE to Capabilities**
- If it has I/O or business logic → **split into appropriate layer**
- If pure orchestration → continue to Step 2

### Step 2: Check for Missing Aggregate

Does the agent struct implement an aggregate trait? If no → create one.

```bash
# Find agents without aggregate implementations
grep -rn "^pub struct" crates/<crate>/src/ | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    struct=$(echo "$line" | grep -oP 'pub struct \K[a-zA-Z_]+')
    grep -q "impl.*Aggregate for $struct" "$file" || echo "MISSING: $file has $struct without aggregate"
done
```

### Step 3: Create Aggregate File (if missing)

Create `contract_<name>_aggregate.rs` in the shared crate with all public method signatures.

**Aggregate location:**

| Crate         | Aggregate Path                                           |
| ------------- | -------------------------------------------------------- |
| import-rules  | `crates/shared/src/import_rules/contract_*_aggregate.rs` |
| code-analysis | `crates/shared/src/code_analysis/contract_*_aggregate.rs`|
| orphan-detector | `crates/shared/src/orphan_detector/contract_*_aggregate.rs` |

### Step 4: Enforce 3-Block Structure

Reorganize into strict 3-block order:

1. `pub struct <Type>` (struct definition with DI fields)
2. `impl I<Name>Aggregate for <Type>` (all public contract methods)
3. `impl <Type>` (constructors, private helpers — utilities extracted to standalone modules)

### Step 5: Verify Struct Discipline

- **1 file = 1 impl struct** — no multiple structs in one file
- **All data classes in shared/taxonomy** — domain structs must be imported, not defined locally
- **Fields use DI** — `Arc<dyn Trait>`, never concrete types
- **No free functions (no `&self`) remain in Block 3** — extract to `*_utility.rs` modules

### Step 6: Verify Layer Compliance

Check forbidden imports and prohibited patterns:

```bash
# Check for computation in agents
grep -n "sum()\|len()\|\.iter()\|\.map(" crates/<crate>/src/agent_*.rs

# Check for forbidden imports
grep -n "capabilities_\|infrastructure_" crates/<crate>/src/agent_*.rs
```

### Step 7: Verify

Run `cargo check` to confirm no violations.

## Verification Checklist

- [ ] File follows the **3-Block Structure** (Struct -> Impl Aggregate -> Impl Struct).
- [ ] Agent struct implements an aggregate trait.
- [ ] Aggregate contains **only** public/contract methods (no private helpers).
- [ ] Private helpers are in Block 3 (`impl Struct`).
- [ ] Constructors (`new`, builders) are in Block 3.
- [ ] No free functions (no `&self`) remain in Block 3 — extracted to `*_utility.rs` modules.
- [ ] Stateless utilities exist in their own `*_utility.rs` files in shared/taxonomy.
- [ ] Generic aggregate methods include `where Self: Sized`.
- [ ] **1 file = 1 impl struct** — no multiple structs in one file.
- [ ] All data classes imported from shared/taxonomy (none defined locally).
- [ ] Impl struct fields use DI (`Arc<dyn Trait>`), not concrete types.
- [ ] **Zero computation** in agent layer (no arithmetic, no sum(), no len()).
- [ ] **Zero I/O** in agent layer (no std::fs, no network, no database).
- [ ] **Zero business logic** in agent layer (no domain rules, no validation).
- [ ] No forbidden imports (no capabilities_*, no infrastructure_*).
- [ ] Aggregate module is registered in the shared crate's `mod.rs`.
- [ ] `cargo check -p <crate-name>` passes without warnings or errors.

## Quick Commands

```bash
# Verify 3-Block Structure order (rough check)
grep -n "^impl\|^pub struct" crates/<crate>/src/agent_*.rs

# Find agents without aggregate implementations
grep -rn "^pub struct" crates/<crate>/src/ | while read line; do
    file=$(echo "$line" | cut -d: -f1)
    struct=$(echo "$line" | grep -oP 'pub struct \K[a-zA-Z_]+')
    grep -q "impl.*Aggregate for $struct" "$file" || echo "MISSING: $file has $struct without aggregate"
done

# Ensure aggregate does NOT contain private helper keywords
grep -E "fn (helper|util|private|internal)" crates/shared/src/contract_*_aggregate.rs || echo "Clean: No helpers in aggregate"

# Check for computation in agents
grep -n "sum()\|len()\|\.iter()\|\.map(" crates/<crate>/src/agent_*.rs

# Check for I/O in agents
grep -n "std::fs\|File::open\|reqwest\|sqlx" crates/<crate>/src/agent_*.rs

# Check for business logic in agents
grep -n "is_orphan\|analyze\|validate" crates/<crate>/src/agent_*.rs

# Check for dataclasses defined in layer files
grep -rn "^pub struct" crates/<crate>/src/ | grep -v "shared/" | grep -v "impl\|trait\|fn " | grep agent

# Check for concrete type fields (non-DI)
grep -rn "^\s*[a-z_]*:" crates/<crate>/src/agent_*.rs | grep -v "Arc<dyn"

# Find free functions in Block 3 that should be extracted
grep -n "^    pub fn [a-z_]*(\s*[^&])" crates/<crate>/src/agent_*.rs

# Check for object safety violations
cargo check -p <crate-name> 2>&1 | grep "cannot be made into an object"

# Find unwrap_or_default() calls (error handling)
rg "unwrap_or_default\(\)" crates/<crate>/src/agent_*.rs

# Find magic constants (hardcoded literals)
rg "[0-9]+\.[0-9]+|#[0-9A-Fa-f]+" crates/<crate>/src/agent_*.rs | grep -v "// " | head -20

# Find computation patterns in agents
rg "\.sum\(\)|\.len\(\)|\.iter\(\)|\.map\(" crates/<crate>/src/agent_*.rs
```

## Computation Detection (from module_logic_validator)

**Agent Layer Computation Rules:**

- **NO computation** — arithmetic, `sum()`, `len()`, `.iter()`, `.map()` are FORBIDDEN
- All orchestration flow uses `for`, `while`, `loop`, `if/else`, `match`
- Control flow for pipelines is ALLOWED — but not actual data transformation

```rust
// [FORBIDDEN] BEFORE
impl OrphanOrchestrator {
    fn process(&self) {
        let total = files.len();  // ← COMPUTATION — should be in capabilities
        let sum: usize = files.iter().map(|f| f.size()).sum();  // ← FORBIDDEN
    }
}

// [OK] AFTER — orchestration only
impl OrphanOrchestrator {
    fn process(&self) {
        for file in &self.files {  // ← ORCHESTRATION — OK
            self.analyzer.analyze(file);  // ← DELEGATE to capabilities
        }
    }
}
```

## Error Handling with LintResult (from fix-error-handling)

**Agent Layer Error Rules:**

- Agent files use `LintResult::new_arch()` for check failures (expected outcomes)
- File read failures → propagate Result or return explicit LintResult record
- Never silently discard errors with `unwrap_or_default()`

```rust
// [OK] LintResult for check failures (not IO failures)
fn check_imports(...) -> Vec<LintResult> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return vec![LintResult::new_arch(
            "PARSE_ERROR", &format!("Cannot read: {}", e), path.clone()
        )],
    };
    // Import check failure -> LintResult (expected outcome)
}

// [OK] Agent error handling pattern
let result = match checker.check() {
    Ok(r) => r,
    Err(e) => {
        violations.push(LintResult::new_arch(
            "CHECK_ERROR", &e.to_string(), file_path.clone()
        ));
        continue;  // ← Continue pipeline, don't abort
    }
};
```

## Magic Constant Extraction (from fix-magic-constant)

**Agent Layer Constant Rules:**

- NO hardcoded literals in agent layer
- All domain values MUST be named constants
- Constants MUST live in `taxonomy_*_constant.rs`

```rust
// [FORBIDDEN] BEFORE
let result = self.process(fps: 24);  // magic number

// [OK] AFTER
use crate::taxonomy_animator_constant::FPS_DEFAULT;
let result = self.process(fps: FPS_DEFAULT);
```

## Import Strategy (from fix-imports)

**Agent Layer Import Rules:**

```
ALLOWED:    contract_* (protocol/port/aggregate traits), taxonomy_*
FORBIDDEN:  capabilities_*, infrastructure_*, surface_* — NEVER import concrete structs
```

### Agent Pattern (DI via Contract)

```rust
// WRONG:
// agent_import_orchestrator.rs
use crate::capabilities_mandatory_checker::MandatoryChecker;  // FORBIDDEN
use crate::infrastructure_parser_adapter::ParserAdapter;      // FORBIDDEN

// CORRECT:
// agent_import_orchestrator.rs
use crate::contract_mandatory_checker_protocol::IMandatoryCheckerProtocol;
use crate::contract_parser_port::IParserPort;

pub struct ImportOrchestrator {
    checker: Arc<dyn IMandatoryCheckerProtocol>,
    parser: Arc<dyn IParserPort>,
}

impl ImportOrchestrator {
    pub fn new(
        checker: Arc<dyn IMandatoryCheckerProtocol>,
        parser: Arc<dyn IParserPort>,
    ) -> Self {
        Self { checker, parser }
    }
}
```

### Circular Dependencies (from fix-imports)

Circular dependencies can occur when Agent ↔ Capabilities create unresolvable cycles:

```rust
// VIOLATION: Cycle between agent and capabilities
// agent_runner.rs
use crate::capabilities_analyzer::Analyzer;  // Agent → Cap

// capabilities_analyzer.rs  
use crate::agent_runner::Runner;  // Cap → Agent (CYCLE!)
```

**Fix — Break the cycle by using traits:**

```rust
// contract_runner_protocol.rs (NEW)
pub trait IRunnerProtocol: Send + Sync {
    fn run(&self) -> Result<(), Error>;
}

// capabilities_analyzer.rs
use crate::contract_runner_protocol::IRunnerProtocol;  // → Contract (ALLOWED)
pub struct Analyzer { runner: Arc<dyn IRunnerProtocol> }

// agent_runner.rs implements protocol
impl IRunnerProtocol for Runner { fn run(&self) -> Result<(), Error> { /* ... */ } }
```

## Common Mistakes (AVOID)

- ❌ **Putting computation in agents**: Arithmetic, `sum()`, `len()`, and data transformation MUST be in capabilities layer.
- ❌ **Putting I/O in agents**: File reads, network calls, and database queries MUST be in infrastructure layer.
- ❌ **Putting business logic in agents**: Domain rules, validation, and computation MUST be in capabilities layer.
- ❌ **Defining data structs in layer files**: Domain data classes must be in shared/taxonomy. Only the impl struct belongs in layer files.
- ❌ **Using concrete types as fields**: Impl struct fields should always be `Arc<dyn Trait>` (DI), never concrete implementations.
- ❌ **Putting private helpers in the aggregate**: This violates encapsulation and forces all implementors to write boilerplate.
- ❌ **Mixing Block 2 and Block 3**: Do not interleave aggregate methods and private helpers. Keep them in separate `impl` blocks.
- ❌ **Placing utilities in Block 3**: Stateless free functions (no `&self`) MUST be extracted to standalone `*_utility.rs` modules. They do NOT belong in the impl block.
- ❌ **Creating "God Aggregates"**: If an aggregate has >10 methods or mixes unrelated concerns, split it into multiple aggregates.
- ❌ **Forgetting `where Self: Sized`**: This will break `dyn Trait` usage for the rest of the aggregate.
- ❌ **Placing `new()` in the aggregate impl**: Constructors must stay in the inherent `impl Struct` block (Block 3).
- ❌ **Multiple impl structs in one file**: Each file should have exactly ONE impl struct. Use `consolidate-files-rust` if merging multiple files.
