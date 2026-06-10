# Refactor: Container Wiring Only — Orchestrator Calls Protocols Directly

## Context

Lint Arwaky uses 6-layer AES architecture. Currently there's an architectural violation where **containers** (`_container`) are doing both DI wiring AND calling protocol methods. This is wrong.

### Correct AES Architecture

```
Surface → calls Aggregate → Orchestrator → calls Protocol/Port → implemented by Capabilities/Infrastructure
Container → ONLY wires dependencies (DI), NO logic
```

### Current (Wrong)

```
Orchestrator → self.checker.check_file_naming() → CheckerContainer (implements ICheckerAggregate)
  → self.naming_checker.check_file_naming() → ArchNamingChecker (protocol)
```

Container is doing orchestration by calling protocol methods. This violates the rule that `_container` = DI wiring only.

### Target (Correct)

```
Orchestrator → self.naming_checker.check_file_naming() → ArchNamingChecker (protocol)
Container → only provides self.naming_checker via DI
```

Orchestrator calls protocols directly. Container only wires dependencies.

---

## Task

Refactor `agent_checker_container.rs` and `agent_checking_orchestrator.rs` so that:

1. **Container (`di-containers/agent_checker_container.rs`)**: Only does DI wiring. Creates all checker instances and provides them via getters. NO method calls to checkers.
2. **Orchestrator (`code-analysis/agent_checking_orchestrator.rs`)**: Calls protocol methods directly on the checker instances. Gets checker instances from container via getters or DI.
3. **Remove `ICheckerAggregate` trait** (or refactor it): This trait forces the container to implement all checker methods, which makes the container do orchestration. Instead, the orchestrator should hold references to individual protocols.

---

## Files to Modify

### 1. `src-rust/di-containers/agent_checker_container.rs`

**Current**: Implements `ICheckerAggregate` with 30+ methods that delegate to checkers.

**Target**:

- Struct with all checker fields (already exists)
- `new()` constructor that creates all checkers (already exists)
- Getter methods for each checker: `naming_checker()`, `import_checker()`, etc.
- NO `check_file_naming()`, `check_domain_suffixes()`, etc. methods

```rust
pub struct CheckerContainer {
    naming_checker: ArchNamingChecker,
    import_mandatory_checker: ArchImportMandatoryChecker,
    import_forbidden_checker: ArchImportForbiddenChecker,
    // ... etc
}

impl CheckerContainer {
    pub fn new(config: ArchitectureConfig) -> Self { ... }
  
    // Getters only — no logic
    pub fn naming_checker(&self) -> &ArchNamingChecker { &self.naming_checker }
    pub fn import_mandatory_checker(&self) -> &ArchImportMandatoryChecker { &self.import_mandatory_checker }
    pub fn import_forbidden_checker(&self) -> &ArchImportForbiddenChecker { &self.import_forbidden_checker }
    pub fn layer_analyzer(&self) -> &ArchComplianceAnalyzer { &self.analyzer }
    pub fn orphan_aggregate(&self) -> &Arc<dyn IOrphanAggregate> { &self.orphan_analyzer }
    // ... etc
}
```

### 2. `src-rust/code-analysis/agent_checking_orchestrator.rs`

**Current**: Calls `self.checker.check_file_naming(...)` where `self.checker` is `Arc<dyn ICheckerAggregate>`.

**Target**:

- Holds individual checker references (or gets them from container)
- Calls protocols directly

```rust
pub struct LintCheckingOrchestrator {
    container: Arc<CheckerContainer>,
}

impl LintCheckingOrchestrator {
    pub fn run_all_checks(&self, config: &ArchitectureConfig, files: &[String], root_dir: &str) -> Vec<LintResult> {
        // ...
        // Instead of: self.checker.check_file_naming(...)
        // Do: self.container.naming_checker().check_file_naming(...)
        self.container.naming_checker().check_file_naming(file, filename, &layer, Some(&def), config, &mut violations);
        self.container.import_mandatory_checker().check_mandatory_imports(file, &def, &mut violations);
        self.container.import_forbidden_checker().check_forbidden_imports(file, &layer, &def, &mut violations);
        // ... etc
    }
}
```

### 3. `src-rust/code-analysis/contract_checker_aggregate.rs`

**Current**: Defines `ICheckerAggregate` trait with 30+ methods.

**Target**: Either:

- **Option A**: Remove entirely. Orchestrator uses individual protocols directly.
- **Option B**: Keep as a thin wrapper that only provides getters (not method delegation).

Recommended: **Option A** — remove `ICheckerAggregate`. The orchestrator should use individual protocol traits directly.

### 4. `src-rust/di-containers/contract_service_aggregate.rs`

Check if `ServiceContainerAggregate` also forces method delegation. If yes, refactor similarly.

---

## Rules to Follow

1. **Container suffix = `_container`**: Only DI wiring, NO logic, NO method calls to protocols
2. **Orchestrator suffix = `_orchestrator`**: Stateless conductor, calls protocols directly
3. **Protocol suffix = `_protocol`**: Interface for capability use-cases
4. **Port suffix = `_port`**: Interface for infrastructure adapters
5. **Aggregate suffix = `_aggregate`**: Facade contract for service containers

## Verification

After refactoring:

```bash
# Build
cargo build --release

# Self-lint — check AES030 orphan violations reduced
cargo run --bin lint-arwaky-cli -- check . --filter AES030

# Run tests
cargo test --workspace
```

The AES030 violations for "Contract protocol not called by any orchestrator" should be resolved because orchestrators now call protocols directly.

## Important Notes

- Do NOT change the protocol/port implementations (capabilities_* files)
- Do NOT change the checker logic — only move WHERE the methods are called
- The container struct fields and constructor can stay the same
- Only remove the `impl ICheckerAggregate for CheckerContainer` block
- Update the orchestrator to get checkers from container and call them directly
