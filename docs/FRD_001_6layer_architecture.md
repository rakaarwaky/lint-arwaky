# FRD — 6-Layer AES Architecture (Core Platform)

> **PRD Reference**: [FR-001](PRD.md) — 6-layer AES architecture enforcement
> **Dependency**: — (Foundation, no prior dependency)
> **Status**: ✅ COMPLETE — Implemented as the core architecture of the project
> **Self-lint**: `lint-arwaky-cli check .` — project audits itself under this architecture

## 1. Problem Statement

Before the 6-layer AES architecture, Lint Arwaky had:

| Issue | Description |
|-------|-------------|
| **No layer boundaries** | All code lived in flat directories without separation of concerns |
| **Circular dependencies** | Infrastructure could import surfaces, capabilities could import infrastructure — no compile-time guard |
| **No naming conventions** | Filenames were inconsistent — no suffix-based layer identification |
| **No import rules** | Any module could import any other module, creating spaghetti dependencies |
| **No self-audit** | Architecture violations were undetectable without manual review |
| **No DI contract** | Surfaces bypassed the DI container and imported infrastructure/capabilities directly |

## 2. Konsep Dasar

AES membagi kode ke 6 layer vertikal dengan aturan ketat:

```
Semakin ke atas → semakin dekat ke user (CLI, MCP)
Semakin ke bawah → semakin dekat ke domain murni (VO, Entity)
```

**Aturan emas**: Layer atas bisa pakai layer bawah, tapi layer bawah TIDAK boleh tahu layer atas.

```
Surfaces ──► Contract ──► Taxonomy
Agent ──► Capabilities + Infrastructure ──► Contract ──► Taxonomy
```

### Sibling Equivalence
Capabilities dan Infrastructure adalah **peer**. Mereka tidak boleh saling import. Keduanya hanya bicara lewat Contract (Ports/Protocols).

### Dependency Inversion
Surfaces tidak pernah import implementasi konkret. Mereka cuma megang `ServiceContainerAggregate` (trait di contract). Implementasi disuntik lewat DI container.

## 3. Mekanisme Kerja — Step by Step

### 3.1 Layer Detection (`architecture_compliance_analyzer.rs`)

Setiap file di-scan, layer-nya dideteksi dengan algoritma ini:

```
FilePath: "src-rust/capabilities/architecture_import_checker.rs"

Step 1: detect_layer()
  ├── Cek prefix path:
  │     "src-rust/taxonomy/"  → LAYER_TAXONOMY
  │     "src-rust/contract/"  → LAYER_CONTRACT
  │     "src-rust/capabilities/" → LAYER_CAPABILITIES  ← MATCH
  │     "src-rust/infrastructure/" → LAYER_INFRASTRUCTURE
  │     "src-rust/agent/"     → LAYER_AGENT
  │     "src-rust/surfaces/"  → LAYER_SURFACES
  │     "src/" atau root → LAYER_ROOT
  │
Step 2: resolve_specialized_layer()
  └── Cocokkan suffix file:
        "_vo"       → taxonomy(vo)
        "_entity"   → taxonomy(entity)
        "_error"    → taxonomy(error)
        "_event"    → taxonomy(event)
        "_constant" → taxonomy(constant)
        "_port"     → contract(port)
        "_protocol" → contract(protocol)
        "_aggregate" → contract(aggregate)
        "_checker"  → capabilities(checker)   ← MATCH
        "_analyzer" → capabilities(analyzer)
        ...
        (fallback → layer umum)

Result: LAYER_CAPABILITIES + sub-layer "checker"
```

### 3.2 Import Validation (`architecture_import_checker.rs`)

Setelah layer diketahui, impor di file divalidasi:

```
File: capabilities/architecture_import_checker.rs
Layer: capabilities

Untuk setiap "use ..." atau "import ..." di file:

1. check_forbidden_imports() → AES001
   ├── Cari tahu layer TARGET dari path import
   │     "use crate::infrastructure::..." → target = infrastructure
   │
   └── Cek aturan:
         capabilities BOLEH import: taxonomy, contract(protocol)
         capabilities TIDAK BOLEH import: infrastructure, surfaces, agent, capabilities(sibling)
         └── infrastructure ada di daftar terlarang → FLAG VIOLATION (AES001)

2. check_mandatory_imports() → AES002
   └── Capabilities WAJIB import: taxonomy, contract(protocol)
         └── Kalau taxonomy tidak diimport → FLAG VIOLATION (AES002)

3. AES023 (khusus surfaces):
   └── Surface cuma boleh akses infra/cap lewat ServiceContainerAggregate
         └── Kalau ada "use crate::infrastructure::..." langsung → FLAG
```

### 3.3 Layer-Gated Compilation (`src-rust/lib.rs`)

Compile-time guard via Cargo features:

```rust
// lib.rs
#[cfg(feature = "check_taxonomy")]
pub mod taxonomy;

#[cfg(feature = "check_contract")]
pub mod contract;   // depends on taxonomy

#[cfg(feature = "check_infrastructure")]
pub mod infrastructure;  // depends on contract

#[cfg(feature = "check_capabilities")]
pub mod capabilities;    // depends on contract

#[cfg(feature = "check_agent")]
pub mod agent;           // depends on capabilities + infrastructure + contract

#[cfg(feature = "check_surfaces")]
pub mod surfaces;        // depends on agent + contract
```

Rantai feature: `check_taxonomy` → `check_contract` → `check_infrastructure` / `check_capabilities` → `check_agent` → `check_surfaces` (default).

Kalau seseorang compile `--features check_agent` tanpa `check_capabilities`, compile **gagal** karena agent butuh capabilities. Ini mencegah circular dependency di compile time.

### 3.4 DI Container Wiring (`agent/dependency_injection_container.rs`)

```
Entry point (cli_main_entry.rs / mcp_main_entry.rs)
    │
    ▼
Buat semua implementasi konkret (infrastructure):
    ├── OSFileSystemAdapter
    ├── ASTRustParserAdapter
    ├── ASTPythonParserAdapter
    ├── ASTJSParserAdapter
    ├── ConfigYamlReader
    ├── LanguageDetectorProvider
    ├── PythonRuffAdapter
    ├── ...
    │
    ▼
Bungkus dalam Arc<dyn TraitContract>:
    ├── Arc<dyn IFileSystemPort>
    ├── Arc<dyn ISourceParserPort>
    ├── Arc<dyn IConfigReaderPort>
    ├── Arc<dyn ILinterAdapterPort>
    ├── ...
    │
    ▼
Buat capabilitiy (logic) dengan injected dependencies:
    ├── ArchitectureComplianceAnalyzer(IFileSystemPort)
    ├── ArchitectureImportChecker(ISourceParserPort)
    ├── ConfigOrchestrationProcessor(IConfigReaderPort, ILanguageDetectorPort)
    ├── ...
    │
    ▼
Buat ServiceContainerAggregate (contract) → implementasi konkret:
    └── DependencyInjectionContainer { semua Arc }
        │
        ▼
Inject ke surfaces (CLI, MCP) via Arc<dyn ServiceContainerAggregate>

Surface PANGGIL:   container.get_compliance_analyzer().detect_layer(path)
Surface TIDAK:     ArchitectureComplianceAnalyzer::new()  ← FORBIDDEN (AES023)
```

### 3.5 Self-Lint Flow

```
lint-arwaky-cli check .
    │
    ├─► ArchLintHandler.find_source_dir(".")
    │     └─► Cari: src-rust/? src-python/? src-javascript/? src/?
    │           → ketemu src-rust/ → language = Rust
    │
    ├─► ConfigLoaderOrchestrator.load_project_config(".")
    │     └─► Baca lint_arwaky.config.rust.yaml → ArchitectureConfig
    │
    ├─► LintCheckingCoordinator.run_all_checks("src-rust/")
    │     │
    │     ├─► Walk semua file .rs di src-rust/
    │     │
    │     ├─► Per file:
    │     │     ├─► detect_layer() → tahu file ini di layer apa
    │     │     ├─► extract_imports() → tahu file ini import apa aja
    │     │     ├─► check naming (AES003) → 3-word snake_case?
    │     │     ├─► check suffix (AES011) → suffix sesuai layer?
    │     │     ├─► check imports (AES001, AES002) → import boleh/tidak?
    │     │     ├─► check bypass (AES014) → ada #[allow(...)]?
    │     │     ├─► check inheritance (AES026, AES027) → implements contract?
    │     │     ├─► check primitives (AES006) → ada String/i32 di entity?
    │     │     ├─► check file size (AES004, AES005) → 10-500 lines?
    │     │     └─► ...
    │     │
    │     └─► Cross file:
    │           ├─► barrel completeness (AES012) → mod.rs export semua?
    │           ├─► circular dependencies (AES020) → ada cycle?
    │           └─► orphan detection (AES017) → file tidak di-refer?
    │
    └─► Kumpulkan semua violation → ArchitectureGovernanceEntity
          ├─► Score: 100 - (LOW*1 + MEDIUM*2 + HIGH*3 + CRITICAL*5)
          ├─► Kalau ada CRITICAL → auto fail
          └─► Cetak violation grouped by severity/rule
```

## 4. Layer Specification

| Layer | Directory | Suffixes | Boleh Import | Tidak Boleh Import |
|-------|-----------|----------|--------------|-------------------|
| **Surface (Smart)** | `surfaces/` | `_command`, `_handler`, `_controller`, `_entry` | taxonomy, contract(aggregate) | agent, capabilities, infrastructure, contract(port), contract(protocol) |
| **Surface (Utility)** | `surfaces/` | `_hook`, `_store`, `_provider`, `_router` | taxonomy, contract(aggregate) | agent, capabilities, infrastructure, Smart surfaces |
| **Surface (Passive)** | `surfaces/` | `_component`, `_layout`, `_view` | taxonomy | agent, contract, capabilities, infrastructure, semua surface lain |
| **Agent (Container)** | `agent/` | `_container`, `_registry`, `_mixin` | taxonomy, contract | surfaces |
| **Agent (Orchestrator)** | `agent/` | `_orchestrator`, `_coordinator`, `_dispatcher` | taxonomy, contract(aggregate) | surfaces, agent siblings |
| **Agent (Support)** | `agent/` | `_manager`, `_handler`, `_result`, `_state` | taxonomy | agent lain, infrastructure, capabilities, surfaces |
| **Capabilities** | `capabilities/` | `_checker`, `_analyzer`, `_processor`, dll | taxonomy, contract(protocol) | infrastructure, surfaces, agent, capabilities(sibling) |
| **Infrastructure** | `infrastructure/` | `_adapter`, `_provider`, `_scanner`, dll | taxonomy, contract(port) | surfaces, capabilities, agent, infrastructure(sibling) |
| **Contract** | `contract/` | `_port`, `_protocol`, `_aggregate` | taxonomy, contract(sibling) | agent, infrastructure, surfaces, capabilities |
| **Taxonomy** | `taxonomy/` | `_vo`, `_entity`, `_event`, `_error`, `_constant` | taxonomy only | semua layer lain |

## 5. File-file Kunci

### Taxonomy (4 files)
| File | Isi |
|------|-----|
| `taxonomy/layer_names_constant.rs` | `LAYER_TAXONOMY = "taxonomy"`, `LAYER_CONTRACT = "contract"`, dll |
| `taxonomy/layer_names_vo.rs` | Fungsi factory: `all_core_layers()` → vec semua layer |
| `taxonomy/layer_definition_vo.rs` | `LayerDefinition { path, allowed_suffixes, allowed_imports, forbidden_imports, mandatory_imports, min_lines, max_lines }` |
| `taxonomy/layer_content_vo.rs` | `LayerNameVO`, `FileContentVO`, `LineContentVO` — data container untuk hasil parsing |

### Contract (5 traits)
| File | Trait | Method |
|------|-------|--------|
| `contract/service_container_aggregate.rs` | `ServiceContainerAggregate` | `get_compliance_analyzer()`, `get_import_checker()`, `get_lint_orchestrator()`, dll |
| `contract/infrastructure_container_aggregate.rs` | `InfrastructureContainerAggregate` | Init semua adapter |
| `contract/capability_container_aggregate.rs` | `CapabilityContainerAggregate` | Init semua checker |
| `contract/orchestrator_container_aggregate.rs` | `OrchestratorContainerAggregate` | Init orchestrator |
| `contract/adapter_container_aggregate.rs` | `AdapterContainerAggregate` | Init linter adapters |

### Capabilities (10 checker files)
| File | Fungsi |
|------|--------|
| `architecture_compliance_analyzer.rs` | `detect_layer(path) → LayerNameVO` — inti layer detection |
| `architecture_naming_checker.rs` | `check_naming(file) → Vec<Violation>` — AES003, AES011 |
| `architecture_import_checker.rs` | `check_imports(file, layer) → Vec<Violation>` — AES001, AES002, AES023 |
| `architecture_role_checker.rs` | `check_role(file, layer) → Vec<Violation>` — AES021 |
| `architecture_metric_checker.rs` | `check_metrics(file) → Vec<Violation>` — AES004, AES005, AES006 |
| `architecture_internal_checker.rs` | `check_internal(file) → Vec<Violation>` — AES012, AES013 |
| `architecture_inheritance_checker.rs` | `check_inheritance(file) → Vec<Violation>` — AES026, AES027 |
| `architecture_orphan_analyzer.rs` | `find_orphans(project) → Vec<Violation>` — AES017 |
| `architecture_cycle_analyzer.rs` | `find_cycles(project) → Vec<Violation>` — AES020 |
| `surface_hierarchy_checker.rs` | `check_surfaces(file) → Vec<Violation>` — AES018, AES019, AES022 |

## 6. Alur Validasi Import (Detail)

```
Sebuah file surfaces/cli_check_command.rs di-check:

1. detect_layer("surfaces/cli_check_command.rs")
   → LAYER_SURFACES, sub-layer "command" (Smart surface)

2. extract_imports() → dapat daftar import:
   ├── "use crate::contract::service_container_aggregate::ServiceContainerAggregate"
   ├── "use crate::infrastructure::rust_linter_adapter::RustLinterAdapter"
   └── "use crate::taxonomy::FilePath"

3. check_forbidden_imports():
   ├── contract → surfaces BOLEH import contract(aggregate) ✅
   ├── infrastructure → surfaces TIDAK BOLEH import infrastructure ❌
   │     └── VIOLATION: AES001 — Layer 'surface' cannot import from 'infrastructure'
   └── taxonomy → surfaces BOLEH import taxonomy ✅

4. check_mandatory_imports():
   └── Surfaces WAJIB import: taxonomy, contract(aggregate)
         └── taxonomy ✅, contract ✅ → OK

5. AES023 check:
   └── Apakah ada import langsung infrastructure/capabilities tanpa lewat container?
         └── Ada: "use crate::infrastructure::..." → VIOLATION (AES023)
```

## 7. Barrel File Mechanism

```
Setiap layer punya mod.rs yang bertindak sebagai "pintu gerbang":

taxonomy/mod.rs:
  pub mod layer_names_constant;
  pub mod layer_names_vo;
  pub mod layer_definition_vo;
  pub mod layer_content_vo;
  // ... semua file di taxonomy/

AES012: Baris di atas WAJIB ada semua. Kalau ada file taxonomy baru
  yang tidak ditambahkan ke mod.rs → VIOLATION "barrel incompleteness"

AES013: File selain mod.rs TIDAK BOLEH punya `pub mod` atau `pub use`
  yang mengekspos modul lain. Hanya mod.rs yang boleh jadi pintu keluar.
```

## 8. Layer-Gated Compilation — Cara Kerja

```bash
# Cuma compile taxonomy (layer paling bawah)
cargo check --lib --no-default-features --features check_taxonomy

# Compile taxonomy + contract
cargo check --lib --no-default-features --features check_contract

# Compile semua layer (default)
cargo check --lib
```

Diagram dependensi feature di `Cargo.toml`:

```toml
[features]
check_taxonomy = []
check_contract = ["check_taxonomy"]
check_infrastructure = ["check_contract"]
check_capabilities = ["check_contract"]
check_agent = ["check_infrastructure", "check_capabilities"]
check_surfaces = ["check_agent"]
default = ["check_surfaces"]
```

Kalau ada `capabilities` yang nyoba import `surfaces`:
- Compile `--features check_capabilities` → OK (surfaces tidak di-include)
- Tapi pas runtime `check surfaces` → ERROR karena `check_capabilities` tidak include surfaces
- **Compile-time protection**: surfaces tidak tersedia di scope capabilities

## 9. Files Summary

| Kategori | Jumlah | File |
|----------|--------|------|
| Layer VOs/Constants | 4 | `layer_names_constant.rs`, `layer_names_vo.rs`, `layer_definition_vo.rs`, `layer_content_vo.rs` |
| Barrel files | 6 | `taxonomy/mod.rs`, `contract/mod.rs`, `capabilities/mod.rs`, `infrastructure/mod.rs`, `agent/mod.rs`, `surfaces/mod.rs` |
| DI Contract traits | 5 | `service_container_aggregate.rs`, `infrastructure_container_aggregate.rs`, `capability_container_aggregate.rs`, `orchestrator_container_aggregate.rs`, `adapter_container_aggregate.rs` |
| Layer enforcement | 10 | Semua file di `capabilities/` untuk layer logic |
| YAML configs | 3 | `lint_arwaky.config.rust.yaml`, `.python.yaml`, `.javascript.yaml` |
| Entry points | 3 | `lib.rs`, `cli_main_entry.rs`, `mcp_main_entry.rs` |

## 10. AES Compliance

| Rule | Status | Mekanisme |
|------|--------|-----------|
| AES001 | ✅ | `check_forbidden_imports()` — cocokkan target import dengan daftar terlarang per layer |
| AES002 | ✅ | `check_mandatory_imports()` — pastikan import wajib ada |
| AES003 | ✅ | `check_naming()` — regex `^\w+_\w+_\w+\.rs$` |
| AES006 | ✅ | `find_primitive_violations()` — scan type annotation: `String`, `i32`, `int`, `str` |
| AES008 | ✅ | `check_suffix()` — contract harus `_port`, `_protocol`, atau `_aggregate` |
| AES011 | ✅ | `check_suffix()` — suffix harus sesuai allowed list per layer |
| AES012 | ✅ | `check_barrel()` — mod.rs harus re-export semua file di layer |
| AES013 | ✅ | `check_internal()` — file non-mod.rs tidak boleh `pub mod` / `pub use` |
| AES018 | ✅ | Utility surfaces tidak boleh import Smart surfaces |
| AES019 | ✅ | Passive surfaces cuma boleh import taxonomy |
| AES022 | ✅ | Smart surfaces harus delegate via `ServiceContainerAggregate` |
| AES023 | ✅ | Surface tidak boleh langsung import infra/capabilities |
| AES026 | ✅ | Contract Aggregate gak boleh `impl PortTrait` — harus composition |
| AES027 | ✅ | Setiap logic file harus implements minimal satu contract trait |
| AES033 | ✅ | `_constant` file cuma berisi `pub const` / `pub static` |

## 11. Implementation Order

1. **Taxonomy**: Definisikan layer names, definitions, content VOs
2. **Contract**: Definisikan `ServiceContainerAggregate` + DI traits
3. **Infrastructure**: Implementasi `OSFileSystemAdapter`, config providers, parsers
4. **Capabilities**: Implementasi compliance analyzer, naming checker, import checker
5. **Agent**: Wiring DI container
6. **Surfaces**: Wiring CLI check command

## 12. Acceptance Criteria

| # | Kriteria | Mekanisme Verifikasi | Status |
|---|----------|---------------------|--------|
| AC001 | Setiap file punya suffix layer sesuai | `check_suffix()` di semua file `src-rust/` | ✅ |
| AC002 | Taxonomy tidak import layer atas | `check_forbidden_imports()` di taxonomy | ✅ |
| AC003 | Surface tidak import infra/cap langsung | `check_legacy_import_rules()` → AES023 | ✅ |
| AC004 | Barrel lengkap (AES012) | `check_barrel()` di semua mod.rs | ✅ |
| AC005 | `features check_taxonomy` hanya compile taxonomy | `cargo check --no-default-features --features check_taxonomy` | ✅ |
| AC006 | `features check_surfaces` compile semua | `cargo check --features check_surfaces` | ✅ |
| AC007 | Self-lint deteksi pelanggaran | `lint-arwaky-cli check .` → 153 violations | ✅ |
| AC008 | Circular dependency terdeteksi | `architecture_cycle_analyzer` | ✅ |
| AC009 | Setiap logic file implements contract trait | `check_inheritance()` → AES027 | ✅ |
| AC010 | Konfigurasi lewat YAML tanpa perubahan kode | Baca `lint_arwaky.config.rust.yaml` | ✅ |
