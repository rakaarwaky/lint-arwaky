// PURPOSE: Module declarations for language-adapters (tracers, providers, linters)
pub use shared::language_adapters::taxonomy_adapter_error::{AdapterError, ScanError, ValidationError};
pub use shared::language_adapters::taxonomy_naming_error::NamingError;
pub use shared::language_adapters::taxonomy_naming_list_vo::{
    primitive_type_list, CallChainList, ImportNameList, PrimitiveTypeList, SymbolNameList,
};
pub use shared::language_adapters::taxonomy_semantic_error::SemanticError;
pub mod infrastructure_js_call_tracer;
pub use infrastructure_js_call_tracer::JSCallAdapter;
pub mod infrastructure_js_flow_tracer;
pub use infrastructure_js_flow_tracer::JSFlowAdapter;
pub mod infrastructure_js_linter_adapter;
pub use infrastructure_js_linter_adapter::{ESLintAdapter, PrettierAdapter, TSCAdapter};
pub mod infrastructure_js_naming_provider;
pub use infrastructure_js_naming_provider::JavascriptNamingProvider;
pub mod infrastructure_js_scope_provider;
pub use infrastructure_js_scope_provider::JSScopeProvider;
pub mod infrastructure_js_scope_adapter;
pub use infrastructure_js_scope_adapter::JSScopeTracer;
pub mod infrastructure_py_quality_adapter;
pub use infrastructure_py_quality_adapter::{
    ComplexityAdapter, DependencyAdapter, DuplicateAdapter, TrendsAdapter,
};
pub mod infrastructure_py_ast_tracer;
pub use infrastructure_py_ast_tracer::PythonTracer;
pub mod infrastructure_py_bandit_adapter;
pub use infrastructure_py_bandit_adapter::BanditAdapter;
pub mod infrastructure_py_mypy_adapter;
pub use infrastructure_py_mypy_adapter::MyPyAdapter;
pub mod infrastructure_py_primitive_detector;
pub use infrastructure_py_primitive_detector::PythonPrimitiveChecker;
pub mod infrastructure_py_ruff_adapter;
pub use infrastructure_py_ruff_adapter::RuffAdapter;
pub mod infrastructure_py_symbol_scanner;
pub use infrastructure_py_symbol_scanner::SymbolCollector;
pub mod infrastructure_py_ast_util;
pub use infrastructure_py_ast_util::PythonAstUtils;
pub mod infrastructure_py_variants;
pub use infrastructure_py_variants::PythonNamingVariantProvider;
pub mod infrastructure_rs_audit_adapter;
pub use infrastructure_rs_audit_adapter::CargoAuditAdapter;
pub mod infrastructure_rs_fmt_adapter;
pub use infrastructure_rs_fmt_adapter::RustFmtAdapter;
pub mod infrastructure_rs_clippy_adapter;
pub use infrastructure_rs_clippy_adapter::RustLinterAdapter;
pub mod root_language_container;
