# Architecture Refactoring: Decentralized Container Pattern

> **Status:** APPROVED & IN PROGRESS
> **Date:** 2026-06-11
> **Scope:** Full DI architecture refactor & Agent specialization

---

## 1. Current Architecture (Before)

### Problem

Saat ini semua wiring ada di `di-containers/` folder dengan satu "God Container" (`DependencyInjectionContainer`) yang:

- Implement `ServiceContainerAggregate` trait (31 methods)
- Wire **semua** features dalam satu file
- Import dari **semua** feature folders
- Surface commands panggil container langsung

```
src-rust/
├── di-containers/
│   ├── contract_service_aggregate.rs    ← God trait (31 methods)
│   ├── agent_injection_container.rs     ← God container (500+ lines, classified under agent layer)
│   ├── agent_checker_container.rs       ← Checker wiring (classified under agent layer)
│   └── mod.rs
│
├── cli_main_entry.rs                    ← create container, pass to commands
├── mcp_main_entry.rs                    ← create container per request
└── tui_main_entry.rs
```

### Issues

| Issue                             | Dampak                                         |
| --------------------------------- | ---------------------------------------------- |
| God Container                     | 1 file 500+ lines, mem-wire 30+ adapters       |
| God Trait                         | `ServiceContainerAggregate` memiliki 31 method |
| Surface import container langsung | Melanggar dependency rule                      |
| Container implement contract      | Contract tidak eksklusif untuk orchestrator    |
| Agent Layer Pollution             | Suffix `_container` mengaburkan tanggung jawab inti `agent` (`_orchestrator`) |

---

## 2. Proposed Architecture (After)

### Principle

- **Container = wiring only** (no trait impl, no logic)
- **Container location = root** (`src-rust/` directly, sejajar main entry files)
- **Container naming = `{feature}_container.rs`** (ends with `_container`, classified under `root` layer)
- **Main entry compose containers** (no aggregator file, direct composition)
- **Agent Layer Specialization** (hanya `_orchestrator` yang diperbolehkan di layer `agent`)
- **Explicit Feature Runner Traits (Opsi B - Recommended)** (menghindari generic polymorphic `ILintRunner` yang merusak type safety untuk feature dengan input/output spesifik seperti `security` atau `fix`)

### New File Structure

```
src-rust/
│
├── cli_main_entry.rs              ← compose: ImportContainer + NamingContainer + ...
├── mcp_main_entry.rs              ← compose: ImportContainer + McpContainer + ...
├── tui_main_entry.rs              ← compose: ImportContainer + NamingContainer + ...
│
├── import_container.rs            ← wiring import-rules feature [NEW - root layer]
├── naming_container.rs            ← wiring naming-rules feature [NEW - root layer]
├── role_container.rs              ← wiring role-rules feature [NEW - root layer]
├── analysis_container.rs          ← wiring code-analysis feature [NEW - root layer]
├── mcp_container.rs               ← wiring mcp-server feature [NEW - root layer]
├── git_container.rs               ← wiring git-hooks feature [NEW - root layer]
├── pipeline_container.rs          ← wiring pipeline-jobs feature [NEW - root layer]
├── config_container.rs            ← wiring config-system feature [NEW - root layer]
├── file_container.rs              ← wiring file-system feature [NEW - root layer]
├── plugin_container.rs            ← wiring plugin-system feature [NEW - root layer]
├── orphan_container.rs            ← wiring orphan-detector feature [NEW - root layer]
├── auto_fix_container.rs          ← wiring auto-fix feature [NEW - root layer]
│
├── import-rules/                  ← feature folder
│   └── contract_import_runner_aggregate.rs   ← Inbound port/runner aggregate for import
├── naming-rules/                  ← feature folder
│   └── contract_naming_runner_aggregate.rs   ← Inbound port/runner aggregate for naming
├── code-analysis/                 ← feature folder
│   └── contract_fix_runner_aggregate.rs      ← Inbound port/runner aggregate for auto-fix
│
└── shared-common/
```

### Key Changes

| Aspect                       | Before                                   | After                        |
| ---------------------------- | ---------------------------------------- | ---------------------------- |
| **Container location**       | `di-containers/` folder                  | Root (`src-rust/`)           |
| **Container naming**         | `agent_injection_container.rs`           | `{feature}_container.rs`     |
| **Container layer**          | `agent` layer                            | `root` layer                 |
| **Container trait**          | Implements `ServiceContainerAggregate`   | **No trait impl**            |
| **Container purpose**        | Wire + logic + trait impl                | **Wire only**                |
| **Contract**                 | `ServiceContainerAggregate` (God trait)  | Feature runner aggregates    |
| **Composition**              | Single God Container                     | Main entry composes directly |
| **Surface call**             | `container.get_xxx()`                    | `runners.run_audit()`        |

---

## 3. Detailed Component Design

### 3.1 Feature Container (wiring only)

Setiap feature memiliki 1 container file di root (`src-rust/`).

```rust
// import_container.rs — wiring import-rules feature
use std::sync::Arc;
use crate::contract::{IArchImportProtocol, ImportIntentProtocol, IImportParserPort, IImportRunnerAggregate};
use crate::import_rules::{ImportMandatoryChecker, ImportForbiddenChecker, ImportIntentChecker, ImportParserAdapter, ImportOrchestrator};

pub struct ImportContainer {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    intent: Arc<dyn ImportIntentProtocol>,
}

impl ImportContainer {
    pub fn new() -> Self {
        let parser: Arc<dyn IImportParserPort> = Arc::new(ImportParserAdapter::new());
        Self {
            mandatory: Arc::new(ImportMandatoryChecker::new(parser.clone())),
            forbidden: Arc::new(ImportForbiddenChecker::new(parser.clone())),
            intent: Arc::new(ImportIntentChecker::new(parser)),
        }
    }

    pub fn orchestrator(&self) -> Arc<dyn IImportRunnerAggregate> {
        Arc::new(ImportOrchestrator::new(
            self.mandatory.clone(),
            self.forbidden.clone(),
            self.intent.clone(),
        ))
    }
}
```

### 3.2 Feature Orchestrator (implements feature runner aggregate)

```rust
// agent/import_orchestrator.rs
use std::sync::Arc;
use crate::contract::{IArchImportProtocol, ImportIntentProtocol, IImportRunnerAggregate};

pub struct ImportOrchestrator {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
    intent: Arc<dyn ImportIntentProtocol>,
}

impl ImportOrchestrator {
    pub fn new(
        mandatory: Arc<dyn IArchImportProtocol>,
        forbidden: Arc<dyn IArchImportProtocol>,
        intent: Arc<dyn ImportIntentProtocol>,
    ) -> Self {
        Self { mandatory, forbidden, intent }
    }
}

#[async_trait]
impl IImportRunnerAggregate for ImportOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let mut results = Vec::new();
        // Orchestrate checkers
        results.extend(self.mandatory.check_mandatory_imports(target).await);
        results.extend(self.intent.check_import_intent(target).await);
        results.extend(self.forbidden.check_forbidden_imports(target).await);
        results
    }

    fn name(&self) -> &str { "import-rules" }
}
```

### 3.3 Contracts (Feature Runner Aggregates)

Alih-alih memaksakan satu generic trait yang membuang type safety, kita mendefinisikan aggregate runner trait spesifik per feature di layer `contract_` masing-masing feature:

```rust
// import-rules/contract_import_runner_aggregate.rs
#[async_trait]
pub trait IImportRunnerAggregate: Send + Sync {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult>;
    fn name(&self) -> &str;
}
```

```rust
// code-analysis/contract_fix_runner_aggregate.rs
#[async_trait]
pub trait IFixRunnerAggregate: Send + Sync {
    // Memiliki output type yang berbeda (FixReport) dan input parameter tambahan (dry_run)
    async fn run_fix(&self, target: &FilePath, dry_run: bool) -> FixReport;
    fn name(&self) -> &str;
}
```

---

## 4. Main Entry Points (Composition Root)

Main entry points bertanggung jawab **menyusun container** dan **pass runners ke surface commands**. Main entry **tidak memanggil orchestrator langsung** — surface yang melakukannya:

```rust
// cli_main_entry.rs
fn main() {
    let args = CliArgs::parse();
  
    // Step 1: Compose containers
    let import_container = ImportContainer::new();
    let naming_container = NamingContainer::new();
    let fix_container = AutoFixContainer::new();
  
    // Step 2: Create runners (typed per feature)
    let import_runner = import_container.orchestrator();
    let naming_runner = naming_container.orchestrator();
    let fix_runner = fix_container.orchestrator();
  
    // Step 3: Pass runners ke surface command (surface yang call orchestrator)
    match args.command {
        "check" => handle_check(&import_runner, &naming_runner, &args.path),
        "fix" => handle_fix(&fix_runner, &args.path, args.dry_run),
        _ => {}
    }
}
```

```rust
// surface/check_command.rs — surface yang memanggil orchestrator
pub fn handle_check(
    import_runner: &dyn IImportRunnerAggregate,
    naming_runner: &dyn INamingRunnerAggregate,
    path: &str,
) -> ExitCode {
    let rt = tokio::runtime::Runtime::new().unwrap();
    let mut results = Vec::new();
    
    // Surface call orchestrator langsung
    results.extend(rt.block_on(import_runner.run_audit(&path_obj)));
    results.extend(rt.block_on(naming_runner.run_audit(&path_obj)));
    
    report_violations(&results)
}
```

---

## 5. Dependency Flow

```
cli_main_entry.rs
    │
    ├── ImportContainer::new()          ← compose (wiring)
    ├── NamingContainer::new()          ← compose (wiring)
    │
    └── pass runners ke surface ──────→ surface/check_command.rs
                                           │
                                           ├── import_runner.run_audit()    ← surface call orchestrator
                                           ├── naming_runner.run_audit()    ← surface call orchestrator
                                           │
                                           └── report_violations()
```

**Flow:**
```
Main Entry → compose containers → create runners → pass ke Surface
Surface → call runners (orchestrator) → orchestrator call checkers → results
```

**Dependency Rule:**

```
surface        → contract (I*RunnerAggregate)
agent          → contract + taxonomy
capabilities   → contract + taxonomy
infrastructure → contract + taxonomy
taxonomy       → (nothing)
```

**Container:**

```
container → capabilities (checker protocols)
container → creates orchestrator
container → NO contract impl
```

---

## 6. Migration Plan

### Phase 1: Create feature runner aggregate contracts + feature containers

| Step | File                                                 | Action                                            |
| ---- | ---------------------------------------------------- | ------------------------------------------------- |
| 1.1  | `import-rules/contract_import_runner_aggregate.rs`   | Create `IImportRunnerAggregate` trait             |
| 1.2  | `naming-rules/contract_naming_runner_aggregate.rs`   | Create `INamingRunnerAggregate` trait             |
| 1.3  | `code-analysis/contract_fix_runner_aggregate.rs`     | Create `IFixRunnerAggregate` trait                |
| 1.4  | `import_container.rs`                                | Create ImportContainer (wire import checkers)     |
| 1.5  | `naming_container.rs`                                | Create NamingContainer (wire naming checkers)     |
| 1.6  | ... other feature containers                         | Create remaining containers                       |

### Phase 2: Create feature orchestrators

| Step | File                                           | Action                                       |
| ---- | ---------------------------------------------- | -------------------------------------------- |
| 2.1  | `import-rules/agent_import_orchestrator.rs`    | Create ImportOrchestrator impl IImportRunnerAggregate |
| 2.2  | `naming-rules/agent_naming_orchestrator.rs`    | Create NamingOrchestrator impl INamingRunnerAggregate |
| 2.3  | `code-analysis/agent_fix_orchestrator.rs`      | Create FixOrchestrator impl IFixRunnerAggregate |
| 2.4  | ... other orchestrators                        | Create remaining orchestrators               |

### Phase 3: Migrate main entry points

| Step | File                | Action                                       |
| ---- | ------------------- | -------------------------------------------- |
| 3.1  | `cli_main_entry.rs` | Compose containers, pass runners to commands |
| 3.2  | `mcp_main_entry.rs` | Compose containers once at startup           |
| 3.3  | `tui_main_entry.rs` | Compose containers                           |

### Phase 4: Update surface commands

| Step | File                       | Action                                         |
| ---- | -------------------------- | ---------------------------------------------- |
| 4.1  | `surface/check_command.rs` | Change from `container.get_xxx()` to `runners` |
| 4.2  | `surface/fix_command.rs`   | Change from `container.get_xxx()` to `runners` |
| 4.3  | ... other surfaces         | Update all surface commands                    |

### Phase 5: Remove old DI infrastructure

| Step | File                                          | Action                                  |
| ---- | --------------------------------------------- | --------------------------------------- |
| 5.1  | `di-containers/contract_service_aggregate.rs` | Delete (replaced by Feature Aggregates) |
| 5.2  | `di-containers/agent_injection_container.rs`  | Delete (replaced by feature containers) |
| 5.3  | `di-containers/agent_checker_container.rs`    | Delete (replaced by feature containers) |
| 5.4  | `di-containers/mod.rs`                        | Delete folder                           |

---

## 7. AES Rules Enforcement

### New Rules

| Rule      | Check                                                        | Severity |
| --------- | ------------------------------------------------------------ | -------- |
| **AES0307** | **Container Purity**: Container tidak boleh implement contract trait, dan hanya boleh berisi logic instantiation wiring saja (`new()` + orchestrator factory method). | HIGH     |
| **AES0308** | **Surface Decoupling**: Surface harus memanggil orchestrator via explicit runner aggregates (e.g. `IImportRunnerAggregate`), dan dilarang mengimport atau menggunakan class container secara langsung. | HIGH     |

### Existing Rules Updated

| Rule       | Change                                                                                |
| ---------- | ------------------------------------------------------------------------------------- |
| **AES001** | Container layer = root (bukan agent). Hanya diperbolehkan berada di root folder `src-rust/`. |
| **AES011** | Naming pattern container wajib berupa `{feature}_container.rs` (lowercase, underscore). |
| **AES012** | Suffix `_container` hanya diperbolehkan untuk `root` layer. Suffix `_lifecycle` dihapus sepenuhnya. Suffix `agent` dibatasi secara ketat hanya untuk `_orchestrator`. |

---

## 8. Benefits

- **Strict Type Safety**: Setiap feature memiliki input/output signature yang sesuai dengan kebutuhannya sendiri tanpa memaksakan generic wrapper.
- **No God Object**: Setiap container berkisar ≤50 lines dan hanya me-wire dependencies untuk 1 feature.
- **Eksplisit**: Main entry point secara transparan mendefinisikan apa saja dependencies dan runners yang digunakan.
- **Flexible Composition**: Tiap entry point menyusun container secara modular.
- **Testable**: Mempermudah mocking runner aggregate per feature secara independen di level surface testing.

---

## 9. Risk & Mitigation

| Risk                        | Mitigation                                                      |
| --------------------------- | --------------------------------------------------------------- |
| Banyak file container       | Setiap container berkisar ≤50 lines, sangat mudah dipahami.     |
| Migration complexity        | Migrasi bertahap, container lama tetap aktif sampai Phase 5.     |
| Breaking changes            | Menjaga kompatibilitas dengan membiarkan deprecated API sementara. |
| Feature containers coupling | Tiap container bersifat independen, dilarang melakukan cross-import. |
