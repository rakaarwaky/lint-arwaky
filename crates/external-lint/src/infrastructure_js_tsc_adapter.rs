// PURPOSE: TSCAdapter — ILinterAdapterPort implementation for TypeScript compiler integration

use regex::Regex;
use shared::cli_commands::contract_executor_port::ICommandExecutorPort;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_result_vo::LintResultList;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_adapter_port::ILinterAdapterPort;
use shared::code_analysis::taxonomy_operation_error::LinterOperationError;
use shared::source_parsing::contract_path_normalization_port::IPathNormalizationPort;
use shared::source_parsing::taxonomy_adapter_error::ScanError;
use shared::source_parsing::taxonomy_path_vo::FilePath;
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
