// PURPOSE: Module declarations for external-lint (external linter adapters)
pub use shared::common::taxonomy_adapter_error::{AdapterError, ScanError, ValidationError};

pub mod agent_external_lint_orchestrator;
pub mod capabilities_stdio_client;
pub use capabilities_stdio_client::StdioClient;
pub mod capabilities_external_lint_executor;
pub use capabilities_external_lint_executor::ExternalLintExecutor;
pub mod utility_external_lint_helper;
pub mod capabilities_js_eslint_adapter;
pub use capabilities_js_eslint_adapter::ESLintAdapter;
pub mod capabilities_js_prettier_adapter;
pub use capabilities_js_prettier_adapter::PrettierAdapter;
pub mod capabilities_js_tsc_adapter;
pub use capabilities_js_tsc_adapter::TSCAdapter;
pub mod capabilities_py_bandit_adapter;
pub use capabilities_py_bandit_adapter::BanditAdapter;
pub mod capabilities_py_mypy_adapter;
pub use capabilities_py_mypy_adapter::MyPyAdapter;
pub mod capabilities_py_ruff_adapter;
pub use capabilities_py_ruff_adapter::RuffAdapter;
pub mod capabilities_rs_audit_adapter;
pub use capabilities_rs_audit_adapter::CargoAuditAdapter;
pub mod capabilities_rs_fmt_adapter;
pub use capabilities_rs_fmt_adapter::RustFmtAdapter;
pub mod capabilities_rs_clippy_adapter;
pub use capabilities_rs_clippy_adapter::RustLinterAdapter;
pub mod root_external_lint_container;
pub use root_external_lint_container::ExternalLintContainer;
