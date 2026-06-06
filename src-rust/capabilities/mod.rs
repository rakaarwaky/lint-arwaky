//! # Capabilities Layer — Domain Logic and Core Use Cases
//!
//! This module is the **thinking layer** of the AES architecture. It contains
//! pure business logic, analysis algorithms, rule checkers, and processors.
//! Implementations are entirely agnostic of concrete infrastructure — they
//! consume only ports and protocols from the Contract layer.
//!
//! ## Layer Rules (AES Compliance)
//! - **Allowed Imports**: `src/taxonomy/` and `src/contract/` only.
//! - **Forbidden Imports**: `src/infrastructure/`, `src/agent/`, `src/surfaces/`.
//!   Violations trigger **AES001**.
//! - **Allowed Suffixes**: `_checker`, `_analyzer`, `_processor`, `_evaluator`,
//!   `_resolver`, `_validator`, `_formatter`, `_handler` (**AES011**).
//! - **Primitive Usage**: Allowed internally (supporting types), but trait
//!   signatures must use taxonomy VOs.
//!
//! ## Module Index
//!
//! | Domain | Key Types | Description |
//! |--------|-----------|-------------|
//! | **Architecture Compliance** | `ArchComplianceAnalyzer`, `ArchLintHandler` | Core orchestration for AES self-lint |
//! | **Architecture Checkers** | `ArchNamingChecker`, `ArchInternalChecker`, `ArchMetricChecker`, `ArchImportRuleChecker` | Per-file rule evaluations |
//! | **Architecture Analyzers** | `ArchOrphanAnalyzer`, `DependencyCycleAnalyzer`, `MandatoryInheritanceChecker`, `ArchRoleChecker` | Project-wide analysis |
//! | **Import Processing** | `ArchImportProcessor` | Import rule evaluation logic |
//! | **Naming** | `NamingRuleChecker`, `NamingVariantAnalyzer`, `SymbolRenamerProcessor` | Naming conventions and refactoring |
//! | **Domain Types** | `DomainTypeRuleChecker` | Primitive usage detection (AES006) |
//! | **Semantic Analysis** | `SemanticScopeAnalyzer`, `CallChainAnalyzer`, `ScopeBoundaryAnalyzer`, `ScopeBoundaryResolver` | Scope & flow analysis |
//! | **Semantic Flow** | `DataFlowAnalyzer` | Variable lifecycle tracking |
//! | **Reporting** | `ReportFormatterProcessor` | SARIF/JUnit/JSON report generation |
//! | **MCP Schema** | `McpSchemaChecker` | AES025 JSON Schema validation |
//! | **Surface Hierarchy** | `SurfaceHierarchyChecker` | AES018/AES019 passive surface checks |
//! | **Dispatch** | `DispatchRoutingChecker` | AES030/AES031/AES032 dispatch validation |
//! | **Config** | `ConfigRulesValidator` | Configuration validation logic |
//! | **Setup** | `SetupManagementProcessor` | Config generation (.env, MCP) |
//!
//! ## Capability Composition Patterns
//!
//! - **Checkers** evaluate one rule per file. Called by `ArchComplianceAnalyzer`.
//! - **Analyzers** evaluate project-wide concerns (orphans, cycles). Called in batch.
//! - **Processors** transform data (reports, renames, configs). Called on demand.
//! - **Handlers** are top-level entry points (`ArchLintHandler`) implementing protocols.

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE DECLARATIONS
// ═══════════════════════════════════════════════════════════════════════════════

// --- Architecture: Core Orchestration ---
pub mod architecture_compliance_analyzer;
pub mod architecture_lint_handler;

// --- Architecture: Per-File Checkers ---
pub mod architecture_import_checker;
pub mod architecture_internal_checker;
pub mod architecture_metric_checker;
pub mod architecture_naming_checker;
pub mod architecture_quality_checker;
pub mod architecture_role_checker;
pub mod architecture_unused_checker;

// --- Architecture: Project-Wide Analyzers ---
pub mod architecture_cycle_analyzer;
pub mod architecture_import_processor;
pub mod architecture_inheritance_checker;
pub mod architecture_orphan_analyzer;

// --- Naming & Refactoring ---
pub mod naming_renamer_processor;
pub mod naming_rule_checker;
pub mod naming_variant_analyzer;

// --- Domain Type Enforcement ---
pub mod domain_type_checker;

// --- Semantic Analysis ---
pub mod semantic_boundary_analyzer;
pub mod semantic_boundary_resolver;
pub mod semantic_flow_analyzer;
pub mod semantic_scope_analyzer;
pub mod semantic_tracer_analyzer;

// --- Reporting & Formatting ---
pub mod lint_reporting_formatter;

// --- MCP Schema Validation ---
pub mod mcp_schema_checker;

// --- Surface Hierarchy Enforcement ---
pub mod surface_hierarchy_checker;

// --- Dispatch Routing Validation ---
pub mod dispatch_routing_processor;

// --- Configuration Logic ---
pub mod config_rules_validator;

// --- Setup Management ---
pub mod setup_management_processor;

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC RE-EXPORTS (Flat Access via Barrel)
// ═══════════════════════════════════════════════════════════════════════════════

// --- Architecture: Core Orchestration ---
pub use architecture_compliance_analyzer::ArchComplianceAnalyzer;
pub use architecture_lint_handler::{
    collect_rs_files, format_report, load_config, run_lint_with_deps, ArchLintHandler,
};

// --- Architecture: Per-File Checkers ---
pub use architecture_import_checker::ArchImportRuleChecker;
pub use architecture_internal_checker::ArchInternalChecker;
pub use architecture_metric_checker::ArchMetricChecker;
pub use architecture_naming_checker::ArchNamingChecker;
pub use architecture_quality_checker::CodeQualityRuleChecker;
pub use architecture_role_checker::ArchRoleChecker;
pub use architecture_unused_checker::UnusedImportRuleChecker;

// --- Architecture: Project-Wide Analyzers ---
pub use architecture_cycle_analyzer::{detect_cycle_edges, DependencyCycleAnalyzer, DependencyEdge};
pub use architecture_import_processor::ArchImportProcessor;
pub use architecture_inheritance_checker::MandatoryInheritanceChecker;
pub use architecture_orphan_analyzer::{
    ArchOrphanAnalyzer, OrphanGraphResolver, OrphanIndicatorEvaluator,
};

// --- Naming & Refactoring ---
pub use naming_renamer_processor::SymbolRenamerProcessor;
pub use naming_rule_checker::NamingRuleChecker;
pub use naming_variant_analyzer::{NamingVariantAnalyzer, NamingVariantDict};

// --- Domain Type Enforcement ---
pub use domain_type_checker::{DomainTypeRuleChecker, PrimitiveViolation};

// --- Semantic Analysis ---
pub use semantic_boundary_analyzer::ScopeBoundaryAnalyzer;
pub use semantic_boundary_resolver::ScopeBoundaryResolver;
pub use semantic_flow_analyzer::{DataFlowAnalyzer, DataFlowEntry};
pub use semantic_scope_analyzer::SemanticScopeAnalyzer;
pub use semantic_tracer_analyzer::CallChainAnalyzer;

// --- Reporting & Formatting ---
pub use lint_reporting_formatter::ReportFormatterProcessor;

// --- MCP Schema Validation ---
pub use mcp_schema_checker::McpSchemaChecker;

// --- Surface Hierarchy Enforcement ---
pub use surface_hierarchy_checker::SurfaceHierarchyChecker;

// --- Dispatch Routing Validation ---
pub use dispatch_routing_processor::{DispatchRoutingChecker, DispatchRoutingParser, MethodArgsVO};

// --- Configuration Logic ---
pub use config_rules_validator::{ConfigRulesValidator, ValidationResult};

// --- Setup Management ---
pub use setup_management_processor::{EnvContent, McpConfig, SetupManagementProcessor};
