// PURPOSE: Module declarations for external-lint (external linter adapters)
pub use shared::common::taxonomy_adapter_error::{AdapterError, ScanError, ValidationError};

pub mod agent_external_lint_orchestrator;
pub mod capabilities_external_lint_selector;
pub mod infrastructure_external_lint_adapter;
pub mod infrastructure_language_detector_adapter;
pub mod infrastructure_stdio_client;
pub use infrastructure_stdio_client::StdioClient;
pub mod infrastructure_js_eslint_adapter;
pub use infrastructure_js_eslint_adapter::ESLintAdapter;
pub mod infrastructure_js_prettier_adapter;
pub use infrastructure_js_prettier_adapter::PrettierAdapter;
pub mod infrastructure_js_tsc_adapter;
pub use infrastructure_js_tsc_adapter::TSCAdapter;
pub mod infrastructure_py_bandit_adapter;
pub use infrastructure_py_bandit_adapter::BanditAdapter;
pub mod infrastructure_py_mypy_adapter;
pub use infrastructure_py_mypy_adapter::MyPyAdapter;
pub mod infrastructure_py_ruff_adapter;
pub use infrastructure_py_ruff_adapter::RuffAdapter;
pub mod infrastructure_rs_audit_adapter;
pub use infrastructure_rs_audit_adapter::CargoAuditAdapter;
pub mod infrastructure_rs_fmt_adapter;
pub use infrastructure_rs_fmt_adapter::RustFmtAdapter;
pub mod infrastructure_rs_clippy_adapter;
pub use infrastructure_rs_clippy_adapter::RustLinterAdapter;
pub mod root_external_lint_container;
pub use root_external_lint_container::ExternalLintContainer;
