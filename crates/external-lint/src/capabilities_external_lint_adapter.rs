use std::path::PathBuf;
use std::sync::Arc;

use shared::code_analysis::LinterOperationError;
use shared::common::{
    AdapterError, AdapterName, ComplianceStatus, ErrorMessage, ICommandExecutorProtocol,
    PatternList, ScanError, Timeout,
};

use shared::common::ResponseData;
use shared::common::{DirectoryPath, FilePath};
use shared::external_lint::{
    IExternalLintCargoProtocol, IExternalLintCommandProtocol, IExternalLintJsProtocol,
    IExternalLintLanguageProtocol, IExternalLintPathProtocol,
};
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;

pub struct ExternalLintUtilityAdapter {
    filesystem: Arc<dyn IFilesystemAggregate>,
}

impl IExternalLintPathProtocol for ExternalLintUtilityAdapter {
    fn canonicalize_path(&self, path_str: &str) -> FilePath {
        let p = self
            .filesystem
            .canonicalize(std::path::Path::new(path_str))
            .unwrap_or_else(|_| PathBuf::from(path_str));
        FilePath::new(p.to_string_lossy().to_string()).unwrap_or_default()
    }

    fn default_working_dir(&self, path: &FilePath) -> FilePath {
        FilePath::new(".".to_string()).unwrap_or_else(|_| path.clone())
    }
}

impl IExternalLintLanguageProtocol for ExternalLintUtilityAdapter {
    fn has_python_files(&self, path: &FilePath) -> bool {
        let p = std::path::Path::new(&path.value);
        if !p.exists() {
            return p.extension().map(|e| e == "py").unwrap_or(false);
        }
        if p.is_file() {
            return p.extension().map(|e| e == "py").unwrap_or(false);
        }
        if let Ok(dir) = DirectoryPath::new(path.value.clone()) {
            self.has_py_in_dir(&dir)
        } else {
            false
        }
    }

    fn has_py_in_dir(&self, dir: &DirectoryPath) -> bool {
        self.filesystem
            .has_python_files(std::path::Path::new(&dir.value))
    }

    fn is_in_path(&self, executable: &str) -> bool {
        self.filesystem.is_executable_in_path(executable)
    }
}

#[async_trait::async_trait]
impl IExternalLintJsProtocol for ExternalLintUtilityAdapter {
    fn resolve_js_cmd(
        &self,
        executable: &str,
        args: PatternList,
        working_dir: &FilePath,
    ) -> PatternList {
        let wd = std::path::Path::new(&working_dir.value);
        if self.filesystem.has_local_bin(wd, executable) {
            let local_bin = wd.join("node_modules").join(".bin").join(executable);
            let mut cmd = vec![local_bin.to_string_lossy().to_string()];
            cmd.extend(args.values);
            return PatternList::new(cmd);
        }
        if self.is_in_path(executable) {
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
        let abs_path = self
            .filesystem
            .canonicalize(std::path::Path::new(path_str))
            .unwrap_or_else(|_| PathBuf::from(path_str));
        let mut current = if abs_path.is_file() {
            abs_path
                .parent()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| PathBuf::from("."))
        } else {
            abs_path.clone()
        };
        for _ in 0..10 {
            if self.filesystem.has_config_file(&current) {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
            }
            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }
        FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default()
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
}

impl IExternalLintCargoProtocol for ExternalLintUtilityAdapter {
    fn resolve_cargo_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        if let Some(resolved) = self.filesystem.has_cargo_toml(path_str) {
            return FilePath::new(resolved).unwrap_or_else(|_| path.clone());
        }
        FilePath::new("nonexistent_directory_for_cargo_toml".to_string()).unwrap_or_default()
    }

    fn resolve_cargo_lock_working_dir(&self, path: &FilePath) -> FilePath {
        let path_str = &path.value;
        if path_str.is_empty() {
            return path.clone();
        }
        if let Some(resolved) = self.filesystem.has_cargo_lock(path_str) {
            return FilePath::new(resolved).unwrap_or_else(|_| path.clone());
        }
        FilePath::new("nonexistent_directory_for_cargo_lock".to_string()).unwrap_or_default()
    }
}

#[async_trait::async_trait]
impl IExternalLintCommandProtocol for ExternalLintUtilityAdapter {
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

    async fn noop_apply_fix(&self) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

impl Default for ExternalLintUtilityAdapter {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalLintUtilityAdapter {
    pub fn new() -> Self {
        Self {
            filesystem: Arc::new(filesystem::FilesystemOrchestrator::new()),
        }
    }

    pub fn with_filesystem(filesystem: Arc<dyn IFilesystemAggregate>) -> Self {
        Self { filesystem }
    }
}
