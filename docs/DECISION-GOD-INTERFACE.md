# Decision: ServiceContainerAggregate God Interface Refactoring

> **Status:** PENDING HUMAN DECISION
> **Issue:** #12 — ServiceContainerAggregate God Interface
> **Severity:** HIGH | **Effort:** 3-4 jam

---

## Problem

`ServiceContainerAggregate` adalah "god interface" — satu trait yang punya **31 method**, semua returning `Option<Arc<dyn T>>`. Trait ini di-implementasi oleh satu class (`DependencyInjectionContainer`) yang punya 30+ field.

**Dampak:**
- Setiap method harus di-implementasi meskipun tidak semua dipanggil
- AES030 orphan detection membutuhkan semua 31 method terdaftar
- Menambah method baru = mengubah satu file besar
- Tidak ada Interface Segregation — caller mendapat akses ke semua method

---

## Current State

```rust
// contract_service_aggregate.rs — 31 methods
pub trait ServiceContainerAggregate: Send + Sync {
    fn file_system(&self) -> Option<Arc<dyn IFileSystemPort>>;
    fn command_executor(&self) -> Option<Arc<dyn ICommandExecutorPort>>;
    fn path_normalization(&self) -> Option<Arc<dyn IPathNormalizationPort>>;
    fn source_parser(&self) -> Option<Arc<dyn ISourceParserPort>>;
    fn linter_adapter(&self, name: &AdapterName) -> Option<Arc<dyn ILinterAdapterPort>>;
    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>>;
    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>>;
    fn metrics_provider(&self) -> Option<Arc<dyn IMetricsProviderPort>>;
    fn get_fix_orchestrator(&self, dry_run: bool) -> Option<Arc<dyn LintFixOrchestratorAggregate>>;
    fn get_report_formatter(&self) -> Option<Box<dyn IReportFormatterProtocol>>;
    // ... 21 more methods
}
```

**Callers:**
- Surface commands: `container.get_architecture_linter()`, `container.linter_adapter()`
- MCP server: `container.get_fix_orchestrator()`
- Tests: Mock implementations

---

## Option A: Split into Focused Sub-Traits

### Proposal

Pecah menjadi 5 sub-trait yang lebih kecil, lalu compose:

```rust
// === Sub-container traits ===

pub trait LintingContainer: Send + Sync {
    fn linter_adapter(&self, name: &AdapterName) -> Option<Arc<dyn ILinterAdapterPort>>;
    fn get_architecture_linter(&self) -> Option<Arc<dyn IArchLintProtocol>>;
    fn get_report_formatter(&self) -> Option<Box<dyn IReportFormatterProtocol>>;
    fn get_fix_orchestrator(&self, dry_run: bool) -> Option<Arc<dyn LintFixOrchestratorAggregate>>;
}

pub trait ConfigContainer: Send + Sync {
    fn get_config_discovery(&self) -> Option<Arc<dyn IConfigDiscoveryPort>>;
    fn get_config_orchestration(&self) -> Option<Arc<dyn IConfigOrchestrationAggregate>>;
    fn get_config_parser(&self) -> Option<Arc<dyn IConfigParserPort>>;
    fn get_config_validator(&self) -> Option<Arc<dyn IConfigValidatorProtocol>>;
}

pub trait GitContainer: Send + Sync {
    fn get_git_commands(&self) -> Option<Arc<dyn GitCommandsAggregate>>;
    fn get_git_orchestrator(&self) -> Option<Arc<dyn HookManagementOrchestratorAggregate>>;
    fn get_import_parser(&self) -> Option<Arc<dyn IImportParserPort>>;
}

pub trait PipelineContainer: Send + Sync {
    fn get_job_registry(&self) -> Option<Arc<dyn IJobRegistryPort>>;
    fn get_pipeline_extended(&self) -> Option<Arc<dyn PipelineExtendedOrchestratorAggregate>>;
    fn get_pipeline_output(&self) -> Option<Arc<dyn PipelineOutputAggregate>>;
}

pub trait AnalysisContainer: Send + Sync {
    fn get_analysis_protocol(&self) -> Option<Arc<dyn IAnalysisProtocol>>;
    fn get_code_metric_protocol(&self) -> Option<Arc<dyn ICodeMetricAnalyzerProtocol>>;
    fn get_target_resolver(&self) -> Option<Arc<dyn ITargetResolverProtocol>>;
    fn get_unused_protocol(&self) -> Option<Arc<dyn IUnusedProtocol>>;
}

// === Top-level aggregate ===

pub trait ServiceContainerAggregate:
    LintingContainer + ConfigContainer + GitContainer + PipelineContainer + AnalysisContainer
{
    fn file_system(&self) -> Option<Arc<dyn IFileSystemPort>>;
    fn command_executor(&self) -> Option<Arc<dyn ICommandExecutorPort>>;
    fn path_normalization(&self) -> Option<Arc<dyn IPathNormalizationPort>>;
    fn source_parser(&self) -> Option<Arc<dyn ISourceParserPort>>;
    fn metrics_provider(&self) -> Option<Arc<dyn IMetricsProviderPort>>;
    fn get_scanner_provider(&self) -> Option<Arc<dyn IScannerProviderPort>>;
}
```

### Pros
- ✅ Interface Segregation Principle (ISP)
- ✅ Caller hanya lihat method yang relevan
- ✅ Menambah method = tambah ke sub-trait yang tepat
- ✅ Lebih mudah di-mock untuk testing
- ✅ Tidak ada `todo!()` atau `panic!()` — setiap method di-implement

### Cons
- ❌ Perlu update semua surface commands (caller)
- ❌ Perlu update semua mock implementations
- ❌ Lebih banyak trait definitions
- ❌ AES030 orphan detection perlu check lebih banyak trait

### Impact Assessment

**Files yang perlu diubah:**
- `contract_service_aggregate.rs` — split trait
- `agent_injection_container.rs` — implement sub-trait
- `surface_check_command.rs` — update caller
- `surface_fix_command.rs` — update caller
- `surface_watch_command.rs` — update caller
- `surface_tools_command.rs` — update caller
- Semua mock/test implementations

**Estimated effort:** 3-4 jam

---

## Option B: Keep God Interface, Add Deprecation Warnings

### Proposal

Tidak split interface, tapi tambah warning untuk method yang jarang dipakai:

```rust
pub trait ServiceContainerAggregate: Send + Sync {
    #[deprecated(note = "Use get_architecture_linter() instead")]
    fn file_system(&self) -> Option<Arc<dyn IFileSystemPort>> { None }
    // ... keep all 31 methods
}
```

### Pros
- ✅ Zero breaking changes
- ✅ Gradual migration path
- ✅ Tidak perlu update caller

### Cons
- ❌ Tidak menyelesaikan masalah fundamental
- ❌ Deprecation warnings bisa mengganggu
- ❌ God interface tetap ada

---

## Option C: Keep Current Implementation (No Change)

### Proposal

Biarkan apa adanya. 31 methods dengan `Option` defaults sudah cukup.

### Pros
- ✅ Zero effort
- ✅ Sudah compile dan test pass
- ✅ AES030 orphan detection tetap jalan

### Cons
- ❌ God interface tetap ada
- ❌ ISP tidak terpenuhi
- ❌ Developer experience kurang optimal

---

## Recommendation

**Option A**是最 clean solution, tapi perlu effort besar. **Option C**是最pragmatic untuk sekarang.

**Suggested approach:**
1. Sekarang: **Option C** (keep current, sudah fix semua `panic!()` → `Option`)
2. Nanti (ketika ada waktu): Migrasi ke **Option A** secara bertahap

---

## Decision Required

Pilih salah satu:
- [ ] **Option A** — Split ke sub-traits (3-4 jam effort)
- [ ] **Option B** — Keep + deprecation warnings
- [ ] **Option C** — Keep current (no change needed now)
