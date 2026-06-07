# PRD — Infrastructure Layer
> **Vision**: The how layer — technical implementations of every contract port

## Layer Identity

**Layer**: Infrastructure (Technical Adapters)
**Path**: `src-rust/infrastructure/`
**Role**: Concrete implementations — external tool wrappers, filesystem, transport, scanners, MCP server
**Dependency Rule**: Can import from `taxonomy` and `contract` only. Sibling imports forbidden (except for composition).

## 1. Strategic Goal

Infrastructure must become the **complete adapter layer** where every `_port` from Contract has a concrete implementation. External tools (Ruff, MyPy, Bandit, ESLint, Prettier, TSC, Clippy) are wrapped behind `ILinterAdapterPort`. All I/O goes through port interfaces. The MCP server speaks valid JSON-RPC 2.0.

## 2. Component Blueprint

### 2.1 Linting Adapters

Each adapter wraps ONE external tool and implements `ILinterAdapterPort`.

| Adapter | Wraps | Language | Runs |
|---------|-------|----------|------|
| `RuffAdapter` | Ruff (`ruff check`) | Python | File / Project |
| `MyPyAdapter` | MyPy (`mypy`) | Python | File / Project |
| `BanditAdapter` | Bandit (`bandit`) | Python | File / Project |
| `ESLintAdapter` | ESLint (`eslint`) | JavaScript/TypeScript | File / Project |
| `PrettierAdapter` | Prettier (`prettier --check`) | JavaScript/TypeScript | File / Project |
| `TSCAdapter` | TypeScript Compiler (`tsc --noEmit`) | TypeScript | Project |
| `RustLinterAdapter` | Clippy (`cargo clippy`) | Rust | Project |
| `ComplexityAdapter` | Radon-style calculation | Python | File / Project |
| `DuplicateAdapter` | Duplicate detection | Multi-language | Project |
| `TrendsAdapter` | Quality history | Multi-language | Project |
| `DependencyAdapter` | pip-audit | Python | Project |
| `ArchComplianceAdapter` | AES self-lint (internal) | Rust | Project |

**Adapter contract**: Every linter adapter:
```rust
pub struct XxxAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    threshold: Option<Count>,
}

impl ILinterAdapterPort for XxxAdapter {
    fn lint_file(&self, path: FilePath) -> Result<LintResultList, AdapterError>;
    fn lint_project(&self, dir: DirectoryPath) -> Result<LintResultList, AdapterError>;
    fn get_version(&self) -> String;
}
```

### 2.2 Source Parsing

| Adapter | Parses | Technology |
|---------|--------|------------|
| `ASTRustParserAdapter` | Rust AST | syn crate (or manual parsing) |
| `ASTPythonParserAdapter` | Python AST | Python AST parser |
| `ASTJSParserAdapter` | JS/TS AST | JavaScript parser |
| `SourceParserOrchestrator` | All languages (delegates) | Composite pattern |

**SourceParserOrchestrator** must:
- Detect language from file extension
- Delegate to correct language parser
- Return `SourceContentVO` with parsed structure
- Handle parse errors gracefully

### 2.3 Semantic Analysis Adapters

| Adapter | Provides | Port |
|---------|----------|------|
| `JSCallAdapter` | JavaScript call trace info | — (composed) |
| `JSFlowAdapter` | JS data flow analysis | IJavascriptFlowPort |
| `JSScopeTracer` | JS scope tracing | IJsTracerPort |
| `JSScopeProvider` | JS scope provider | IJavascriptScopePort |
| `PythonTracer` | Python symbol resolution | ISemanticTracerPort |
| `PythonASTAdapter` | Python AST manipulation | — (composed) |
| `PythonSymbolCollector` | Python symbol collection | — (composed) |
| `PythonAnalysisAdapter` | Python metrics + analysis | — (composed) |
| `PythonMetricsAdapter` | Python metrics provider | IMetricsProviderPort |
| `PythonNamingVariantProvider` | Python naming variants | INamingVariantPort |

### 2.4 Configuration

| Adapter | Port | Behavior |
|---------|------|----------|
| `ConfigDiscoveryProvider` | IConfigDiscoveryPort | Walk up directories looking for YAML config files |
| `ConfigParserProvider` | IConfigParserPort + IConfigProviderPort | Parse YAML/TOML/JSON into ArchitectureConfig VO |

**Config discovery algorithm:**
```
start_dir.ancestors()
  .flat_map(|d| [d/".lint_arwaky.config.yaml", d/"lint_arwaky.config.rust.yaml"])
  .find(|p| p.exists())
```

### 2.5 Naming Providers

| Provider | Port | Language |
|----------|------|----------|
| `JavascriptNamingProvider` | INamingProviderPort | JS/TS |
| `PythonNamingVariantProvider` | INamingVariantPort | Python |

### 2.6 Core Infrastructure

| Provider | Port | Implementation |
|----------|------|----------------|
| `OSFileSystemAdapter` | IFileSystemPort | std::fs |
| `PathNormalizationProvider` | IPathNormalizationPort | std::path |
| `WatchServiceProvider` | IWatchProviderPort | Poll-based (inotify not required) |
| `PluginSystemProvider` | IPluginManagerPort | Dynamic loading |

### 2.7 Transport

| Client | Port | Protocol |
|--------|------|----------|
| `StdioClient` | ICommandExecutorPort | External process execution |
| `SyncHttpProvider` | IHttpProviderPort | HTTP/HTTPS (reqwest blocking) |

### 2.8 Git

| Adapter | Port | Operations |
|---------|------|------------|
| `GitHookAdapter` | IHookManagerPort | Install, uninstall, check pre-commit hooks |
| `GitDiffScanner` | — | Parse `git diff --name-only` output |

### 2.9 MCP Server Infrastructure

| Module | Purpose | Key Exports |
|--------|---------|-------------|
| `McpServerWrapper` | MCP server runtime | `McpServerWrapper` struct |
| `McpServerSchemas` | Tool schema definitions | `build_tool_schemas()`, `ToolSchema` |
| `McpServerValidator` | Input validation | `validate_path()`, `validate_string()`, `ValidationError` |
| `McpServerLifespan` | Server lifecycle context | `WrapperContext` |

### 2.10 Job Tracking

| Adapter | Port | Storage |
|---------|------|---------|
| `MemoryJobRegistryAdapter` | IJobRegistryPort | In-memory HashMap + OnceLock |

## 3. Infrastructure Design Rules

### 3.1 Constructor Injection

Every adapter accepts its dependencies via constructor:
```rust
pub fn new(
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
    threshold: Option<Count>,
) -> Self;
```

### 3.2 Error Handling (CRITICAL — Never Silent)

Errors must NEVER produce silent empty results. An empty `Vec<LintResult>` is ONLY valid when ALL tools ran and found zero violations.

| Condition | Behavior | Severity |
|-----------|----------|----------|
| Tool not installed/found | Return CRITICAL `LintResult` — user must know tool is missing | CRITICAL |
| Tool crashes | Return HIGH `LintResult` with stderr content | HIGH |
| Timeout | Return HIGH `LintResult` with timeout info + partial results | HIGH |
| Parse error | Return MEDIUM `LintResult` with parse error details | MEDIUM |
| All clean | Return empty `Vec<LintResult>` — correct | — |

> **Critical rule**: NEVER return empty result for error conditions. Empty = "no violations found" which is the OPPOSITE of what happened. Every error produces at least one LintResult so the user/CI knows something went wrong.

### 3.3 MCP Server Contract

- JSON-RPC 2.0 over stdin/stdout
- Protocol version: `2024-11-05`
- Capabilities: `tools.listChanged = false`
- Methods: `initialize`, `tools/list`, `tools/call`
- Tool descriptions must match registered commands

## 4. Import & Relation Rules

### Infrastructure

| Rule | Setting |
|------|---------|
| Allowed imports | `taxonomy`, `contract` |
| Mandatory imports | `taxonomy`, `contract(port)` (AES002) |
| Forbidden imports | `surfaces`, `capabilities`, `agent`, `infrastructure` (siblings), `root` |
| AES001 | Infrastructure adapters must be isolated; importing siblings creates technical coupling and cycles |
| AES002 | Infrastructure must implement a `_port` from the contract layer |
| AES017 | Infrastructure must be registered in Agent Container and implement a port |

### Primitive Policy

| Scope | `no_primitives` |
|-------|----------------|
| `infrastructure` | `false` — may use primitive types as supporting/local types |

### Barrel (AES012)

`mod.rs` must export all public adapters. ZERO `pub mod`/`pub use` in non-barrel sub-modules (AES013).

## 5. Architectural Rules

| Rule | Constraint |
|------|------------|
| AES001 | Zero imports to capabilities, agent, surfaces, or sibling infrastructure |
| AES002 | Must import `taxonomy` + `contract(port)` |
| AES003 | Filenames: word1_word2_word3 pattern |
| AES011 | Suffixes: flexible mode — all 38 allowed suffixes (see below). Forbidden: `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate`, `_io` |
| AES012 | `mod.rs` must export all public symbols (barrel completeness) |
| AES013 | No `pub mod`/`pub use` in non-barrel sub-modules |
| AES017 | Adapter must be wired in Agent Container and implement a port |
| AES027 | Every file must implement at least one imported contract type |

### Allowed Infrastructure Suffixes (AES011)

`_adapter`, `_provider`, `_scanner`, `_client`, `_constants`, `_schemas`, `_lifespan`, `_wrapper`, `_tracer`, `_tracker`, `_variants`, `_detector`, `_patterns`, `_util`, `_system`, `_repository`, `_cache`, `_store`, `_loader`, `_writer`, `_reader`, `_driver`, `_connector`, `_gateway`, `_serializer`, `_encoder`, `_decoder`, `_fetcher`, `_watcher`, `_indexer`, `_dispatcher`, `_recorder`, `_proxy`, `_publisher`, `_subscriber`, `_listener`, `_poller`, `_streamer`

Forbidden: `_vo`, `_entity`, `_error`, `_event`, `_port`, `_protocol`, `_aggregate`, `_io`

## 6. Non-Functional Targets

| Metric | Target |
|--------|--------|
| External tool wrapping | 100% behind ILinterAdapterPort |
| Business logic in adapters | ZERO — only tool wrapping |
| Crash on missing tool | NEVER — graceful degradation |
| DI compatibility | All adapters accept Arc-wrapped ports in constructor |
| MCP protocol compliance | Valid JSON-RPC 2.0 |
| Sibling imports | ZERO (except orchestration composition) |

## 7. Success Criteria

An Infrastructure layer is **complete** when:
- Every external linter (Ruff, MyPy, Bandit, ESLint, Prettier, TSC, Clippy) has a working adapter
- Complexity, duplicate, trends, and dependency adapters produce accurate metrics
- `SourceParserOrchestrator` handles all 3 languages correctly
- Config providers discover and parse YAML config from any ancestor directory
- WatchServiceProvider polls directory for changes
- PluginSystemProvider discovers and loads plugins
- `StdioClient` executes commands with timeout and captures stdout/stderr
- `SyncHttpProvider` makes GET and POST requests
- Git hooks install/uninstall correctly
- MCP server handles initialize → tools/list → tools/call lifecycle
- Memory job registry is thread-safe
- Zero imports to capabilities, agent, or surfaces
- No public exports in non-barrel sub-modules (AES013)
