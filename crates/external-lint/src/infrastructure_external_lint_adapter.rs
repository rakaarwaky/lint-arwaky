use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_adapter_name_vo::AdapterName;
use shared::common::taxonomy_common_error::ErrorMessage;
use shared::common::taxonomy_common_vo::{BooleanVO, PatternList};
use shared::common::taxonomy_duration_vo::Timeout;
use shared::common::taxonomy_message_vo::ComplianceStatus;
use shared::common::taxonomy_path_vo::{DirectoryPath, FilePath};
use shared::common::taxonomy_response_data_vo::ResponseData;
use shared::external_lint::contract_external_lint_utility_port::IExternalLintUtilityPort;
use std::path::{Path, PathBuf};

pub struct ExternalLintUtilityAdapter;

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

#[async_trait::async_trait]
impl IExternalLintUtilityPort for ExternalLintUtilityAdapter {
    fn canonicalize_path(&self, path_str: &str) -> FilePath {
        match std::fs::canonicalize(path_str) {
            Ok(p) => FilePath::new(p.to_string_lossy().to_string()).unwrap_or_default(),
            Err(_) => FilePath::new(path_str.to_string()).unwrap_or_default(),
        }
    }

    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
    }

    fn has_python_files(&self, path: &FilePath) -> BooleanVO {
        let p = std::path::Path::new(&path.value);
        if !p.exists() {
            return BooleanVO::new(p.extension().map(|e| e == "py").unwrap_or(false));
        }
        if p.is_file() {
            return BooleanVO::new(p.extension().map(|e| e == "py").unwrap_or(false));
        }
        if let Ok(dir) = DirectoryPath::new(path.value.clone()) {
            self.has_py_in_dir(&dir)
        } else {
            BooleanVO::new(false)
        }
    }

    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: PatternList,
        working_dir: &FilePath,
    ) -> PatternList {
        let local_bin = Path::new(&working_dir.value)
            .join("node_modules")
            .join(".bin")
            .join(executable);
        if local_bin.exists() {
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
        if let Ok(abs_path) = std::fs::canonicalize(path_str) {
            let mut current = if abs_path.is_file() {
                abs_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or_else(|| PathBuf::from("."))
            } else {
                abs_path.clone()
            };
            for _ in 0..10 {
                if current.join("lint_arwaky.config.yaml").is_file()
                    || current.join("lint_arwaky.config.python.yaml").is_file()
                    || current.join("package.json").is_file()
                    || current.join(".git").is_dir()
                {
                    return FilePath::new(current.to_string_lossy().to_string())
                        .unwrap_or_default();
                }
                match current.parent() {
                    Some(parent) => current = parent.to_path_buf(),
                    None => break,
                }
            }
            return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
        }
        FilePath::new(".".to_string()).unwrap_or_default()
    }

    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        let current = Path::new(path_str);
        if current.is_dir() {
            if current.join("Cargo.toml").exists() {
                return path.clone();
            }
        } else if let Some(parent) = current.parent() {
            if parent.join("Cargo.toml").exists() {
                return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
            if let Some(grandparent) = parent.parent() {
                if grandparent.join("Cargo.toml").exists() {
                    return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|_| path.clone());
                }
            }
        }
        FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
    }

    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        let current = Path::new(path_str);
        if current.is_dir() {
            if current.join("Cargo.lock").exists() {
                return path.clone();
            }
        } else if let Some(parent) = current.parent() {
            if parent.join("Cargo.lock").exists() {
                return FilePath::new(parent.to_string_lossy().replace('\\', "/"))
                    .unwrap_or_else(|_| path.clone());
            }
            if let Some(grandparent) = parent.parent() {
                if grandparent.join("Cargo.lock").exists() {
                    return FilePath::new(grandparent.to_string_lossy().replace('\\', "/"))
                        .unwrap_or_else(|_| path.clone());
                }
            }
        }
        FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
    }

    async fn exec_cmd_scan(
        &self,
        executor: &dyn ICommandExecutorPort,
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
        executor: &dyn ICommandExecutorPort,
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
        executor: &dyn ICommandExecutorPort,
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

    fn has_py_in_dir(&self, dir: &DirectoryPath) -> BooleanVO {
        let Ok(entries) = std::fs::read_dir(&dir.value) else {
            return BooleanVO::new(false);
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Ok(sub_dir) = DirectoryPath::new(path.to_string_lossy().to_string()) {
                    if self.has_py_in_dir(&sub_dir).value {
                        return BooleanVO::new(true);
                    }
                }
            } else if path.extension().map(|e| e == "py").unwrap_or(false) {
                return BooleanVO::new(true);
            }
        }
        BooleanVO::new(false)
    }

    fn is_in_path(&self, executable: &str) -> BooleanVO {
        if let Ok(path_var) = std::env::var("PATH") {
            for path_dir in std::env::split_paths(&path_var) {
                let path = path_dir.join(executable);
                if path.is_file() {
                    return BooleanVO::new(true);
                }
            }
        }
        BooleanVO::new(false)
    }
}
