//! # Infrastructure Layer — Technical Implementations
//!
//! This module contains **concrete implementations** of the ports defined in the
//! Contract layer. Every struct here implements at least one `_port` trait.
//!
//! ## Layer Rules (AES Compliance)
//! - **Allowed Imports**: `src/taxonomy/` and `src/contract/` only.
//! - **Forbidden Imports**: `src/capabilities/`, `src/agent/`, `src/surfaces/`.
//! - **Allowed Suffixes**: `_adapter`, `_provider`, `_scanner`, `_client`,
//!   `_constants`, `_schemas`, `_lifespan`, `_validator`, `_wrapper`
//! - **Sibling Isolation**: Infrastructure modules should not import from each other
//!   except for composition patterns (e.g., `source_parser_adapter` composing scanners).
//!
//! ## Module Index
//!
//! | Domain | Adapters | Description |
//! |--------|----------|-------------|
//! | **File System** | `OSFileSystemAdapter` | std::fs-based file operations |
//! | **Source Parsing** | `SourceParserOrchestrator`, `ASTRustParserAdapter`, `ASTJSParserAdapter`, `ASTPythonParserAdapter` | Multi-language AST parsing |
//! | **Linting** | `RuffAdapter`, `MyPyAdapter`, `BanditAdapter`, `RustLinterAdapter`, `ESLintAdapter`, `PrettierAdapter`, `TSCAdapter` | External linter wrappers |
//! | **Metrics** | `MetricsProvider`, `ComplexityAdapter`, `DuplicateAdapter`, `TrendsAdapter`, `DependencyAdapter` | Code quality metrics |
//! | **Config** | `ConfigDiscoveryProvider`, `ConfigParserProvider` | Configuration loading |
//! | **Transport** | `StdioClient`, `SyncHttpProvider` | Command execution and HTTP |
//! | **Semantic** | `JSCallAdapter`, `JSFlowAdapter`, `PythonTracer`, `JSScopeTracer`, `JSScopeProvider` | Semantic analysis |
//! | **Naming** | `JavascriptNamingProvider`, `PythonNamingVariantProvider` | Naming convention variants |
//! | **Git** | `GitHookAdapter`, `GitDiffScanner` | Git integration |
//! | **Plugin** | `PluginSystemProvider` | Plugin discovery and loading |
//! | **MCP** | `McpServerWrapper`, `ToolSchema`, `WrapperContext` | MCP server infrastructure |
//! | **Job Tracking** | `MemoryJobRegistryAdapter` | In-memory job registry |
//! | **Watch** | `WatchServiceProvider` | File system watching |
//! | **Path** | `PathNormalizationProvider` | Path resolution and normalization |

// ═══════════════════════════════════════════════════════════════════════════════
// MODULE DECLARATIONS
// ═══════════════════════════════════════════════════════════════════════════════

// --- Adapters: Linting ---
pub mod architecture_compliance_adapter;
pub mod javascript_linter_adapter;
pub mod python_bandit_adapter;
pub mod python_mypy_adapter;
pub mod python_ruff_adapter;
pub mod rust_linter_adapter;
pub mod rust_fmt_adapter;
pub mod cargo_audit_adapter;

// --- Adapters: Source Parsing ---
pub mod ast_js_scanner;
pub mod ast_py_scanner;
pub mod ast_rust_scanner;
pub mod source_parser_adapter;

// --- Adapters: Semantic Analysis ---
pub mod javascript_call_adapter;
pub mod javascript_flow_adapter;
pub mod javascript_scope_adapter;
pub mod javascript_scope_provider;
pub mod python_ast_adapter;

// --- Adapters: Metrics & Analysis ---
pub mod python_analysis_adapter;
pub mod python_metrics_adapter;
pub mod python_symbol_scanner;

// --- Providers: Configuration ---
pub mod config_discovery_provider;
pub mod config_parser_provider;

// --- Providers: Naming ---
pub mod javascript_naming_provider;
pub mod naming_variant_provider;

// --- Providers: Infrastructure ---
pub mod os_fs_scanner;
pub mod plugin_system_provider;
pub mod source_path_provider;
pub mod watch_service_provider;

// --- Clients: Transport ---
pub mod http_request_client;
pub mod stdio_transport_client;

// --- Scanners: Git ---
pub mod git_diff_scanner;
pub mod git_hook_adapter;

// --- MCP Server Infrastructure ---
pub mod mcp_server_lifespan;
pub mod mcp_server_schemas;
pub mod mcp_server_validator;
pub mod mcp_server_wrapper;

// --- Job Tracking ---
pub mod memory_registry_adapter;

// ═══════════════════════════════════════════════════════════════════════════════
// PUBLIC RE-EXPORTS (Flat Access via Barrel)
// ═══════════════════════════════════════════════════════════════════════════════

// --- Linting Adapters ---
pub use architecture_compliance_adapter::ArchComplianceAdapter;
pub use javascript_linter_adapter::{ESLintAdapter, PrettierAdapter, TSCAdapter};
pub use python_bandit_adapter::BanditAdapter;
pub use python_mypy_adapter::MyPyAdapter;
pub use python_ruff_adapter::RuffAdapter;
pub use rust_linter_adapter::RustLinterAdapter;
pub use rust_fmt_adapter::RustFmtAdapter;
pub use cargo_audit_adapter::CargoAuditAdapter;

// --- Source Parsing ---
pub use ast_js_scanner::ASTJSParserAdapter;
pub use ast_py_scanner::ASTPythonParserAdapter;
pub use ast_rust_scanner::ASTRustParserAdapter;
pub use source_parser_adapter::SourceParserOrchestrator;

// --- Semantic Analysis ---
pub use javascript_call_adapter::JSCallAdapter;
pub use javascript_flow_adapter::JSFlowAdapter;
pub use javascript_scope_adapter::JSScopeTracer;
pub use javascript_scope_provider::JSScopeProvider;
pub use python_ast_adapter::PythonTracer;

// --- Metrics & Analysis ---
pub use python_analysis_adapter::{
    ComplexityAdapter, DependencyAdapter, DuplicateAdapter, TrendsAdapter,
};
pub use python_metrics_adapter::MetricsProvider;
pub use python_symbol_scanner::SymbolCollector;

// --- Configuration ---
pub use config_discovery_provider::ConfigDiscoveryProvider;
pub use config_parser_provider::ConfigParserProvider;

// --- Naming ---
pub use javascript_naming_provider::JavascriptNamingProvider;
pub use naming_variant_provider::PythonNamingVariantProvider;

// --- Infrastructure Providers ---
pub use os_fs_scanner::OSFileSystemAdapter;
pub use plugin_system_provider::PluginSystemProvider;
pub use source_path_provider::PathNormalizationProvider;
pub use watch_service_provider::WatchServiceProvider;

// --- Transport ---
pub use http_request_client::SyncHttpProvider;
pub use stdio_transport_client::StdioClient;

// --- Git ---
pub use git_diff_scanner::{DiffResult, GitDiffScanner};
pub use git_hook_adapter::GitHookAdapter;

// --- MCP Server ---
pub use mcp_server_lifespan::WrapperContext;
pub use mcp_server_schemas::{build_tool_schemas, ToolSchema};
pub use mcp_server_validator::{
    validate_path, validate_string_input, ValidationError, ValidationErrorType, ValidationResult,
};
pub use mcp_server_wrapper::McpServerWrapper;

// --- Job Tracking ---
pub use memory_registry_adapter::MemoryJobRegistryAdapter;
