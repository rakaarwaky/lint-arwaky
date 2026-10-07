// PURPOSE: Module declarations for external-lint (external linter adapters)
pub use shared_common::taxonomy_adapter_error::{AdapterError, ScanError, ValidationError};

/// Convert between the two `LinterOperationError` types in the shared crate.
/// `common::LinterOperationError` is used by the external-lint protocol seams.
/// `quality_rules::LinterOperationError` is used by `ILinterAdapterProtocol`.
pub(crate) fn convert_executor_error(
    e: shared_common::taxonomy_adapter_error::LinterOperationError,
) -> shared_quality_rules::LinterOperationError {
    use shared_common::taxonomy_adapter_error::LinterOperationError as CommonErr;
    use shared_quality_rules::LinterOperationError as QaErr;
    match e {
        CommonErr::Scan(s) => QaErr::Scan(s),
        CommonErr::Adapter(a) => QaErr::Adapter(a),
    }
}

/// Map a raw subprocess failure onto the scan error the caller expects (FR-006).
pub(crate) fn map_scan_error(
    e: anyhow::Error,
    path: shared_common::taxonomy_path_vo::FilePath,
    adapter_name: Option<shared_common::taxonomy_adapter_name_vo::AdapterName>,
) -> shared_common::taxonomy_adapter_error::LinterOperationError {
    use shared_common::ScanError;
    use shared_common::taxonomy_common_vo::ErrorMessage;
    shared_common::taxonomy_adapter_error::LinterOperationError::Scan(ScanError {
        path,
        message: ErrorMessage::new(e.to_string()),
        error_code: None,
        adapter_name,
        cause: None,
        error_id: shared_common::ErrorId::raw(2),
    })
}

/// Map a raw subprocess failure onto the adapter error the caller expects (FR-006).
pub(crate) fn map_adapter_error(
    e: anyhow::Error,
    adapter_name: shared_common::taxonomy_adapter_name_vo::AdapterName,
) -> shared_common::taxonomy_adapter_error::LinterOperationError {
    use shared_common::taxonomy_adapter_error::AdapterError;
    use shared_common::taxonomy_common_vo::ErrorMessage;
    shared_common::taxonomy_adapter_error::LinterOperationError::Adapter(AdapterError::new(
        adapter_name,
        ErrorMessage::new(e.to_string()),
    ))
}

/// Map a raw subprocess failure onto the adapter error type used by
/// `ILinterAdapterProtocol` implementations (shared_quality_rules).
/// Shared by `rs_audit`, `rs_clippy`, and `rs_fmt` adapters.
pub(crate) fn map_executor_err(
    e: anyhow::Error,
    adapter_name: shared_common::taxonomy_adapter_name_vo::AdapterName,
) -> shared_quality_rules::LinterOperationError {
    shared_quality_rules::LinterOperationError::Adapter(
        shared_common::taxonomy_adapter_error::AdapterError::new(
            adapter_name,
            shared_common::taxonomy_common_vo::ErrorMessage::new(e.to_string()),
        ),
    )
}

pub mod agent_external_lint_orchestrator;
pub mod capabilities_cargo_dir_resolver;
pub mod capabilities_command_executor;
pub mod capabilities_external_lint_selector;
pub mod capabilities_js_tool_resolver;
pub mod capabilities_output_normalizer;
pub use capabilities_output_normalizer::OutputNormalizer;
pub mod capabilities_stdio_client;
pub use capabilities_stdio_client::StdioClient;
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
pub mod capabilities_rs_clippy_adapter;
pub use capabilities_rs_clippy_adapter::RustLinterAdapter;
pub mod capabilities_rs_fmt_adapter;
pub use capabilities_rs_fmt_adapter::RustFmtAdapter;
pub mod capabilities_md_markdownlint_adapter;
pub use capabilities_md_markdownlint_adapter::MarkdownLintAdapter;
pub mod root_external_lint_container;
pub use root_external_lint_container::ExternalLintContainer;
