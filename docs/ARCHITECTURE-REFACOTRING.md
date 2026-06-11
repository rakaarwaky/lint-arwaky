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

- **Single crate** — tidak ada multi-workspace, satu Cargo.toml
- **Feature-based folders** — setiap feature punya folder sendiri
- **Container inside feature folder** — `{feature}/_container.rs`
- **Consistent structure** — setiap feature punya pola folder yang sama
- **Main entry compose containers** — direct composition, no aggregator file
- **No God Trait** — hapus `ServiceContainerAggregate`, pakai typed orchestrators

### New File Structure

**Pattern per feature folder** (contoh: `auto-fix/`):

```
crates/
├── auto-fix/                          ← feature folder
│   ├── auto_fix_container.rs          ← container (wiring only)
│   └── src/
│       ├── mod.rs
│       ├── contract_fix_aggregate.rs  ← contract/aggregate
│       ├── contract_fix_protocol.rs   ← protocol traits
│       ├── capabilities_fix_processor.rs  ← capabilities
│       ├── agent_fix_orchestrator.rs  ← orchestrator (agent)
│       ├── taxonomy_fix_applied_event.rs  ← taxonomy (VO/event)
│       └── taxonomy_fix_vo.rs         ← taxonomy (VO)
│
├── import-rules/
│   ├── import_container.rs
│   └── src/
│       ├── mod.rs
│       ├── contract_import_aggregate.rs
│       ├── contract_import_protocol.rs
│       ├── capabilities_import_mandatory.rs
│       ├── capabilities_import_forbidden.rs
│       ├── capabilities_import_intent.rs
│       ├── infrastructure_import_parser.rs
│       ├── agent_import_orchestrator.rs
│       └── taxonomy_rule_vo.rs
│
├── naming-rules/
│   ├── naming_container.rs
│   └── src/
│       └── ...
│
├── role-rules/
│   ├── role_container.rs
│   └── src/
│       └── ...
│
├── cli/
│   └── main.rs                       ← entry point
│
├── mcp/
│   └── main.rs                       ← entry point
│
└── tui/
    └── main.rs                       ← entry point
```

### Key Changes

| Aspect | Before | After |
|--------|--------|-------|
| **Structure** | Flat files di `src-rust/` | Feature folders dalam `crates/` |
| **Container** | `di-containers/` (God folder) | `{feature}/{feature}_container.rs` |
| **Container location** | `di-containers/agent_injection_container.rs` | `crates/{feature}/{feature}_container.rs` |
| **Container trait** | Implements `ServiceContainerAggregate` | **No trait impl** |
| **Container purpose** | Wire + logic + trait impl | **Wire only** |
| **Source files** | Flat di `src-rust/{feature}/` | `crates/{feature}/src/` |
| **Contract** | `ServiceContainerAggregate` (God trait) | Typed orchestrators per feature |
| **Composition** | Single God Container | Main entry composes directly |
| **Surface call** | `container.get_xxx()` | `orchestrator.run_audit()` |

### Taxonomy Categories

Taxonomy layer punya 7 kategori suffix files:

| Suffix | Category | Aturan | Contoh |
|--------|----------|--------|--------|
| `_vo` | Value Object | Strict — harus wrap primitive, tidak boleh ada logic | `taxonomy_line_number_vo.rs` |
| `_entity` | Entity | Strict — represent domain entity | `taxonomy_user_entity.rs` |
| `_error` | Error type | Strict — custom error enum/struct | `taxonomy_lint_error.rs` |
| `_event` | Event | Strict — domain events | `taxonomy_fix_applied_event.rs` |
| `_constant` | Constant | Strict — hanya `pub const` / `pub static` | `taxonomy_layer_names_constant.rs` |
| `_utility` | **Utility** | **Relaxed** — boleh punya logic, boleh dipakai oleh layer manapun | `taxonomy_path_utility.rs` |
| `_helper` | **Helper** | **Relaxed** — boleh punya logic, boleh dipakai oleh layer manapun | `taxonomy_format_helper.rs` |

**Perbedaan Utama:**

| Aspek | Strict (vo/entity/error/event/constant) | Relaxed (util/helper) |
|-------|----------------------------------------|----------------------|
| **Logic** | Tidak boleh ada logic (hanya data) | Boleh ada logic |
| **Import** | Hanya boleh di-import oleh layer bawah | Boleh di-import oleh layer manapun |
| **Contract trait** | Wajib implement jika di-capabilities | Tidak wajib implement contract trait |
| **Usage** | Pure data representation | Shared utilities untuk multiple layers |

**Contoh `taxonomy_path_utility.rs`:**
```rust
// Boleh punya logic — relaxed rules
pub fn normalize_path(path: &str) -> String {
    path.replace("\\", "/")
}

pub fn is_source_file(path: &str) -> bool {
    path.ends_with(".rs") || path.ends_with(".py") || path.ends_with(".ts")
}
```

**Contoh `taxonomy_format_helper.rs`:**
```rust
// Boleh dipakai oleh capabilities DAN infrastructure
pub fn format_violation(code: &str, message: &str) -> String {
    format!("[{}] {}", code, message)
}
```

---

## 3. Detailed Component Design

### 3.1 Feature Container (wiring only)

Setiap feature punya 1 container file di dalam folder-nya: `{feature}/_container.rs`.

```rust
// import-rules/_container.rs — wiring import-rules feature
use std::sync::Arc;
use crate::import_rules::src::_protocol::{IArchImportProtocol, IImportParserPort};

pub struct ImportContainer {
    mandatory: Arc<dyn IArchImportProtocol>,
    forbidden: Arc<dyn IArchImportProtocol>,
}

impl ImportContainer {
    pub fn new() -> Self {
        let parser: Arc<dyn IImportParserPort> = Arc::new(
            crate::import_rules::src::infrastructure_import_parser::ImportParserAdapter::new(),
        );
        Self {
            mandatory: Arc::new(
                crate::import_rules::src::capabilities_import_mandatory::ImportMandatoryChecker::new(parser.clone()),
            ),
            forbidden: Arc::new(
                crate::import_rules::src::capabilities_import_forbidden::ImportForbiddenChecker::new(parser),
            ),
        }
    }

    pub fn orchestrator(&self) -> ImportOrchestrator {
        ImportOrchestrator::new(
            self.mandatory.clone(),
            self.forbidden.clone(),
        )
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

---

## 10. Circular Dependencies (BLOCKER untuk Multi-Crate Workspace)

### Temuan

Ada **11 circular dependencies** yang menghalangi migrasi ke multi-crate workspace:

| # | Cycle | Types yang cross-import |
|---|-------|------------------------|
| 1 | `shared-common` ↔ `import-rules` | `MandatoryImportRuleVO` |
| 2 | `shared-common` ↔ `naming-rules` | `SuffixPolicyVO` |
| 3 | `shared-common` ↔ `pipeline-jobs` | `JobId`, `ResponseData` |
| 4 | `shared-common` ↔ `output-report` | `Severity` |
| 5 | `source-parsing` ↔ `code-analysis` |双向 import |
| 6 | `source-parsing` ↔ `language-adapters` |双向 import |
| 7 | `source-parsing` ↔ `file-system` |双向 import |
| 8 | `cli-commands` ↔ `cli-transport` |双向 import |
| 9 | `config-system` ↔ `di-containers` |双向 import |
| 10 | `code-analysis` ↔ `di-containers` |双向 import |
| 11 | `naming-rules` ↔ `import-rules` |双向 import |

### Solusi: Phase 0 — Break Circular Dependencies

**Sebelum bisa multi-crate workspace, harus fix ini dulu:**

1. **Pindahkan shared types ke `shared-common`**
   - `Severity` dari `output-report`
   - `JobId`, `ResponseData` dari `pipeline-jobs`
   - `FilePath`, `DirectoryPath` dari `source-parsing`
   - `MandatoryImportRuleVO` dari `import-rules`
   - `SuffixPolicyVO` dari `naming-rules`

2. **Buang `di-containers`** — ganti dengan per-feature containers
   - `di-containers` import semua 23 features → ini yang bikin cycle
   - Ganti dengan `{feature}_container.rs` yang hanya wire 1 feature

3. **Break `source-parsing` ↔ `code-analysis` cycle**
   - Pindahkan `FilePath` ke `shared-common`
   - Buat `source-parsing` independent

### Estimated Effort

| Phase | Effort | Description |
|-------|--------|-------------|
| Phase 0 | 8-12 jam | Break circular dependencies |
| Phase 1 | 4-6 jam | Create Cargo.toml per feature |
| Phase 2 | 2-4 jam | Setup workspace root Cargo.toml |
| Phase 3 | 4-6 jam | Update all imports |
| **Total** | **18-28 jam** | |

### Status

**BLOCKED** — Circular dependencies harus di-fix dulu sebelum multi-crate workspace bisa jalan.

**Saat ini:** Single crate dengan feature-based folders (sudah berjalan).
