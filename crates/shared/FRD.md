# FRD — shared

## System Overview

The shared crate is the foundation layer of the lint-arwaky workspace. It contains all taxonomy VOs (Value Objects), contract traits/protocols/aggregates, and utility functions used by every other feature crate. It has zero dependencies on other feature crates — all other crates depend on it. The shared crate is organized into domain-specific modules that map to the AES 7-layer taxonomy.

```
shared (foundation — no feature crate dependencies)
├── common/            — Core VOs, path utilities, language detection, layer detection
├── code-analysis/     — Code analysis VOs, bypass detection, duplication detection
├── config-system/     — ArchitectureConfig, config parsing, merging, validation
├── import-rules/      — Import rule contracts, cycle detection, symbol extraction
├── naming-rules/      — Naming convention contracts and VOs
├── orphan-detector/   — Orphan detection contracts, graph analysis VOs, file I/O
├── role-rules/        — Role enforcement contracts, layer name constants
├── cli-commands/      — CLI result VOs (LintResult, Severity)
├── mcp-server/        — MCP tool contracts
├── external-lint/     — External linter adapter contracts
├── auto-fix/          — Auto-fix contracts
├── file-watch/        — File watch contracts
├── git-hooks/         — Git hook contracts
├── project-setup/     — Project setup contracts
├── maintenance/       — Maintenance contracts
└── tui/               — TUI component VOs
```

## Functional Requirements

### FR-001: Taxonomy Value Objects (common/)
- **Description**: Provide all domain-level value objects that represent the shared language of the linting system.
- **Input**: Construction from primitives or deserialization from JSON.
- **Output**: Strongly-typed VOs used across all feature crates.
- **Business Rules**:
  - All VOs implement `Serialize`, `Deserialize`, `Clone`, `Debug`, `PartialEq`.
  - VOs wrap primitives to enforce type safety: `FilePath`, `LineNumber`, `ColumnNumber`, `Severity`, `ErrorCode`, `Score`, `Timestamp`, `BooleanVO`, `Count`, `PatternList`.
  - `Severity` enum: `HIGH`, `MEDIUM`, `LOW` with `score_impact()` method.
  - `FilePath` validates non-empty path strings.
  - `LineNumber` and `ColumnNumber` use `i64` values (1-indexed).
  - `Score` supports `is_perfect()`, `is_passing(threshold)`, `deduct(severity)`.
  - `PatternList` supports flexible construction from `&str`, `String`, `Vec<String>`, `Vec<&str>`.
- **Edge Cases**: `FilePath::new("")` returns `Err`. `Score::new(f64::NAN)` is valid but `is_perfect()` returns false.
- **Error Handling**: Invalid VO construction returns `Err` with descriptive error type.

### FR-002: Lint Result VOs (cli-commands/)
- **Description**: Provide the `LintResult` struct and related types that represent a single lint violation across all features.
- **Input**: Constructed by feature crates during analysis.
- **Output**: `LintResult` serialized to JSON/SARIF/JUnit by CLI and MCP layers.
- **Business Rules**:
  - `LintResult` contains: `file`, `line`, `column`, `code` (ErrorCode), `message` (LintMessage), `source` (AdapterName), `severity`, `enclosing_scope` (ScopeRef), `related_locations` (LocationList).
  - `ScopeRef` holds function/class name, kind, file, and line range.
  - `Location` provides additional context for related code locations.
  - `ViolationConstraint` captures rule thresholds (min/max values).
- **Edge Cases**: `LintResult` with `line: 0` represents file-level violations (no specific line). Empty `related_locations` is valid.
- **Error Handling**: N/A — pure data structures.

### FR-003: Contract Traits and Aggregates
- **Description**: Define all protocol traits and aggregate traits that feature crates implement, following the AES contract layer convention.
- **Input**: N/A — trait definitions only.
- **Output**: Trait objects used for dependency injection across the workspace.
- **Business Rules**:
  - **Orphan detector contracts** (`orphan-detector/`):
    - `IOrphanAggregate` — aggregate trait for orphan detection.
    - `ITaxonomyOrphanProtocol`, `IContractOrphanProtocol`, `ICapabilitiesOrphanProtocol`, `IUtilityOrphanProtocol`, `IAgentOrphanProtocol`, `ISurfacesOrphanProtocol` — layer-specific orphan indicator protocols.
    - `IOrphanGraphResolverProtocol` — graph construction protocol.
  - **Role rules contracts** (`role-rules/`):
    - `IRoleRunnerAggregate` — aggregate trait for role enforcement.
    - `ITaxonomyRoleChecker`, `IContractRoleChecker`, `ICapabilitiesRoleChecker`, `ISurfaceRoleChecker`, `IAgentRoleChecker`, `IUtilityRoleChecker` — layer-specific role checker protocols.
  - **Import rules contracts** (`import-rules/`):
    - `IImportRunnerAggregate` — aggregate trait for import rules.
    - `ICycleImportProtocol`, `IDummyImportProtocol`, `IImportForbiddenProtocol`, `IImportMandatoryProtocol`, `IUnusedImportProtocol` — import rule protocols.
  - **Code analysis contracts** (`code-analysis/`):
    - `ICodeAnalysisAggregate` — aggregate trait for code analysis.
    - `IAdapterProtocol`, `IBypassCheckerProtocol`, `IClassProtocol`, `ICodeMetricAnalyzerProtocol`, `IDeadInheritanceProtocol`, `ILineProtocol` — analysis protocols.
  - **Config system contracts** (`config-system/`):
    - `IConfigOrchestratorAggregate` — aggregate for config loading.
    - `IParserProtocol`, `IReaderProtocol`, `IValidatorProtocol`, `IWorkspaceDetectorProtocol` — config protocols.
  - All protocol traits require `Send + Sync` for thread safety.
  - Aggregate traits define the public API surface; protocol traits define internal DI boundaries.
- **Edge Cases**: A protocol with zero methods is valid (marker trait pattern). An aggregate with a single method is valid.
- **Error Handling**: N/A — trait definitions only, no implementation logic.

### FR-004: Graph Analysis VOs (code-analysis/)
- **Description**: Provide value objects for import graph construction, reachability analysis, and orphan detection.
- **Input**: Constructed during graph analysis.
- **Output**: `GraphAnalysisContext`, `ImportGraph`, `InboundLinkMap`, `FileDefinitionMap`, `InheritanceMap`, `OrphanIndicatorResult`, `ReachabilityResult`.
- **Business Rules**:
  - `ImportGraph` stores forward edges (`mapping: HashMap<String, Vec<String>>`).
  - `InboundLinkMap` stores reverse edges for inbound import lookup.
  - `FileDefinitionMap` stores trait/class definitions per file.
  - `InheritanceMap` stores implementation relationships.
  - `OrphanIndicatorResult` contains `is_orphan`, `reason`, `severity`.
  - `ReachabilityResult` contains the set of reachable file paths.
  - `GraphAnalysisContext` bundles all graph data into a single context object.
- **Edge Cases**: Empty graph (zero files) produces empty maps. Graph with cycles is handled by BFS visited set.
- **Error Handling**: N/A — pure data structures.

### FR-005: Configuration Value Objects (config-system/)
- **Description**: Provide the `ArchitectureConfig` and related types that define rule configuration, layer definitions, and ignore paths.
- **Input**: Parsed from YAML/JSON configuration files.
- **Output**: `ArchitectureConfig` consumed by all feature crates.
- **Business Rules**:
  - `ArchitectureConfig` contains: `enabled` (BooleanVO), `layers` (HashMap<LayerNameVO, LayerDefinition>), `rules` (Vec<ArchitectureRule>), `naming` (NamingConfig), `ignored_paths` (FilePathList), `mandatory_class_definition` (BooleanVO).
  - `LayerDefinition` contains: `exceptions` (PatternList), `orphan` (OrphanRuleVO), `role` (RoleRuleVO), `naming` (NamingRuleVO).
  - `OrphanRuleVO` has `check_orphan` (BooleanVO) and `orphan_entry_points` (PatternList).
  - `RoleRuleVO` has flags for each role constraint (no_domain_logic, stateless_execution, etc.).
  - `ArchitectureRule` bundles rule metadata with flattened naming/code_analysis/role/orphan configs.
  - Default config: enabled=true, empty layers, empty rules, max naming segments=3.
- **Edge Cases**: Empty config is valid (all defaults). Config with `enabled: false` disables all checks.
- **Error Handling**: Malformed config falls back to defaults per field.

### FR-006: Utility Functions
- **Description**: Provide shared utility functions for file I/O, path normalization, language detection, layer detection, and signature parsing.
- **Input**: File paths, file content, configuration.
- **Output**: Computed results used by feature crates.
- **Business Rules**:
  - `utility_layer_detector` — Detect AES layer from filename prefix (`taxonomy_*`, `contract_*`, etc.). Returns `Option<String>`.
  - `utility_language_detector` — Detect programming language from file extension. Returns `LanguageVO`.
  - `utility_path_normalization` — Normalize file paths for cross-platform consistency.
  - `utility_signature_parser` — Extract method signatures from Rust/Python/TypeScript source code.
  - `utility_scope_matcher` — Match code scopes for violation reporting.
  - `utility_file_handler` — Read file contents with error recovery.
  - `utility_command_runner` — Execute external commands (for external linter adapters).
  - `utility_compliance_score` — Compute compliance scores from violation counts.
  - `utility_orphan_io` — File I/O for orphan detection (read, scan, is_dir, scan_directory_recursive).
  - `utility_orphan_filename` — Filename parsing (stem, suffix, basename).
  - `utility_orphan_path` — Path resolution and ignore checking.
  - `utility_import_resolver` — Resolve import paths across languages.
  - `utility_import_symbol_extractor` — Extract imported symbols from import statements.
  - `utility_cycle_detector` — Detect circular imports in the dependency graph.
  - `utility_config_parser` — Parse YAML/JSON configuration files.
  - `utility_config_merger` — Merge multiple config sources.
  - `utility_config_defaults` — Provide default configuration values.
- **Edge Cases**: Nonexistent file path returns error, not panic. Empty file content is valid input. Unsupported language returns `LanguageVO::Unknown`.
- **Error Handling**: File I/O errors propagated as `Result`. Invalid paths return `Err`.

### FR-007: Layer Name Constants and VOs
- **Description**: Provide canonical layer name constants and value objects used for layer identification across the workspace.
- **Input**: N/A — constants and VOs.
- **Output**: `LayerNameVO`, layer name constants (`LAYER_TAXONOMY`, `LAYER_CONTRACT`, etc.).
- **Business Rules**:
  - `LayerNameVO` wraps a layer name string with serialization support.
  - Constants: `LAYER_TAXONOMY`, `LAYER_CONTRACT`, `LAYER_CAPABILITIES`, `LAYER_UTILITY`, `LAYER_AGENT`, `LAYER_SURFACES`.
  - `LayerNames` provides helper functions: `layer_taxonomy()`, `layer_contract()`, `layer_capabilities()`, `layer_utility()`, `layer_agent()`, `layer_surfaces()`, `layer_root()`, `layer_global()`.
  - Used by orphan-detector and role-rules for layer classification.
- **Edge Cases**: Case-insensitive comparison for layer name matching.
- **Error Handling**: N/A — constants and thin wrappers.

## Data Model / Entity Relationship

```
ArchitectureConfig
├── enabled: BooleanVO
├── layers: HashMap<LayerNameVO, LayerDefinition>
│   └── LayerDefinition
│       ├── exceptions: PatternList
│       ├── orphan: OrphanRuleVO
│       │   ├── check_orphan: BooleanVO
│       │   └── orphan_entry_points: PatternList
│       ├── role: RoleRuleVO
│       └── naming: NamingRuleVO
├── rules: Vec<ArchitectureRule>
├── naming: NamingConfig
├── ignored_paths: FilePathList
└── mandatory_class_definition: BooleanVO

LintResult
├── file: FilePath
├── line: LineNumber
├── column: ColumnNumber
├── code: ErrorCode
├── message: LintMessage
├── source: AdapterName
├── severity: Severity
├── enclosing_scope: Option<ScopeRef>
└── related_locations: LocationList

GraphAnalysisContext
├── import_graph: ImportGraph
├── inbound_links: InboundLinkMap
├── file_definitions: FileDefinitionMap
└── inheritance_map: InheritanceMap
```

## API Contract

| Module | Key Types / Functions | Description |
|--------|----------------------|-------------|
| `common::taxonomy_path_vo` | `FilePath` | Typed file path VO with validation |
| `common::taxonomy_severity_vo` | `Severity` | HIGH / MEDIUM / LOW enum |
| `common::taxonomy_error_vo` | `ErrorCode` | Lint rule code (e.g., "AES401") |
| `common::taxonomy_lint_vo` | `LintResult`, `ScopeRef`, `Location`, `LocationList` | Violation output types |
| `common::taxonomy_common_vo` | `BooleanVO`, `Score`, `PatternList`, `Count`, `LineNumber`, `ColumnNumber` | Primitive wrapper VOs |
| `common::taxonomy_source_vo` | `SourceContentVO`, `ContentString` | File content with metadata |
| `common::taxonomy_layer_vo` | `LayerNameVO`, `LineContentVO` | Layer identification |
| `common::taxonomy_definition_vo` | `LayerDefinition`, `NamingConfig` | Layer configuration |
| `common::utility_layer_detector` | `detect_layer_from_prefix()`, `extract_filename()` | Layer detection from filename |
| `common::utility_language_detector` | `detect_language()` | Language detection from extension |
| `common::utility_signature_parser` | `extract_trait_method_signatures()` | Method signature extraction |
| `code_analysis::taxonomy_analysis_vo` | `GraphAnalysisContext`, `ImportGraph`, `OrphanIndicatorResult` | Graph analysis types |
| `config_system::taxonomy_config_vo` | `ArchitectureConfig`, `ArchitectureRule`, `OrphanRuleVO` | Configuration types |
| `config_system::contract_config_orchestrator_aggregate` | `IConfigOrchestratorAggregate` | Config loading contract |
| `orphan_detector::contract_orphan_aggregate` | `IOrphanAggregate` | Orphan detection aggregate |
| `orphan_detector::contract_orphan_protocol` | 6 `I*OrphanProtocol` traits | Layer-specific orphan protocols |
| `role_rules::contract_role_runner_aggregate` | `IRoleRunnerAggregate` | Role enforcement aggregate |
| `role_rules::contract_*_role_protocol` | 6 `I*RoleChecker` traits | Layer-specific role protocols |
| `import_rules::contract_import_runner_aggregate` | `IImportRunnerAggregate` | Import rules aggregate |

## Integration Points

- **Internal**:
  - `common/` is the most widely imported module — used by every feature crate via `shared::taxonomy_*` and `shared::common::*` re-exports.
  - `config_system/` provides configuration to orphan-detector, role-rules, and import-rules via `IConfigOrchestratorAggregate`.
  - `code_analysis/` provides graph analysis types consumed by orphan-detector.
  - `orphan-detector/` contracts are implemented by the `orphan-detector` feature crate.
  - `role-rules/` contracts are implemented by the `role-rules` feature crate.
  - `import-rules/` contracts are implemented by the `import-rules` feature crate.
- **External**: None — the shared crate has no external dependencies beyond standard Rust crates (serde, async-trait, chrono).

## Non-functional Requirements (Detailed)

- **Performance**: VOs are lightweight wrappers; construction and cloning should be O(1) or O(n) where n is the contained data.
- **Memory**: VOs use `String` and `Vec` internally. No heap allocation beyond contained data.
- **Thread safety**: All protocol traits require `Send + Sync`. Aggregate traits require `Send + Sync`.
- **Serialization**: All VOs support JSON serialization via serde for CLI output, MCP tool responses, and report generation.
- **Zero-dependency policy**: The shared crate must NOT depend on any other feature crate. All feature crates depend on shared, never the reverse.
- **Stability**: Contract traits define the public API surface. Changes to contract traits require updating all implementors.

## Test Scenarios / QA Checklist

- [ ] `FilePath::new("")` returns `Err`.
- [ ] `FilePath::new("src/main.rs")` returns `Ok` with correct value.
- [ ] `Severity::HIGH.score_impact()` returns correct deduction value.
- [ ] `Score::new(100.0).is_perfect()` returns true.
- [ ] `Score::new(85.0).is_passing(&Score::new(80.0))` returns true.
- [ ] `PatternList::new("*.rs")` creates list with one pattern.
- [ ] `detect_layer_from_prefix("taxonomy_foo.rs")` returns `Some("taxonomy")`.
- [ ] `detect_layer_from_prefix("main.rs")` returns `None`.
- [ ] `detect_language("main.rs")` returns `LanguageVO::Rust`.
- [ ] `detect_language("app.py")` returns `LanguageVO::Python`.
- [ ] `ArchitectureConfig::default()` has `enabled: true`, empty layers, empty rules.
- [ ] `LintResult` serializes to valid JSON with all required fields.
- [ ] `GraphAnalysisContext` with empty maps represents a workspace with no imports.
- [ ] All protocol traits are object-safe (can be used as `Arc<dyn Trait>`).
- [ ] No circular dependencies between shared sub-modules.

## Assumptions & Constraints

- The shared crate is the foundation layer — it must never depend on any feature crate.
- All VOs are serializable for CLI output and MCP tool responses.
- Protocol traits define internal DI boundaries; aggregate traits define public API surfaces.
- The crate uses `async_trait` for async protocol methods.
- Configuration is loaded once and shared via `Arc` or cloning across feature crates.
- Layer name constants are string-based for cross-language compatibility.

## Glossary

| Term | Definition |
|------|------------|
| **VO** | Value Object — a typed wrapper around a primitive or collection that enforces domain invariants |
| **Aggregate** | A trait defining the public API surface of a feature crate (e.g., `IOrphanAggregate`) |
| **Protocol** | A trait defining an internal DI boundary within a feature crate (e.g., `ITaxonomyOrphanProtocol`) |
| **AES** | Architecture Enforcement Standard — the 7-layer coding convention |
| **Contract** | Pure trait definitions in the shared crate that feature crates implement |
| **Taxonomy** | The domain foundation layer — stable language of the domain, free from technical concerns |
| **DI** | Dependency Injection — wiring implementations to trait/interface contracts via `Arc<dyn Trait>` |

## Reference

- PRD: [PRD.md](../../PRD.md)
