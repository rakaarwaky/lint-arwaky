# PRD — Contract Layer
> **Vision**: The formal promise layer — defines WHAT without defining HOW

## Layer Identity

**Layer**: Contract (Abstraction Layer)
**Path**: `src-rust/contract/`
**Role**: Interface definitions — Ports (outbound), Protocols (inbound), Aggregates (composition)
**Dependency Rule**: Can import from `taxonomy` and `contract` only. ZERO imports to `capabilities`, `infrastructure`, `agent`, or `surfaces`.

## 1. Strategic Goal

Contract must be the **single source of interface truth**. Every technical concern has a `_port` trait. Every business capability has a `_protocol` trait. Every logical orchestration group has a single `_aggregate` trait. Trait signatures use ONLY taxonomy VOs. The barrel re-exports every type with `*Ref` type aliases.

## 2. Component Blueprint

### 2.1 Ports — Outbound Interfaces (for Infrastructure)

Every technical operation must be behind a port trait. Ports define the boundary between domain logic and the outside world.

| Port | Required Methods | Implemented By |
|------|-----------------|----------------|
| `IFileSystemPort` | `read_file()`, `write_file()`, `file_exists()`, `read_dir()`, `create_dir()`, `remove_file()`, `metadata()` | OSFileSystemAdapter |
| `ISourceParserPort` | `parse_rust()`, `parse_python()`, `parse_javascript()`, `parse_file()` | SourceParserOrchestrator |
| `ICommandExecutorPort` | `execute(command, args)`, `execute_with_timeout(command, timeout)` | StdioClient |
| `IJobRegistryPort` | `create_job(JobId, Action)`, `get_job(JobId)`, `cancel_job(JobId)`, `list_jobs()` | MemoryJobRegistryAdapter |
| `ILinterAdapterPort` | `lint_file(FilePath)`, `lint_project(DirectoryPath)`, `get_version()` | All linter adapters |
| `IWatchProviderPort` | `watch(DirectoryPath)`, `unwatch(DirectoryPath)`, `poll()` | WatchServiceProvider |
| `IPathNormalizationPort` | `normalize(path)`, `resolve(base, relative)`, `relative_to(path, base)` | PathNormalizationProvider |
| `IHttpProviderPort` | `get(url)`, `post(url, body)`, `request(method, url, headers)` | SyncHttpProvider |
| `IHookManagerPort` | `install_hook(hook_type, script)`, `uninstall_hook(hook_type)`, `is_installed(hook_type)` | GitHookAdapter |
| `IPluginManagerPort` | `discover_plugins()`, `load_plugin(name)`, `list_plugins()` | PluginSystemProvider |
| `IMcpServerPort` | `handle_request(request)`, `send_response(response)`, `is_running()` | McpServerWrapper |
| `IConfigDiscoveryPort` | `discover_config(start_dir)`, `find_upwards(filename)` | ConfigDiscoveryProvider |
| `IConfigParserPort` | `parse_yaml(content)`, `parse_toml(content)`, `parse_json(content)` | ConfigParserProvider |
| `IConfigProviderPort` | `get_config()`, `set_config(key, value)`, `validate_config()` | ConfigParserProvider |
| `IConfigValidationPort` | `validate(config)`, `report_issues()` | ConfigRulesValidator |
| `IScannerProviderPort` | `scan_file(FilePath)`, `scan_project(DirectoryPath)` | PythonSymbolScanner |
| `ISemanticTracerPort` | `resolve_symbol(name, scope)`, `trace_type(expression)` | PythonTracer / JSScopeTracer |
| `IJavascriptScopePort` | `get_scope(FilePath)`, `resolve_variable(name)`, `find_references(symbol)` | JSScopeProvider |
| `IJavascriptFlowPort` | `trace_call(fn_name)`, `get_callees(fn_name)` | JSFlowAdapter |
| `INamingProviderPort` | `get_naming_variants(identifier)`, `validate_naming(name, convention)` | JavascriptNamingProvider |
| `INamingVariantPort` | `get_variants(identifier)`, `suggest_alternatives(name)` | PythonNamingVariantProvider |
| `IMetricsProviderPort` | `get_complexity(FilePath)`, `get_duplicates(DirectoryPath)`, `get_trends(DirectoryPath)` | MetricsProvider |

### 2.2 Protocols — Inbound Interfaces (for Capabilities)

Every business capability must be behind a protocol trait.

| Protocol | Required Methods | Implemented By |
|----------|-----------------|----------------|
| `IArchLintProtocol` | `run_self_lint(project_root)`, `run_self_lint_dir(src_dir)`, `format_report(results, root)` | ArchLintHandler |
| `IArchComplianceProtocol` | `check_layer_compliance(files)`, `check_import_rules(files, config)` | ArchComplianceAnalyzer |
| `IArchRuleProtocol` | Sub-traits: `INamingChecker`, `IArchImportProcessor`, `IArchStructure`, `ICodeQuality`, `IMetricChecker`, `IInternalChecker`, `IRoleChecker` | Individual checkers |
| `IArchOrphanProtocol` | Sub-traits: `IOrphanGraphProtocol`, `IOrphanIndicatorProtocol` | ArchOrphanAnalyzer |
| `IDomainTypeProtocol` | `check_primitives(files)`, `validate_vo_usage(files)` | DomainTypeRuleChecker |
| `IDispatchRoutingProtocol` | `dispatch(action, args)`, `route(action)`, `validate(routing)` | DispatchRoutingChecker |
| `IDispatchRoutingParserProtocol` | `parse_method_args(method_ref)`, `validate_method_ref(ref)` | DispatchRoutingParser |
| `ISemanticTracerProtocol` | `trace_callchain(start_symbol)`, `resolve_scope(symbol)` | CallChainAnalyzer |
| `ISemanticFlowProtocol` / `IDataFlowProtocol` | `analyze_data_flow(function)`, `track_variable(name, scope)` | DataFlowAnalyzer |
| `ILintReportingProtocol` | `format_json(results)`, `format_sarif(results)`, `format_junit(results)` | ReportFormatterProcessor |
| `INamingVariantProtocol` | `generate_variants(name)`, `detect_naming_style(name)` | NamingVariantAnalyzer |
| `ICodeTransformationProtocol` | `rename_symbol(old, new)`, `apply_fix(fix)` | SymbolRenamerProcessor |
| `IProjectGovernanceProtocol` | Sub-traits: `IArchRuleEngine`, `IConfigRules`, `IMetricAnalyzer` | — |
| `ISetupManagementProtocol` | `generate_env()`, `generate_mcp_config(client)` | SetupManagementProcessor |
| `IScopeBoundaryProtocol` | `resolve_boundary(scope)`, `check_cross_boundary(source, target)` | ScopeBoundaryResolver |

### 2.3 Aggregates — Logical Composition Facades

Aggregates group related ports and protocols into logical clusters. Target: 10-12 aggregates maximum. Each aggregate represents a MAJOR subsystem, not every single concern.

| Aggregate | Composes | Purpose |
|-----------|----------|---------|
| **`ServiceContainerAggregate`** | All essential ports + arch linter + job registry | **THE** DI container interface — one aggregate to rule them all |
| **`PipelineCoreAggregate`** | Input, output, execution, extended, action dispatcher | Full pipeline lifecycle (scan → parse → analyze → format) |
| **`LintPipelineOrchestratorAggregate`** | Pipeline stages + report formatting | Complete lint pipeline entry point |
| **`CommandCoreAggregate`** | Check, fix, dev, report, git, plugin, watch, maintenance | All CLI/MCP command orchestration |
| **`InfrastructureCoreAggregate`** | File system, config discovery, config parser, provider, scanner, path normalization | Infrastructure provider wiring |
| **`CapabilityCoreAggregate`** | Architecture compliance, orphan, rule engine, naming, semantic, dispatch | Capability handler wiring |
| **`OrchestratorCoreAggregate`** | Architecture orchestration, compliance coordination, analysis | High-level orchestration wiring |
| **`AdapterAggregate`** | Linter adapters, MCP server, plugin manager, hook manager | Adapter wiring |
| **`MonitoringCoreAggregate`** | Watch provider, job registry, directory watch, lifecycle, output client | Background operations and state management |
| **`ProjectCoreAggregate`** | Container registry, project container, multi-project orchestrator | Multi-project and project-scoped DI |
| **`SetupCoreAggregate`** | Setup management, hook management, mcp config | Setup and configuration |
| **`ContainerRegistryAggregate`** | Per-project DI container registry | Multi-project container management |

> **Design rule**: Aggregates are composition facades, NOT trait wrappers for every struct in Agent. If an aggregate has only 1 method that just delegates, it should be merged into a parent aggregate. The goal is 10-15 aggregates, not 30+.

### 2.4 Barrel (`mod.rs`) Requirements

The Contract barrel MUST:
- Declare every module with `pub mod`
- Re-export every public type via `pub use`
- Provide `Arc<dyn ...>` type aliases: `FileSystemPortRef`, `SourceParserPortRef`, `ArchLintProtocolRef`, `ServiceContainerRef`, etc.
- Provide utility functions: `find_command(name)`, `is_command_registered(name)`, `command_count()`, `list_command_names()`
- Include barrel tests verifying command catalog completeness

> **Note on utility functions**: The barrel contains `find_command()` and related as LIMITED helpers that only query the static `COMMAND_CATALOG` constant (defined in Taxonomy). These are query operations on a constant, NOT business logic. Any mutation or routing logic must live in Capabilities as `ICommandRegistryPort`.

## 3. Architectural Rules

| Rule | Constraint |
|------|------------|
| AES001 | Zero imports to capabilities, infrastructure, agent, or surfaces |
| AES006 | Trait signatures use taxonomy VOs exclusively |
| AES007 | All contract imports via barrel (`contract::*`) |
| AES008 | Suffixes: `_port`, `_protocol`, `_aggregate` only |
| AES026 | Aggregates may compose Ports + Protocols; zero business logic |
| AES027 | Every importing layer must implement at least one contract |

## 4. Non-Functional Targets

| Metric | Target |
|--------|--------|
| Port traits | Every technical concern covered |
| Protocol traits | Every business capability covered |
| Aggregate traits | 10-12 max — logical groups only |
| VO usage in signatures | 100% — zero primitives |
| Barrel re-exports | 100% of public types |
| Type aliases | One `*Ref` per port and protocol |

## 5. Success Criteria

A Contract layer is **complete** when:
- Every infrastructure adapter has a corresponding `_port` trait
- Every capability checker/analyzer has a corresponding `_protocol` trait
- Agent wiring is grouped into ~12 logical `_aggregate` traits (not 30+)
- Zero raw primitives (`String`, `i64`, `bool`, etc.) appear in trait method signatures
- All trait methods return/accept taxonomy VOs
- Barrel re-exports every type with documentation
- Arc-wrapped type aliases exist for DI wiring
- Command catalog utility functions correctly query COMMAND_CATALOG
