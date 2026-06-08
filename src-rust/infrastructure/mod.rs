//! # Infrastructure Layer — Technical Implementations

pub mod config;
pub mod core;
pub mod git;
pub mod javascript_adapter;
pub mod mcp;
pub mod python_adapter;
pub mod runtime;
pub mod rust_adapter;
pub mod source_parsing;

pub use config::{ConfigParserProvider, ConfigYamlReader, LanguageDetectorProvider};
pub use core::{ArchComplianceAdapter, PluginSystemProvider};
pub use git::{DiffResult, GitDiffScanner, GitHookAdapter};
pub use javascript_adapter::{
    ESLintAdapter, JSCallAdapter, JSFlowAdapter, JSScopeProvider, JSScopeTracer,
    JavascriptNamingProvider, PrettierAdapter, TSCAdapter,
};
pub use mcp::{
    build_tool_schemas, validate_path, validate_string_input, McpServerWrapper, ToolSchema,
    ValidationError, ValidationResult, WrapperContext,
};
pub use python_adapter::{
    BanditAdapter, ComplexityAdapter, DependencyAdapter, DuplicateAdapter, MetricsProvider,
    MyPyAdapter, PythonNamingVariantProvider, PythonTracer, RuffAdapter, SymbolCollector,
    TrendsAdapter,
};
pub use runtime::{
    MemoryJobRegistryAdapter, OSFileSystemAdapter, PathNormalizationProvider, StdioClient,
    SyncHttpProvider, WatchServiceProvider,
};
pub use rust_adapter::{CargoAuditAdapter, RustFmtAdapter, RustLinterAdapter};
pub use source_parsing::{
    ASTJSParserAdapter, ASTPythonParserAdapter, ASTRustParserAdapter, SourceParserOrchestrator,
};
