# PRD — Capabilities Layer
> **Vision**: The thinking layer — all domain logic, rules, and analysis

## Layer Identity

**Layer**: Capabilities (Domain Logic Layer)
**Path**: `src-rust/capabilities/`
**Role**: Pure business logic — rule checkers, analyzers, processors, formatters, handlers
**Dependency Rule**: Can import from `taxonomy` and `contract` only. ZERO imports to `infrastructure`, `agent`, or `surfaces`.

## 1. Strategic Goal

Capabilities must become the **comprehensive rule engine** that enforces the AES architecture and provides deep code analysis across Rust, Python, and JavaScript/TypeScript. Every checker is registered, every analyzer is stateless, every formatter is pure. All output uses taxonomy types. Zero infrastructure concerns leak in.

## 2. Component Blueprint

### 2.1 Architecture Compliance Core

| Component | Role | Contract Implemented |
|-----------|------|---------------------|
| `ArchComplianceAnalyzer` | Orchestrates all per-file checkers + project-wide analyzers | IArchComplianceProtocol |
| `ArchLintHandler` | Top-level entry point for lint requests | IArchLintProtocol |

**ArchComplianceAnalyzer** must:
- Accept `ArchitectureConfig` (from taxonomy)
- Accept `Vec<SourceContentVO>` (files already parsed by Agent layer)
- Iterate all files, run each registered checker
- Run project-wide analyzers after per-file checks
- Return `Vec<LintResult>` grouped by severity

**ArchLintHandler** must:
- Accept `Arc<dyn IFileSystemPort>` + `Arc<dyn ISourceParserPort>`
- Implement `IArchLintProtocol::run_self_lint()` and `run_self_lint_dir()`
- Locate config (from YAML or default), delegate to Analyzer
- Provide `format_report()` for human-readable output

> **IMPORTANT**: Capabilities must NOT receive `Arc<dyn ISourceParserPort>` directly. Parsing must be done by Agent layer BEFORE calling any analyzer. Analyzers receive already-parsed `SourceContentVO` data. This preserves the rule that Capabilities never depend on Infrastructure implementations.

### 2.2 Per-File Checkers

Each checker evaluates ONE file against ONE rule category. Checkers receive `(file: &FilePath, config: &ArchitectureConfig, source: &SourceContentVO)` and return `Vec<LintResult>`.

| Checker | Evaluates | AES Codes |
|---------|-----------|-----------|
| `ArchNamingChecker` | Filename convention (word1_word2_word3) | AES003 |
| `ArchInternalChecker` | Layer import rules (who can import what) | AES001, AES023 |
| `ArchMetricChecker` | File size min/max thresholds | AES004, AES005 |
| `ArchImportRuleChecker` | Mandatory/forbidden imports per layer | AES002, AES010 |
| `CodeQualityRuleChecker` | Code quality metrics, complexity | — |
| `ArchRoleChecker` | Layer role mandates (statelessness, passivity) | AES021, AES022 |
| `UnusedImportRuleChecker` | Dead mandatory imports | AES015 |

### 2.3 Project-Wide Analyzers

Each analyzer evaluates the ENTIRE project against a pre-built data structure (import graph, file map, symbol table) assembled by Agent.

| Analyzer | Detects | Input | AES Codes |
|----------|---------|-------|-----------|
| `DependencyCycleAnalyzer` | Circular imports between modules | ImportGraph | AES020 |
| `ArchOrphanAnalyzer` | Files unreachable from surfaces (with sub-resolvers) | FilePathSet + surface entry points | AES017 |
| `MandatoryInheritanceChecker` | Empty/hollow contract inheritance | FileDefinitionMap | AES016, AES027 |
| `ArchRoleChecker` (project) | Agent role violations across project | FileDefinitionMap | AES021, AES024 |

**Analyzer contract**: Every analyzer receives pre-computed domain data (never raw ports):
```rust
fn analyze(
    &self,
    files: &[SourceContentVO],
    config: &ArchitectureConfig,
) -> Vec<LintResult>
```

> **Design rule**: Analyzers receive data, not dependencies. Agent is responsible for building the import graph, resolving symbols, and collecting files BEFORE calling analyzers. This keeps Capabilities pure and testable without mocking infrastructure.

### 2.4 Naming & Refactoring

| Module | Operations | Input → Output |
|--------|-----------|----------------|
| `NamingRuleChecker` | Validate naming per convention | FilePath → Vec<LintResult> |
| `NamingVariantAnalyzer` | Generate snake_case, camelCase, PascalCase, kebab-case | Identifier → NameVariants |
| `SymbolRenamerProcessor` | Project-wide symbol rename | (OldName, NewName) → Vec<FixResult> |

### 2.5 Domain Type Enforcement

| Module | Operations | AES |
|--------|-----------|-----|
| `DomainTypeRuleChecker` | Detect raw primitives in contract/taxonomy boundaries | AES006 |

### 2.6 Semantic Analysis

| Module | Operations |
|--------|-----------|
| `SemanticScopeAnalyzer` | Show enclosing function/class for each violation |
| `CallChainAnalyzer` | Trace call chains across project files |
| `ScopeBoundaryAnalyzer` | Detect cross-boundary scope violations |
| `ScopeBoundaryResolver` | Resolve symbol references across file boundaries |
| `DataFlowAnalyzer` | Track variable lifecycle within scope |

### 2.7 Reporting & Formatting

| Module | Output Formats |
|--------|----------------|
| `ReportFormatterProcessor` | Text (human-readable), JSON (machine), SARIF 2.1.0 (GitHub), JUnit XML (CI) |

### 2.8 Validation Layer

| Module | Validates | AES |
|--------|-----------|-----|
| `McpSchemaChecker` | MCP tool JSON Schema correctness | AES025 |
| `SurfaceHierarchyChecker` | Surface barrel reachability + passivity | AES018, AES019 |
| `DispatchRoutingChecker` | Method resolution in COMMAND_CATALOG | AES030, AES031 |
| `DispatchRoutingParser` | Method argument parsing | AES032 |
| `ConfigRulesValidator` | Config YAML validity | — |

### 2.9 Configuration Logic

| Module | Purpose |
|--------|---------|
| `ConfigRulesValidator` | Validate `.lint_arwaky.config.yaml` against ArchitectureConfig schema |

### 2.10 Setup Management

| Module | Produces |
|--------|----------|
| `SetupManagementProcessor` | `.env` file, MCP client config JSON (Claude, VSCode, Hermes) |

## 3. Architectural Rules

| Rule | Constraint |
|------|------------|
| AES001 | Zero imports to infrastructure, agent, or surfaces |
| AES006 | Public method signatures use taxonomy VOs |
| AES011 | Suffixes: `_analyzer`, `_actions`, `_formatters`, `_generator`, `_processor`, `_evaluator`, `_checker`, `_validator`, `_transformer`, `_calculator`, `_builder`, `_handler`, `_executor`, `_resolver`, `_compiler`, `_aggregator`, `_classifier`, `_extractor`, `_reporter`, `_mapper`, `_filter`, `_collector`, `_comparator`, `_scorer`, `_inspector`, `_reviewer`, `_assessor` (flexible mode) |
| AES030 | Every capability method in COMMAND_CATALOG must exist on target class |
| AES031 | Dispatch routes must distribute across multiple capabilities |
| AES032 | Capability calls must pass typed request/data VOs |

## 4. Non-Functional Targets

| Metric | Target |
|--------|--------|
| Infrastructure imports | ZERO — no std::process, no file I/O |
| Contract adherence | All checkers/analyzers implement at least one protocol |
| Port dependencies | ZERO — analyzers receive `SourceContentVO`, not ports |
| Return types | 100% `LintResult` / `LintResultList` |
| Statelessness | Zero mutable shared state between runs |
| Semantic coverage | Rust + Python + JavaScript/TypeScript |
| Report formats | 4: text, JSON, SARIF 2.1.0, JUnit XML |

## 5. Success Criteria

A Capabilities layer is **complete** when:
- `ArchComplianceAnalyzer` correctly orchestrates ALL per-file checkers
- `ArchLintHandler` implements `IArchLintProtocol` fully
- Project-wide analyzers detect cycles, orphans, inheritance violations, and role violations — using pre-built domain data
- Report formatter produces all 4 formats with correct structure
- Semantic analysis traces scope, call chains, and data flow for all 3 languages
- Naming variant analysis generates all 4 naming conventions
- Config rules validator catches invalid YAML
- SetupManagementProcessor generates ready-to-use configs
- Zero false positives in compliance checking
- Zero port dependencies in analyzer signatures
- Zero infrastructure code leaks into capabilities
