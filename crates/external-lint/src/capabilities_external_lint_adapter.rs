use shared::cli_commands::contract_executor_protocol::ICommandExecutorProtocol;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_common_vo::{bool, PatternList};
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_utility_protocol::IExternalLintUtilityProtocol;
use shared::external_lint::utility_external_lint_io as ext_io;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct ExternalLintUtilityAdapter;

// ─── Block 2: Protocol Trait Implementation ───────────────

#[async_trait::async_trait]
impl IExternalLintUtilityProtocol for ExternalLintUtilityAdapter {
    fn canonicalize_path(&self, path_str: &str) -> FilePath {
        let p = ext_io::canonicalize_path(path_str);
        FilePath::new(p.to_string_lossy().to_string()).unwrap_or_default()
    }

    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
    }

    fn has_python_files(&self, path: &FilePath) -> bool {
        let p = std::path::Path::new(&path.value);
        if !ext_io::path_exists(p) {
            return bool::new(p.extension().map(|e| e == "py").unwrap_or(false));
        }
        if ext_io::is_file(p) {
            return bool::new(p.extension().map(|e| e == "py").unwrap_or(false));
        }
        if let Ok(dir) = DirectoryPath::new(path.value.clone()) {
            self.has_py_in_dir(&dir)
        } else {
            bool::new(false)
        }
    }

    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: PatternList,
        working_dir: &FilePath,
    ) -> PatternList {
        let wd = std::path::Path::new(&working_dir.value);
        if ext_io::has_local_bin(wd, executable) {
            let local_bin = wd.join("node_modules").join(".bin").join(executable);
            let mut cmd = vec![local_bin.to_string_lossy().to_string()];
            cmd.extend(args.values);
            return PatternList::new(cmd);
        }
        if self.is_in_path(executable).value {
            let mut cmd = vec![executable.to_string()];
            cmd.extend(args.values);
            return PatternList::new(cmd);
        }
        let mut cmd = vec![executable.to_string()];
        cmd.extend(args.values);
        PatternList::new(cmd)
    }

    fn resolve_js_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        let abs_path = ext_io::canonicalize_path(path_str);
        let mut current = if ext_io::is_file(&abs_path) {
            abs_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            abs_path.clone()
        };
        for _ in 0..10 {
            if ext_io::has_config_file(&current) {
                return FilePath::new(current.to_string_lossy().to_string())
                    .unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default()
    }

    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        if let Some(resolved) = ext_io::has_cargo_toml(path_str) {
            return FilePath::new(resolved).unwrap_or_else(|_| path.clone());
        }
        FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
    }

    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        if let Some(resolved) = ext_io::has_cargo_lock(path_str) {
            return FilePath::new(resolved).unwrap_or_else(|_| path.clone());
        }
        FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
    }

    async fn exec_cmd_scan(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        args: PatternList,
        working_dir: FilePath,
        timeout_secs: Timeout,
        adapter_name: Option<AdapterName>,
        path: &FilePath,
    ) -> Result<ResponseData, LinterOperationError> {
        executor
            .execute_command(args, working_dir, Some(timeout_secs))
            .await
            .map_err(|e| {
                LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name,
                    cause: None,
                })
            })
    }

    async fn exec_cmd_adapter(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        args: PatternList,
        working_dir: FilePath,
        timeout_secs: Timeout,
        adapter_name: AdapterName,
    ) -> Result<ResponseData, LinterOperationError> {
        executor
            .execute_command(args, working_dir, Some(timeout_secs))
            .await
            .map_err(|e| {
                LinterOperationError::Adapter(AdapterError::new(
                    adapter_name,
                    ErrorMessage::new(e.to_string()),
                ))
            })
    }

    async fn js_apply_fix(
        &self,
        executor: &dyn ICommandExecutorProtocol,
        path: &FilePath,
        tool: &str,
        fix_arg: &str,
    ) -> Result<ComplianceStatus, LinterOperationError> {
        let wd = self.resolve_js_working_dir(path);
        let abs_path = self.canonicalize_path(&path.value);
        let cmd = self.resolve_js_cmd(
            tool,
            PatternList::new(vec![abs_path.value, fix_arg.to_string()]),
            &wd,
        );
        let response = self
            .exec_cmd_adapter(
                executor,
                cmd,
                wd,
                Timeout::new(60.0),
                AdapterName::raw(tool),
            )
            .await?;
        Ok(ComplianceStatus::new(response.returncode == 0))
    }

    async fn noop_apply_fix(&self) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }

    fn has_py_in_dir(&self, dir: &DirectoryPath) -> bool {
        ext_io::has_python_files(&dir.value)
    }

    fn is_in_path(&self, executable: &str) -> bool {
        ext_io::is_executable_in_path(executable)
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl ExternalLintUtilityAdapter {
    pub fn new() -> Self {
        Self
    }
}

impl Default for ExternalLintUtilityAdapter {
    fn default() -> Self {
        Self::new()
    }
}

