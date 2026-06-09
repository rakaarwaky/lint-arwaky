/// javascript_linter_adapter — ESLint, Prettier, and TSC adapters for JS/TS linting.
use crate::cli_commands::contract_executor_port::ICommandExecutorPort;
use crate::code_analysis::contract_adapter_port::ILinterAdapterPort;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_error::AdapterError;
use crate::shared_common::taxonomy_adapter_error::ScanError;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_error::ErrorMessage;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_common_vo::PatternList;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_message_vo::ComplianceStatus;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_operation_error::LinterOperationError;
use crate::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use regex::Regex;
use serde_json::Value;
use std::path::Path;
use std::sync::Arc;

fn is_bun_available() -> bool {
    std::process::Command::new("bun")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
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
                None => return FilePath::new(".".to_string()).unwrap_or_default(),
            }
        } else {
            abs_path
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
    }
    FilePath::new(".".to_string()).unwrap_or_default()
}

// ── Prettier Adapter ───────────────────────────────────────────────────────

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
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    60.0,
                )),
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
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    60.0,
                )),
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

// ── TSC Adapter ────────────────────────────────────────────────────────────

pub struct TSCAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
}

impl TSCAdapter {
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
impl ILinterAdapterPort for TSCAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("tsc")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if Path::new(path_str).is_file()
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let mut args = vec![
            "--noEmit".to_string(),
            "--pretty".to_string(),
            "false".to_string(),
        ];
        if abs_path != "." && abs_path != "./" {
            args.push(abs_path);
        }

        let cmd = resolve_js_cmd("tsc", args, &wd.value);

        let response = match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd.clone(),
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    60.0,
                )),
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

        let output = format!("{}{}", response.stdout, response.stderr);
        let mut results = Vec::new();

        let pattern1 = match Regex::new(r"^([^(]+)\((\d+),(\d+)\):\s+error\s+(TS\d+):\s+(.*)$") {
            Ok(r) => r,
            Err(_) => return Ok(LintResultList::new(vec![])),
        };
        let pattern2 = match Regex::new(r"^([^:]+):(\d+):(\d+)\s+-\s+error\s+(TS\d+):\s+(.*)$") {
            Ok(r) => r,
            Err(_) => return Ok(LintResultList::new(vec![])),
        };

        for line in output.lines() {
            let line = line.trim();
            if let Some(caps) = pattern1.captures(line).or_else(|| pattern2.captures(line)) {
                let filename = caps.get(1).map(|m| m.as_str()).unwrap_or("").to_string();
                let line_num = caps
                    .get(2)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or(1);
                let col_num = caps
                    .get(3)
                    .and_then(|m| m.as_str().parse::<usize>().ok())
                    .unwrap_or(0);
                let code = caps.get(4).map(|m| m.as_str()).unwrap_or("").to_string();
                let msg = caps.get(5).map(|m| m.as_str()).unwrap_or("").to_string();

                let filename_vo = self.path_norm.resolve_infrastructure_path(
                    FilePath::new(filename).unwrap_or_default(),
                    Some(path.clone()),
                );

                results.push(LintResult {
                    file: filename_vo,
                    line: LineNumber::new(line_num as i64),
                    column: ColumnNumber::new(col_num as i64),
                    code: ErrorCode::raw(&code),
                    message: LintMessage::new(msg),
                    source: Some(self.name()),
                    severity: Severity::HIGH,
                    enclosing_scope: Default::default(),
                    related_locations: Default::default(),
                });
            }
        }

        Ok(LintResultList::new(results))
    }

    async fn apply_fix(&self, _path: &FilePath) -> Result<ComplianceStatus, LinterOperationError> {
        Ok(ComplianceStatus::new(false))
    }
}

// ── ESLint Adapter ─────────────────────────────────────────────────────────

pub struct ESLintAdapter {
    executor: Arc<dyn ICommandExecutorPort>,
    path_norm: Arc<dyn IPathNormalizationPort>,
}

impl ESLintAdapter {
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
impl ILinterAdapterPort for ESLintAdapter {
    fn name(&self) -> AdapterName {
        AdapterName::raw("eslint")
    }

    async fn scan(&self, path: &FilePath) -> Result<LintResultList, LinterOperationError> {
        let path_str = &path.value;
        if Path::new(path_str).is_file()
            && !path_str.ends_with(".ts")
            && !path_str.ends_with(".tsx")
            && !path_str.ends_with(".js")
            && !path_str.ends_with(".jsx")
        {
            return Ok(LintResultList::default());
        }

        let wd = resolve_working_dir(path);
        let abs_path = match std::fs::canonicalize(path_str) {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path_str.clone(),
        };

        let cmd = resolve_js_cmd(
            "eslint",
            vec![abs_path, "--format".to_string(), "json".to_string()],
            &wd.value,
        );

        let response = match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd.clone(),
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    60.0,
                )),
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

        let stdout_str = response.stdout.to_string();
        if stdout_str.trim().is_empty() {
            return Ok(LintResultList::default());
        }

        let parsed: Value = match serde_json::from_str(&stdout_str) {
            Ok(v) => v,
            Err(e) => {
                return Err(LinterOperationError::Scan(ScanError {
                    path: path.clone(),
                    message: ErrorMessage::new(format!("Failed to parse JSON: {}", e)),
                    error_code: None,
                    adapter_name: Some(self.name()),
                    cause: None,
                }));
            }
        };

        let mut results = Vec::new();
        if let Some(files) = parsed.as_array() {
            for file_data in files {
                let filename = file_data["filePath"].as_str().unwrap_or("").to_string();
                let filename_vo = self.path_norm.resolve_infrastructure_path(
                    FilePath::new(filename).unwrap_or_default(),
                    Some(path.clone()),
                );

                if let Some(messages) = file_data["messages"].as_array() {
                    for msg in messages {
                        let line_num = msg["line"].as_u64().unwrap_or(1) as usize;
                        let col_num = msg["column"].as_u64().unwrap_or(0) as usize;
                        let rule_id = msg["ruleId"].as_str().unwrap_or("ESLINT").to_string();
                        let message_text = msg["message"].as_str().unwrap_or("").to_string();
                        let sev_code = msg["severity"].as_u64().unwrap_or(1);

                        let severity = if sev_code == 2 {
                            Severity::HIGH
                        } else {
                            Severity::MEDIUM
                        };

                        results.push(LintResult {
                            file: filename_vo.clone(),
                            line: LineNumber::new(line_num as i64),
                            column: ColumnNumber::new(col_num as i64),
                            code: ErrorCode::raw(rule_id),
                            message: LintMessage::new(message_text),
                            source: Some(self.name()),
                            severity,
                            enclosing_scope: Default::default(),
                            related_locations: Default::default(),
                        });
                    }
                }
            }
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

        let cmd = resolve_js_cmd("eslint", vec![abs_path, "--fix".to_string()], &wd.value);

        match self
            .executor
            .execute_command(
                PatternList::new(cmd),
                wd,
                Some(crate::shared_common::taxonomy_duration_vo::Timeout::new(
                    60.0,
                )),
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
