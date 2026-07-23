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
  - All VOs support serialization, cloning, debugging, and equality comparison.
  - VOs wrap primitives to enforce type safety: file path, line number, column number, severity, error code, score, timestamp, boolean flag, count, pattern list.
  - Severity levels: HIGH, MEDIUM, LOW with score impact calculation.
  - File path validates non-empty path strings.
  - Line number and column number use integer values (1-indexed).
  - Score supports perfection check, threshold passing, and severity deduction.
  - Pattern list supports flexible construction from strings and string lists.
- **Edge Cases**: Empty file path returns error. Score with NaN is valid but perfection check returns false.
- **Error Handling**: Invalid VO construction returns `Err` with descriptive error type.

### FR-002: Lint Result VOs (cli-commands/)

- **Description**: Provide the lint result struct and related types that represent a single lint violation across all features.
- **Input**: Constructed by feature crates during analysis.
- **Output**: Lint results serialized to JSON/SARIF/JUnit by CLI and MCP layers.
- **Business Rules**:
  - Lint result contains: file, line, column, code (ErrorCode), message (LintMessage), source (AdapterName), severity, enclosing scope (ScopeRef), related locations (LocationList).
  - Scope reference holds function/class name, kind, file, and line range.
  - `Location` provides additional context for related code locations.
  - Violation constraint captures rule thresholds (min/max values).
- **Edge Cases**: Lint result with `line: 0` represents file-level violations (no specific line). Empty related locations is valid.
- **Error Handling**: N/A — pure data structures.

### FR-003: Contract Traits and Aggregates

- **Description**: Define all protocol traits and aggregate traits that feature crates implement, following the AES contract layer convention.
- **Input**: N/A — trait definitions only.
- **Output**: Trait objects used for dependency injection across the workspace.
- **Business Rules**:
  - **Orphan detector contracts** (orphan detection module):
    - Orphan detection aggregate — aggregate trait for orphan detection.
    - Layer-specific orphan indicator protocols — taxonomy, contract, capabilities, utility, agent, and surfaces orphan indicator protocols.
    - Graph construction protocol — protocol for building the import graph.
  - **Role rules contracts** (role enforcement module):
    - Role enforcement aggregate — aggregate trait for role enforcement.
    - Layer-specific role checker protocols — taxonomy, contract, capabilities, surface, agent, and utility role checker protocols.
  - **Import rules contracts** (import rules module):
    - Import rules aggregate — aggregate trait for import rules.
    - Import rule protocols — cycle import, dummy import, forbidden import, mandatory import, and unused import protocols.
  - **Code analysis contracts** (code analysis module):
    - Code analysis aggregate — aggregate trait for code analysis.
    - Analysis protocols — adapter, bypass checker, class, code metric analyzer, dead inheritance, and line protocols.
  - **Config system contracts** (config system module):
    - Config loading aggregate — aggregate for config loading.
    - Config protocols — parser, reader, validator, and workspace detector protocols.
  - All protocol traits require `Send + Sync` for thread safety.
  - Aggregate traits define the public API surface; protocol traits define internal DI boundaries.
- **Edge Cases**: A protocol with zero methods is valid (marker trait pattern). An aggregate with a single method is valid.
- **Error Handling**: N/A — trait definitions only, no implementation logic.

### FR-004: Graph Analysis VOs (code-analysis/)

- **Description**: Provide value objects for import graph construction, reachability analysis, and orphan detection.
- **Input**: Constructed during graph analysis.
- **Output**: Graph analysis context, import graph, reverse link map, file definition map, inheritance map, orphan indicator result, and reachability result.
- **Business Rules**:
  - Import graph stores forward edges.
  - Inbound link map stores reverse edges for inbound import lookup.
  - File definition map stores trait/class definitions per file.
  - Inheritance map stores implementation relationships.
  - Orphan indicator result contains is_orphan, reason, and severity.
  - Reachability result contains the set of reachable file paths.
  - Graph analysis context bundles all graph data into a single context object.
- **Edge Cases**: Empty graph (zero files) produces empty maps. Graph with cycles is handled by BFS visited set.
- **Error Handling**: N/A — pure data structures.

### FR-005: Configuration Value Objects (config-system/)

- **Description**: Provide the architecture configuration and related types that define rule configuration, layer definitions, and ignore paths.
- **Input**: Parsed from YAML/JSON configuration files.
- **Output**: Architecture configuration consumed by all feature crates.
- **Business Rules**:
  - Architecture config contains: enabled flag, layers map (layer name to layer definition), rules list, naming config, ignored paths list, mandatory class definition flag.
  - Layer definition contains: exceptions (PatternList), orphan (OrphanRuleVO), role (RoleRuleVO), naming (NamingRuleVO).
  - Orphan rule VO has check_orphan (BooleanVO) and orphan_entry_points (PatternList).
  - Role rule VO has flags for each role constraint (no_domain_logic, stateless_execution, etc.).
  - Architecture rule bundles rule metadata with flattened naming/code_analysis/role/orphan configs.
  - Default config: enabled=true, empty layers, empty rules, max naming segments=3.
- **Edge Cases**: Empty config is valid (all defaults). Config with `enabled: false` disables all checks.
- **Error Handling**: Malformed config falls back to defaults per field.

### FR-006: Utility Functions

- **Description**: Provide shared utility functions for file I/O, path normalization, language detection, layer detection, and signature parsing.
- **Input**: File paths, file content, configuration.
- **Output**: Computed results used by feature crates.
- **Business Rules**:
  - Layer detection utility — Detect AES layer from filename prefix (`taxonomy_*`, `contract_*`, etc.). Returns optional layer name string.
  - Language detection utility — Detect programming language from file extension. Returns language value object.
  - Path normalization utility — Normalize file paths for cross-platform consistency.
  - Signature parser utility — Extract method signatures from Rust/Python/TypeScript source code.
  - Scope matcher utility — Match code scopes for violation reporting.
  - File handler utility — Read file contents with error recovery.
  - Command runner utility — Execute external commands (for external linter adapters).
  - Compliance score utility — Compute compliance scores from violation counts.
  - Orphan file I/O utility — File I/O for orphan detection (read, scan, is_dir, scan_directory_recursive).
  - Orphan filename utility — Filename parsing (stem, suffix, basename).
  - Orphan path utility — Path resolution and ignore checking.
  - Import resolver utility — Resolve import paths across languages.
  - Import symbol extractor utility — Extract imported symbols from import statements.
  - Cycle detector utility — Detect circular imports in the dependency graph.
  - Config parser utility — Parse YAML/JSON configuration files.
  - Config merger utility — Merge multiple config sources.
  - Config defaults utility — Provide default configuration values.
- **Edge Cases**: Nonexistent file path returns error, not panic. Empty file content is valid input. Unsupported language returns unknown language value object.
- **Error Handling**: File I/O errors propagated as `Result`. Invalid paths return `Err`.

### FR-007: Layer Name Constants and VOs

- **Description**: Provide canonical layer name constants and value objects used for layer identification across the workspace.
- **Input**: N/A — constants and VOs.
- **Output**: Layer name value object with layer name constants (taxonomy, contract, capabilities, utility, agent, surfaces).
- **Business Rules**:
  - Layer name value object wraps a layer name string with serialization support.
  - Constants for all six architectural layers.
  - Layer name helper functions for each layer plus root and global.
  - Used by orphan-detector and role-rules for layer classification.
- **Edge Cases**: Case-insensitive comparison for layer name matching.
- **Error Handling**: N/A — constants and thin wrappers.

## Data Model / Entity Relationship

```
Architecture Config
├── enabled: boolean flag
├── layers: map of layer name to layer definition
│   └── Layer Definition
│       ├── exceptions: pattern list
│       ├── orphan: orphan rule
│       │   ├── check_orphan: boolean flag
│       │   └── orphan_entry_points: pattern list
│       ├── role: role rule
│       └── naming: naming rule
├── rules: list of architecture rules
├── naming: naming config
├── ignored_paths: list of file paths
└── mandatory_class_definition: boolean flag

Lint Result
├── file: file path
├── line: line number
├── column: column number
├── code: error code
├── message: lint message
├── source: adapter name
├── severity: severity level
├── enclosing_scope: optional scope reference
└── related_locations: list of related locations

Graph Analysis Context
├── import_graph: import graph
├── inbound_links: inbound link map
├── file_definitions: file definition map
└── inheritance_map: inheritance map
```

## API Contract


| Module                               | Key Types / Functions                                                      | Description                        |
| -------------------------------------- | ---------------------------------------------------------------------------- | ------------------------------------ |
| common path value object             | file path value object                                                                 | Typed file path value object with validation |
| common severity value object         | severity value object                                                                 | HIGH / MEDIUM / LOW levels           |
| common error value object            | error code                                                                | Lint rule code (e.g., "AES401")    |
| common lint value object             | lint result, scope reference, location, location list                       | Violation output types             |
| common primitive value objects       | boolean flag, score, pattern list, count, line number, column number | Primitive wrapper value objects              |
| common source content value object   | source content value object, content string                                         | File content with metadata         |
| common layer value object            | layer name value object, line content value object                                             | Layer identification               |
| common definition value object       | layer definition, naming config                                            | Layer configuration                |
| common layer detection utility       | layer detection from filename prefix                                       | Layer detection from filename      |
| common language detection utility    | language detection from file extension                                     | Language detection from extension  |
| common signature parser utility      | method signature extraction                                                | Method signature extraction        |
| code analysis graph value objects    | graph analysis context, import graph, orphan indicator result              | Graph analysis types               |
| config system configuration types    | architecture config, architecture rule, orphan rule                        | Configuration types                |
| config system orchestrator aggregate | config loading aggregate                                                   | Config loading contract            |
| orphan detection aggregate           | orphan detection aggregate                                                 | Orphan detection aggregate         |
| orphan detection protocols           | layer-specific orphan protocols                                            | Layer-specific orphan protocols    |
| role enforcement aggregate           | role enforcement aggregate                                                 | Role enforcement aggregate         |
| role enforcement protocols           | layer-specific role protocols                                              | Layer-specific role protocols      |
| import rules aggregate               | import rules aggregate                                                     | Import rules aggregate             |

## Integration Points

- **Internal**:
  - The common module is the most widely imported module — used by every feature crate via shared taxonomy and common re-exports.
  - The config system module provides configuration to orphan-detector, role-rules, and import-rules via the config loading aggregate.
  - The code analysis module provides graph analysis types consumed by orphan-detector.
  - The orphan detection contracts are implemented by the orphan-detector feature crate.
  - The role enforcement contracts are implemented by the role-rules feature crate.
  - The import rules contracts are implemented by the import-rules feature crate.
- **External**: None — the shared crate has no external dependencies beyond standard library crates.

## Non-functional Requirements (Detailed)

- **Performance**: VOs are lightweight wrappers; construction and cloning should be O(1) or O(n) where n is the contained data.
- **Memory**: VOs use `String` and `Vec` internally. No heap allocation beyond contained data.
- **Thread safety**: All protocol traits require `Send + Sync`. Aggregate traits require `Send + Sync`.
- **Serialization**: All VOs support JSON serialization via serde for CLI output, MCP tool responses, and report generation.
- **Zero-dependency policy**: The shared crate must NOT depend on any other feature crate. All feature crates depend on shared, never the reverse.
- **Stability**: Contract traits define the public API surface. Changes to contract traits require updating all implementors.

## Test Scenarios / QA Checklist

- [ ]  File path with empty string returns error.
- [ ]  File path with valid path returns success with correct value.
- [ ]  Severity HIGH score impact returns correct deduction value.
- [ ]  Score 100.0 is perfect returns true.
- [ ]  Score 85.0 passing threshold 80.0 returns true.
- [ ]  Pattern list with "*.rs" creates list with one pattern.
- [ ]  Layer detection from prefix "taxonomy_foo.rs" returns taxonomy.
- [ ]  Layer detection from prefix "main.rs" returns none.
- [ ]  Language detection for "main.rs" returns Rust.
- [ ]  Language detection for "app.py" returns Python.
- [ ]  Architecture config default has enabled: true, empty layers, empty rules.
- [ ]  Lint result serializes to valid JSON with all required fields.
- [ ]  Graph analysis context with empty maps represents a workspace with no imports.
- [ ]  All protocol traits are object-safe for dependency injection.
- [ ]  No circular dependencies between shared sub-modules.

## Assumptions & Constraints

- The shared crate is the foundation layer — it must never depend on any feature crate.
- All VOs are serializable for CLI output and MCP tool responses.
- Protocol traits define internal DI boundaries; aggregate traits define public API surfaces.
- The crate uses async traits for async protocol methods.
- Configuration is loaded once and shared via dependency injection across feature crates.
- Layer name constants are string-based for cross-language compatibility.

## Glossary


| Term          | Definition                                                                                       |
| --------------- | -------------------------------------------------------------------------------------------------- |
| **VO**        | Value Object — a typed wrapper around a primitive or collection that enforces domain invariants |
| **Aggregate** | A trait defining the public API surface of a feature crate                                       |
| **Protocol**  | A trait defining an internal DI boundary within a feature crate                                  |
| **AES**       | Architecture Enforcement Standard — the 7-layer coding convention                               |
| **Contract**  | Pure trait definitions in the shared crate that feature crates implement                         |
| **Taxonomy**  | The domain foundation layer — stable language of the domain, free from technical concerns       |
| **DI**        | Dependency Injection — wiring implementations to trait/interface contracts                      |

## Reference

- PRD: [PRD.md](../../PRD.md)
