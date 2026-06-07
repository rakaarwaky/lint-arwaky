# PRD — Agent Layer
> **Vision**: The conductor — DI wiring, orchestration, lifecycle, and governance

## Layer Identity

**Layer**: Agent (Orchestration & Governance)
**Path**: `src-rust/agent/`
**Role**: Dependency injection, pipeline orchestration, lifecycle management, job registry, command dispatch
**Dependency Rule**: Component-type dependent:
- **Containers/Registries/Mixins** (wiring): import from `taxonomy`, `contract`, `capabilities`, `infrastructure`, and sibling agent orchestrators
- **Orchestrators/Coordinators** (execution): import ONLY from `taxonomy` and `contract` — access capabilities/infrastructure through `ServiceContainerAggregate` (contract)
- **Managers** (support): import from `taxonomy` and `contract(aggregate)` only
ZERO imports to `surfaces` for all types.

## 1. Strategic Goal

Agent must become the **composition root and orchestrator** of the entire system. It creates all adapters, wires them into the DI container, orchestrates pipeline execution, manages job lifecycles, coordinates multi-project runs, and supervises watchers. Agent is the ONLY layer that connects capabilities to infrastructure.

## 2. Two Categories of Components

Agent has TWO distinct component categories with different state rules:

### 2.1 Stateless Orchestrators (one-shot operations)

These components execute a single operation and return. They MUST have ZERO mutable fields. All state is passed as function parameters.

| Orchestrator | Flow | State Rule |
|-------------|------|------------|
| `ArchitectureLintOrchestrator` | Collect files → load config → create analyzer → run checks → return results | STATELESS |
| `ArchLintPipelineOrchestrator` | Create orchestrator → execute → format report | STATELESS |
| `PipelineExecutionOrchestrator` | Stage 1→2→3→4→5 sequentially | STATELESS |
| `PipelineActionOrchestrator` | Receive action → route to capability → collect results | STATELESS |
| `PipelineExtendedOrchestrator` | Filtering, batching, progress | STATELESS |
| `AnalysisOrchestrator` | Execute scan commands | STATELESS |
| `LintFixOrchestrator` | Check → apply fix → verify | STATELESS |

```rust
// Stateless orchestrator pattern — zero fields, all data passed as params
pub struct AnalysisOrchestrator;
impl AnalysisOrchestrator {
    pub fn execute(&self, files: &[FilePath], config: &ArchitectureConfig) -> Vec<LintResult> { ... }
}
```

### 2.2 Stateful Services (continuous background operations)

These components manage persistent state across multiple calls. State is explicitly managed in thread-safe containers.

| Service | State Managed | Thread Safety |
|---------|-------------|---------------|
| `WatchExecutionOrchestrator` | Poll timer, last-scan results, file change set | Per-watch-instance |
| `WatchCommandsOrchestrator` | Active watch sessions | Arc<Mutex<HashMap>> |
| `LifecycleStateManager` | Session lifecycle, background task handles | Arc<Mutex<>> |
| `PipelineJobRegistry` | Active jobs map (JobId → JobStatus) | Arc<Mutex<HashMap>> |
| `ProjectContainerRegistry` | Per-project DI containers | Arc<RwLock<HashMap>> |
| `OutputClientOrchestrator` | Output channel configuration | Arc<RwLock<>> |
| `HookManagementOrchestrator` | Hook installation state | Read-only after init |

```rust
// Stateful service pattern — explicit state in thread-safe container
pub struct WatchExecutionOrchestrator {
    poll_interval: Duration,
    last_results: Arc<Mutex<HashMap<FilePath, Vec<LintResult>>>>,
}
impl WatchExecutionOrchestrator {
    pub fn poll(&self) -> Vec<LintResult> {
        let mut cache = self.last_results.lock().unwrap();
        // ... update cache with new results
    }
}
```

> **Design rule**: Stateful services must document WHAT state they hold and WHY it can't be eliminated. State must be in explicit `Arc<Mutex<>>` or `Arc<RwLock<>>` containers — never in raw mutable fields.

## 3. DI Container (Composition Root)

| Module | Responsibility |
|--------|---------------|
| `DependencyInjectionContainer` | Creates ALL adapters, wires ALL ports, exposes ALL protocols |

**Container requirements:**
- Creates: `OSFileSystemAdapter`, `SourceParserOrchestrator` (with 3 language parsers), `StdioClient`, `MemoryJobRegistryAdapter`, ALL 12 linter adapters, `ArchLintHandler`
- Stores: `Arc<dyn IFileSystemPort>`, `Arc<dyn ICommandExecutorPort>`, `Arc<dyn IPathNormalizationPort>`, `Arc<dyn ISourceParserPort>`, `Arc<dyn IArchLintProtocol>`, `HashMap<String, Arc<dyn ILinterAdapterPort>>`
- Implements: `ServiceContainerAggregate` (all getter methods)
- Pattern: Builder/Constructor — immutable after construction

```rust
pub struct DependencyInjectionContainer {
    file_system: Arc<dyn IFileSystemPort>,
    command_executor: Arc<dyn ICommandExecutorPort>,
    path_normalization: Arc<dyn IPathNormalizationPort>,
    source_parser: Arc<dyn ISourceParserPort>,
    architecture_linter: Arc<dyn IArchLintProtocol>,
    linter_adapters: HashMap<String, Arc<dyn ILinterAdapterPort>>,
}
```

## 4. Mixin Containers (DI Wiring Helpers)

| Mixin | Wires |
|-------|-------|
| `InfrastructureMixinContainer` | File system, source parser, command executor, path normalization |
| `CapabilityMixinContainer` | Architecture linter, compliance analyzer, orphan analyzer |
| `OrchestratorMixinContainer` | Pipeline orchestrators, watch orchestrators, coordinators |
| `AdapterMixinContainer` | All linter adapters |

## 5. Coordinators (Policy-Level — stateless)

| Coordinator | Policy |
|-------------|--------|
| `ArchitectureOrchestrator` | High-level architecture compliance: configure → lint → report → score |
| `ArchComplianceCoordinator` | Multi-stage compliance: naming → imports → metrics → orphans → cycles |

## 6. Managers (Lifecycle — delegates to registries)

| Manager | Manages | State? |
|---------|---------|--------|
| `SetupManagementOrchestrator` | `init`, `doctor`, `mcp-config`, `hermes` setup flows | Stateless |
| `MaintenanceCommandsOrchestrator` | System health, diagnostics, cleanup | Stateless |

## 7. Pipeline Flow (Ideal)

```
User Input
  │
  ▼
DI Container ──► Create ──► OSFileSystemAdapter
                    │           SourceParserOrchestrator
                    │           StdioClient
                    │           MemoryJobRegistryAdapter
                    │           All linter adapters
                    │           ArchLintHandler
                    │
                    ▼
ArchitectureLintOrchestrator (stateless)
  │
  ├── Stage 1: Scan ──► collect_rs_files() + load_config()
  ├── Stage 2: Parse ──► SourceParserOrchestrator → SourceContentVO[]
  ├── Stage 3: Analyze ──► ArchComplianceAnalyzer.execute(SourceContentVO[])
  ├── Stage 4: Project-Wide ──► cycles → orphans → inheritance → roles
  └── Stage 5: Output ──► format_report() / print_json() / print_sarif() / print_junit()
```

> **Key difference from earlier design**: Stage 2 (Parse) is EXPLICIT. Agent calls `SourceParserOrchestrator` to produce `Vec<SourceContentVO>`, then passes the parsed data to Capabilities analyzers. This keeps Capabilities pure — they never touch infrastructure ports directly.

## 8. Architectural Rules

| Rule | Constraint |
|------|------------|
| AES001 | Zero imports to surfaces |
| AES003 | Filenames: word1_word2_word3 pattern |
| AES021 | Orchestrators MUST be stateless; services must document state explicitly |
| AES024 | Zero `Any` type annotations — use typed containers |
| AES027 | Every file must implement at least one imported contract type |

## 9. Non-Functional Targets

| Metric | Target |
|--------|--------|
| DI container creation time | < 100ms |
| Stateless orchestrators | 100% — zero mutable fields |
| Stateful services | Explicit state management with Arc<Mutex> |
| Pipeline stage chain | 5 stages in correct order |
| Job registry concurrency | Thread-safe (Arc<Mutex>) |
| Business logic in Agent | ZERO — delegate to Capabilities |
| Surface imports | ZERO |

## 10. Success Criteria

An Agent layer is **complete** when:
- `DependencyInjectionContainer` creates AND wires ALL adapters correctly
- `ArchitectureLintOrchestrator` runs end-to-end (files → parsed content → results)
- Pipeline orchestrator chains all 5 stages correctly
- Job registry supports concurrent create/get/cancel/list
- All command orchestrators delegate to proper capabilities
- `LifecycleStateManager` handles service lifecycle correctly
- Watch mode polls and re-lints on interval (stateful, documented)
- Multi-project mode lints multiple paths
- Plugin system discovers and loads plugins
- Zero business logic in agent files
- Zero mutable state in orchestrators (stateful services exempted)
