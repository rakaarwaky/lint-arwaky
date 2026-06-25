// PURPOSE: PrettierAdapter — ILinterAdapterPort implementation for Prettier integration

use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::common::contract_path_normalization_port::IPathNormalizationPort;
use shared::common::taxonomy_adapter_error::AdapterError;
use shared::common::taxonomy_adapter_error::ScanError;
use shared::common::taxonomy_path_vo::FilePath;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_error::ErrorMessage;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_common_vo::PatternList;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_message_vo::ComplianceStatus;
use shared::taxonomy_message_vo::LintMessage;
use std::path::Path;
use std::sync::Arc;

fn is_bun_available() -> bool {
    match std::process::Command::new("bun")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
    {
        Ok(s) => s.success(),
        Err(_) => false,
    }
}

fn resolve_js_cmd(executable: &str, args: Vec<String>, working_dir: &str) -> Vec<String> {
    let local_bin = Path::new(working_dir)
        .join("node_modules")
        .join(".bin")
        .join(executable);

    if local_bin.exists() {
        let mut cmd = vec![local_bin.to_string_lossy().to_string()];
        cmd.extend(args);
        return cmd;
    }

    let runner = if is_bun_available() { "bunx" } else { "npx" };
    let mut cmd = vec![runner.to_string(), executable.to_string()];
    cmd.extend(args);
    cmd
}

fn resolve_working_dir(path: &FilePath) -> FilePath {
    let path_str = &path.value;
    if let Ok(abs_path) = std::fs::canonicalize(path_str) {
        let mut current = if abs_path.is_file() {
            match abs_path.parent() {
                Some(p) => p.to_path_buf(),
                None => std::path::PathBuf::from("."),
            }
        } else {
            abs_path.clone()
        };

        for _ in 0..10 {
            if current.join("lint_arwaky.config.yaml").is_file()
                || current.join("lint_arwaky.config.python.yaml").is_file()
                || current.join("package.json").is_file()
                || current.join(".git").is_dir()
            {
                return FilePath::new(current.to_string_lossy().to_string()).unwrap_or_default();
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

pub struct PrettierAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
}

impl PrettierAdapter {
    pub fn new(
        executor: Arc<dyn ICommandExecutorPort>,
        path_norm: Arc<dyn IPathNormalizationPort>,
    ) -> Self {
        Self {
            executor,
            path_norm,
        }
    }
}

#[async_trait::async_trait]
impl ILinterAdapterPort for PrettierAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("prettier")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if Path::new(path_str).is_file()
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
            && !path_str.ends_with(".json")
            && !path_str.ends_with(".css")
            && !path_str.ends_with(".md")
            && !path_str.ends_with(".yml")
            && !path_str.ends_with(".yaml")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let cmd = resolve_js_cmd("prettier", vec!["--check".to_string(), abs_path], &wd.value);

        let response = match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd.clone(),
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(r) => r,
            Err(e) => {
                return Err(LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(e.to_string()),
                    error_code: None,
                    adapter_name: Some(self.name()),
                    cause: None,
                }));
            }
        };

        let mut results = Vec::new();
        let combined_output = format!("{}{}", response.stdout, response.stderr);

        if combined_output.contains("[warn]") {
            let filename_vo = self
                .path_norm
                .resolve_infrastructure_path(path.clone(), Some(path.clone()));
            results.push(LintResult {
                file: filename_vo,
                line: LineNumber::new(1),
                column: ColumnNumber::new(0),
                code: ErrorCode::raw("formatting"),
                message: LintMessage::new("Code style issues found. Run Prettier to fix."),
                source: Some(self.name()),
                severity: Severity::LOW,
                enclosing_scope: Default::default(),
                related_locations: Default::default(),
            });
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        let path_str = &path.value;
        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let cmd = resolve_js_cmd("prettier", vec!["--write".to_string(), abs_path], &wd.value);

        match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd,
                Some(shared::taxonomy_duration_vo::Timeout::new(60.0)),
            )
            .await
        {
            Ok(r) => Ok(ComplianceStatus::new(r.returncode == 0)),
            Err(e) => Err(LinterOperationError::Adapter(AdapterError {
                adapter_name: self.name(),
                message: ErrorMessage::new(e.to_string()),
                error_code: None,
                command: None,
                stderr: Some(ErrorMessage::new("")),
                exit_code: None,
            })),
        }
    }
}
