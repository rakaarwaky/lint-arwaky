# Decision: AES002 Mandatory Import Dead Code Pattern

> **Status:** PENDING HUMAN DECISION
> **Issue:** #14 — `_use_mandatory_imports()` Dead Code Hacks
> **Severity:** LOW | **Effort:** 2-3 jam

---

## Apa Itu AES002?

AES002 adalah rule yang **mensyaratkan setiap file harus import types tertentu dari layer lain**. Tujuannya memastikan dependency antar layer selalu terhubung — tidak ada file yang "terisolasi" dari dependency graph.

### Cara Kerja AES002

AES002 membaca config `lint_arwaky.config.rust.yaml` → section `architecture.layers`. Setiap layer punya field `mandatory` yang mendefinisikan import yang **wajib** ada di file-file layer tersebut.

**Contoh:** Layer `surfaces` (command/controller files) punya config:

```yaml
# RULES_AES.md — Rule #11: surfaces(command|controller)
# Mandatory Imports: taxonomy, contract(aggregate)
```

Artinya: **Setiap file** yang namanya diawali `surface_` atau suffix-nya `_command.rs`/`_controller.rs` **wajib** import minimal:
1. Satu type dari `taxonomy` layer (misal `LineNumber`, `ColumnNumber`)
2. Satu type dari `contract(aggregate)` layer (misal `ReportCommandsAggregate`)

### Masalahnya

Banyak surface files **tidak butuh** imports ini untuk business logic mereka:

```rust
// surface_map_command.rs — hanya punya delegation functions
pub fn handle_cancel(job_id: String) -> ExitCode {
    println!("Cancel requested for job: {}", job_id);
    ExitCode::SUCCESS
}

pub fn handle_import(config_file: String) -> ExitCode {
    // ... baca file, copy config ...
    ExitCode::SUCCESS
}
```

File ini **tidak butuh** `LineNumber` atau `ReportCommandsAggregate`. Tapi AES002 tetap mensyaratkan import-nya. Developer terpaksa bikin hack:

```rust
// Hack untuk satisfy AES002 + AES023
fn _use_mandatory_imports() {
    let _ = LineNumber::new(1);  // import taxonomy — "digunakan" di sini
    let _ = std::marker::PhantomData::<dyn ReportCommandsAggregate>;  // import contract
}
```

Tanpa hack ini:
- AES002 flag: "File missing mandatory import `LineNumber`"
- AES023 flag: "Import `LineNumber` is unused" (jika import tanpa hack)

---

## Inventory: 29 Files dengan Dead Code Hacks

### Pattern 1: `LineNumber::new(1)` — 26 files

```rust
fn _use_mandatory_imports() {
    let _ = LineNumber::new(1);  // hack: "menggunakan" LineNumber
}
```

**Import yang diwajibkan:** `crate::shared_common::taxonomy_common_vo::LineNumber`

**Files:**
| Layer | Files |
|-------|-------|
| `cli-commands/` | `surface_bootstrap_command.rs`, `surface_config_command.rs`, `surface_core_command.rs`, `surface_git_command.rs`, `surface_maintenance_command.rs`, `surface_map_command.rs`, `surface_report_command.rs`, `surface_setup_command.rs`, `surface_tui_command.rs`, `surface_watch_command.rs` |
| `mcp-server/` | `capabilities_schema_checker.rs`, `infrastructure_server_lifespan.rs`, `infrastructure_server_validation_util.rs`, `surface_client_controller.rs`, `surface_execute_command.rs`, `surface_health_controller.rs`, `surface_server_controller.rs`, `surface_tools_command.rs`, `surface_tools_controller.rs` |
| `code-analysis/` | `capabilities_renamer_processor.rs`, `contract_code_metric_analyzer_protocol.rs` |
| `language-adapters/` | `infrastructure_py_ast_util.rs`, `infrastructure_py_primitive_detector.rs` |
| `source-parsing/` | `infrastructure_barrel_provider.rs` |
| `shared-common/` | `taxonomy_common_error.rs` |

### Pattern 2: `PhantomData::<dyn T>` — 15 files

```rust
fn _use_contract_aggregates() {
    let _ = std::marker::PhantomData::<dyn ReportCommandsAggregate>;
}
```

**Import yang diwajibkan:** Contract aggregate types

**Files:**
| Import Type | Files |
|-------------|-------|
| `ReportCommandsAggregate` | 10 surface commands + 6 MCP controllers |
| `DevCommandsAggregate` | 7 MCP controllers + 1 surface |
| `LintFixOrchestratorAggregate` | `surface_fix_command.rs` |
| `PipelineActionDispatcherAggregate` | `surface_watch_command.rs` |
| `PluginCommandsAggregate` | `surface_plugin_command.rs` |
| `OutputClientAggregate` | `surface_report_command.rs` |
| `IMcpServerPort` | 2 infrastructure files |
| `INamingProviderPort` | 2 language adapter files |
| `IScannerProviderPort` | `infrastructure_barrel_provider.rs` |
| `IAnalysisProtocol` | 2 code-analysis files |

---

## Kenapa Ini Terjadi?

### Root Cause: AES002 vs AES023 Conflict

```
AES002: "File surface_* WAJIB import taxonomy + contract(aggregate)"
         ↓
AES023: "Import yang tidak digunakan adalah VIOLASI"
         ↓
Developer: "Tapi file ini tidak butuh import itu!"
         ↓
Hack: _use_mandatory_imports() → "import digunakan" di dummy function
```

**Konflik:** AES002 mensyaratkan import yang tidak diperlukan, AES023 melarang unused import. Satu-satunya jalan keluar adalah hack.

### Kenapa AES002 Begitu Ketat?

AES002 dirancang untuk memastikan **setiap file terhubung ke dependency graph**. Ide-nya:
- Jika file di layer `surfaces`, dia harus punya dependensi ke `taxonomy` dan `contract`
- Ini memastikan tidak ada file yang "mengambang" tanpa koneksi ke domain model

**Tapi implementasinya terlalu broad:** rule ini apply ke **semua** file di layer, tanpa mempertimbangkan apakah file tersebut benar-benar butuh import tersebut.

---

## Solusi: Revisi AES002

### Opsi A: Skip Mandatory Import untuk File Tanpa Business Logic

**Konsep:** AES002 hanya wajibkan import jika file punya **business logic** yang sebenarnya membutuhkan types dari layer lain.

**Concrete Change:**

```rust
// capabilities_import_mandatory_checker.rs — check_scope_mandatory_imports()

// BEFORE (current):
// Selalu check mandatory imports untuk semua file di layer

// AFTER (revisi):
// Skip jika file tidak punya business logic:
if is_delegation_only_file(file) {
    return;  // skip mandatory import check
}
```

**Deteksi "delegation only file":**
```rust
fn is_delegation_only_file(file: &str) -> bool {
    let content = std::fs::read_to_string(file).unwrap_or_default();
    let has_struct = content.contains("pub struct ") || content.contains("pub trait ");
    let has_impl = content.contains("impl ");
    let has_real_function = content.lines().filter(|l| l.trim().starts_with("pub fn ")).count() > 3;

    // File delegation hanya punya imports + 1-3 delegation functions
    !has_struct && !has_impl && !has_real_function
}
```

**Pros:**
- ✅ Tidak perlu delete 29 dead functions (mereka jadi unused tapi tidak di-flag)
- ✅ Minimal change ke rules engine
- ✅ File dengan business logic tetap di-check

**Cons:**
- ❌ Dead functions masih ada (meskipun tidak di-flag)
- ❌ Heuristic bisa salah (file yang seharusnya di-check malah skip)

---

### Opsi B: Hapus Dead Functions + Skip Check untuk Surface/MCP Files

**Konsep:**
1. Hapus semua 29 `_use_mandatory_imports()` functions
2. AES002 skip check untuk files yang suffix-nya `_command.rs`, `_controller.rs`, `_handler.rs`

**Concrete Change:**

```yaml
# lint_arwaky.config.rust.yaml — tambah exceptions ke AES002
AES002:
  enabled: true
  scope: [taxonomy, contract, capabilities, infrastructure, agent, surfaces]
  exceptions:
    - main.rs
    - lib.rs
    - mod.rs
    - cli_main_entry.rs
    - mcp_main_entry.rs
    - tui_main_entry.rs
    - "*_command.rs"      # NEW
    - "*_controller.rs"   # NEW
    - "*_handler.rs"      # NEW
```

**Pros:**
- ✅ Hapus semua dead code
- ✅ Simple config change
- ✅ Surface files memang tidak butuh mandatory imports

**Cons:**
- ❌ Bisa kebablasan (file yang seharusnya di-check malah skip)
- ❌ Exception list bisa meluas

---

### Opsi C: Ubah Mandatory Import Jadi "Recommended" untuk Surface Layer

**Konsep:** Untuk layer `surfaces`, mandatory import diubah menjadi "recommended" (warning, bukan error). Hanya error jika file benar-benar tidak punya koneksi ke domain.

**Concrete Change:**

```yaml
# lint_arwaky.config.rust.yaml
AES001:
  conditions:
    - scope: "surfaces(command|controller|page|entry)"
      allowed: [taxonomy, contract]
      mandatory: [taxonomy, contract(aggregate)]  # SEBELUM: error jika missing
      mandatory_severity: MEDIUM  # NEW: downgrade dari HIGH ke MEDIUM
      forbidden: [agent*, infrastructure*, capabilities*, ...]
```

```rust
// capabilities_import_mandatory_checker.rs
// Saat flag missing import:
let severity = if is_surface_file(file) {
    Severity::MEDIUM  # warning, bukan error
} else {
    Severity::HIGH    # tetap error untuk layer lain
};
```

**Pros:**
- ✅ Tidak perlu delete dead functions (mereka tetap "valid" sebagai optional usage)
- ✅ Surface files dapat warning, bukan error
- ✅ Tidak break CI (MEDIUM != CRITICAL)

**Cons:**
- ❌ Dead functions masih ada
- ❌ Developer bisa ignore warning

---

## Recommendation

**Opsi B** 是最 clean solution:
1. Hapus semua 29 dead functions
2. Tambah exceptions ke config untuk surface/MCP files
3. AES002 tetap jalan untuk layer lain (taxonomy, contract, capabilities)

**Effort:** 1-2 jam

---

## Decision Required

Pilih salah satu:
- [ ] **Opsi A** — Skip check untuk file delegation (heuristic)
- [ ] **Opsi B** — Hapus dead functions + tambah exceptions ke config
- [ ] **Opsi C** — Ubah mandatory jadi "recommended" untuk surface layer
