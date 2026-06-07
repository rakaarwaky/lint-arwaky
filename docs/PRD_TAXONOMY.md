# PRD — Taxonomy Layer

> **Vision**: The pure domain foundation — the vocabulary of the system

## Layer Identity

**Layer**: Taxonomy (Innermost Layer)
**Path**: `src-rust/taxonomy/`
**Role**: Pure, framework-agnostic domain models — Value Objects, Entities, Events, Errors, Constants
**Dependency Rule**: ZERO imports from any outer layer. Only self-imports allowed.

## 1. Strategic Goal

Taxonomy must become the **single source of truth** for every domain concept in the system. Every string, number, or boolean that crosses a layer boundary must be wrapped in a typed Value Object. No raw primitives in public API surfaces. Every error must be a proper domain error implementing `thiserror::Error`.

## 2. Component Blueprint

### 2.1 Value Objects (`_vo`)

Every Value Object must:

- Be IMMUTABLE (all fields private, no `mut`)
- Implement `Clone + Debug + PartialEq + Serialize`
- Validate on construction via `new()` returning `Result`
- Wrap a single primitive or a validated composition of other VOs
- Carry domain behavior (methods) where appropriate

**Required VOs by domain** (target: 30-35 VOs total):

| Domain                 | Required VOs                                                                                                   | Notes                                                   |
| ---------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| **Core**         | `LineNumber`, `ColumnNumber`, `Position`, `Score`, `Count`, `Timestamp`, `Duration`, `Timeout` | Fundamental wrappers — use generic Vec for collections |
| **Path**         | `FilePath`, `DirectoryPath`                                                                                | Use `Vec<FilePath>` instead of `FilePathList`       |
| **Lint**         | `LintResult`, `Severity`, `ErrorCode`, `LintMessage`                                                   | `LintResultList` NOT a VO — use `Vec<LintResult>`  |
| **Architecture** | `LayerNameVO`, `LayerDefinition`, `ArchitectureConfig`, `ArchitectureRule`, `GovernanceReport`       | Config + rules                                          |
| **Error**        | `ErrorMessage`, `Cause`, `ExitCode`, `FieldName`, `ModuleName`                                       | Common error components                                 |
| **Source**       | `ContentString`, `SourceContentVO`, `ImportInfo`                                                         | Parsed source data                                      |
| **Config**       | `ConfigKey`, `Thresholds`, `AppConfig`, `ProjectConfig`                                                | Application config                                      |
| **Metadata**     | `CommandMetadataVO`, `AdapterName`, `JobId`, `PluginGroup`                                             | System metadata                                         |
| **Naming**       | `SymbolName`, `NameVariants`                                                                               | Naming conventions                                      |
| **Transport**    | `TransportEndpoint`, `TransportProtocol`                                                                   | Transport config                                        |
| **Git**          | `GitRef`, `GitDiffResultVO`                                                                                | Git operations                                          |
| **Watch**        | `WatchResult`                                                                                                | Watch mode results                                      |
| **Doctor**       | `DoctorResultVO`                                                                                             | Diagnostics                                             |

> **Consolidation**: Use `Vec<LintResult>` instead of `LintResultList`. Use `HashMap<K,V>` instead of custom Map types. Avoid creating List wrapper VOs unless they carry validation logic.

### 2.2 Entities (`_entity`)

| Entity                           | Identity    | Purpose                                             |
| -------------------------------- | ----------- | --------------------------------------------------- |
| `ArchitectureGovernanceEntity` | Layer scope | Tracks architecture compliance state with lifecycle |

### 2.3 Events (`_event`)

| Event                 | Trigger                    | Fields                                     |
| --------------------- | -------------------------- | ------------------------------------------ |
| `ScanStarted`       | `check`/`scan` command | target, timestamp                          |
| `ScanCompleted`     | Scan finishes              | target, results_count, duration, timestamp |
| `ScanFailed`        | Scan crashes               | target, error, timestamp                   |
| `FixApplied`        | Auto-fix succeeds          | file, fix_type, changes, timestamp         |
| `HookInstalled`     | `install-hook`           | hook_path, timestamp                       |
| `HookRemoved`       | `uninstall-hook`         | hook_path, timestamp                       |
| `AdapterRegistered` | DI init                    | adapter_name, timestamp                    |

### 2.4 Errors (`_error`)

Every error must implement `thiserror::Error + Display + Debug + Clone`.

| Error                 | Source                  | Variants                                        |
| --------------------- | ----------------------- | ----------------------------------------------- |
| `FileSystemError`   | I/O operations          | NotFound, PermissionDenied, IoError             |
| `SourceParserError` | AST parsing failures    | SyntaxError, UnsupportedLanguage                |
| `AdapterError`      | Linter adapter failures | ToolNotFound, ExecutionFailed, ParseError       |
| `JobError`          | Job registry            | NotFound, AlreadyExists, InvalidState           |
| `ConfigError`       | Config parsing          | FileNotFound, ParseError, ValidationError       |
| `TransportError`    | Stdio/HTTP transport    | ConnectionFailed, Timeout, ProtocolError        |
| `NamingError`       | Naming validation       | InvalidConvention, ReservedName                 |
| `MetricsError`      | Metrics calculation     | CalculationError, InsufficientData              |
| `PluginError`       | Plugin system           | DiscoveryError, LoadFailed, RegistrationError   |
| `WatchServiceError` | File watcher            | PollError, SubscriptionError                    |
| `GitHookError`      | Git hooks               | InstallFailed, RemoveFailed                     |
| `SemanticError`     | Semantic analysis       | ResolutionFailed, CircularReference, ScopeError |

### 2.5 Constants (`_constant`)

Constants are compile-time literals. RULES:

- ONLY `pub const` / `pub static` declarations allowed
- NO `struct`, `enum`, `fn`, `impl`, `mod`, `pub mod` — trigger **AES033**
- For cross-cutting values shared across multiple VOs/layers

| Constant File                | Contents                                                                                                                          | Used By          |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------------- | ---------------- |
| `mcp_server_constant`      | `MCP_SERVER_VERSION`, `AUTO_LINT_VERSION`, `MAX_BATCH_SIZE`, `MAX_PATH_DEPTH`, `MAX_PATH_LENGTH`, `MAX_STRING_LENGTH` | MCP server       |
| `layer_names_constant`     | Canonical layer name constants                                                                                                    | All layers       |
| `lint_score_constant`      | Report format identifiers:`FORMAT_TEXT`, `FORMAT_JSON`, `FORMAT_SARIF`, `FORMAT_JUNIT`                                    | Reporting        |
| `naming_symbols_constant`  | `CORE_PRIMITIVE_TYPES`                                                                                                          | AES006 checker   |
| `command_catalog_constant` | `COMMAND_CATALOG`                                                                                                               | Command dispatch |

## 3. Barrel (`mod.rs`) Requirements

The Taxonomy barrel must:

- Declare all modules with `pub mod`
- Re-export ALL public types via `pub use` (AES012)
- ZERO `pub mod` / `pub use` in non-barrel sub-modules (AES013)
- Provide factory functions: `line_number(42)`, `file_path("...")`, `score(95.5)`, etc.
- Provide type aliases for common complex types (NOT for simple Vec wrappers)
- Provide utility functions (e.g., `compute_score()`)

## 4. Import & Relation Rules

### Taxonomy Value Objects (`_vo`)

| Rule              | Setting                                                                                                                 |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------- |
| Allowed imports   | `taxonomy` only                                                                                                       |
| Mandatory imports | None                                                                                                                    |
| Forbidden imports | `taxonomy(entity,error,event)`, `agent`, `infrastructure`, `surfaces`, `contract`, `capabilities`, `root` |
| AES001            | Taxonomy VO must have zero dependencies on outer layers or other taxonomy roles                                         |

### Taxonomy Entity, Error, Event (`_entity`, `_error`, `_event`)

| Rule              | Setting                                                                                             |
| ----------------- | --------------------------------------------------------------------------------------------------- |
| Allowed imports   | `taxonomy` only                                                                                   |
| Mandatory imports | `taxonomy(vo)` — AES002                                                                          |
| Forbidden imports | `agent`, `infrastructure`, `surfaces`, `contract`, `capabilities`, `root`               |
| AES001            | Core domain structures must be composed of Value Objects and must not depend on outer system layers |

### Taxonomy Primitives (AES006)

| Sub-layer     | `no_primitives`                                                  |
| ------------- | ------------------------------------------------------------------ |
| `_vo`       | `false` — VO internals may use primitives as underlying storage |
| `_entity`   | `true` — entities must use VOs                                  |
| `_error`    | `true` — errors must use VOs                                    |
| `_event`    | `true` — events must use VOs                                    |
| `_constant` | `false` — constants are primitives by definition                |

## 5. Architectural Rules

| Rule   | Constraint                                                                         |
| ------ | ---------------------------------------------------------------------------------- |
| AES001 | Zero imports to any outer layer                                                    |
| AES002 | Entity/Error/Event must import from `taxonomy(vo)`                               |
| AES003 | Filenames: word1_word2_word3 pattern                                               |
| AES006 | Primitive types forbidden in entity/error/event — must use VOs                    |
| AES011 | Suffixes:`_vo`, `_entity`, `_event`, `_error`, `_constant` only          |
| AES012 | `mod.rs` must export all public symbols (barrel completeness)                    |
| AES013 | No `pub mod`/`pub use` in non-barrel sub-modules                               |
| AES017 | Taxonomy component must be consumed by at least one outer layer                    |
| AES033 | `_constant` files: ONLY `pub const` / `pub static` — no struct/enum/fn/impl |

## 6. Non-Functional Targets

| Metric                | Target                                                |
| --------------------- | ----------------------------------------------------- |
| External dependencies | Zero (except serde + chrono)                          |
| VO count              | 30-35 — avoid List wrappers, use generic Vec/HashMap |
| Entities              | 1-2 maximum                                           |
| Events                | 7 standard domain events                              |
| Errors                | 12 domain error types                                 |
| Constants             | 5 constant files                                      |
| Primitive wrapping    | 100% — no raw primitives in public APIs              |
| Factory functions     | 20+ ergonomic constructors in barrel                  |

## 7. Success Criteria

A Taxonomy layer is **complete** when:

- Every concept shared across layers has a typed VO
- Contract trait signatures use ONLY taxonomy VOs
- Collections use generic `Vec<T>` / `HashMap<K,V>` — no List wrapper VOs
- All errors implement `thiserror::Error + Display + Debug + Clone`
- Events capture every meaningful domain occurrence
- Constants are PURE — zero struct/enum/fn contamination (AES033)
- Barrel provides clean public API with factory functions
- YAML config maps directly to `ArchitectureConfig` VO
- Zero outer-layer imports
- Entity/Error/Event all import from `taxonomy(vo)`
