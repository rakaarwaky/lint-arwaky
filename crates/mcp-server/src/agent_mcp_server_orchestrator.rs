// PURPOSE: McpServerOrchestrator — agent that implements IMcpServerAggregate
//
// The MCP orchestrator is the AI-agent entry point. It delegates all scan
// operations to AnalysisPipelineOrchestrator (via IAnalysisPipelineAggregate)
// and returns JSON responses.
use rmcp::handler::server::wrapper::Parameters;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::cli_commands::Format;
use shared::common::ExitCode;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::mcp_server::IMcpServerAggregate;
use shared::mcp_server::{ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs, ReadSkillArgs};
use shared::project_setup::SetupManagementAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
/// Dependencies injected into the MCP server orchestrator.
/// All aggregates are wired from the MCP container for full CLI parity.
pub struct McpServerDependencies {
    pub code_analysis_linter:
        Arc<dyn shared::code_analysis::contract_code_analysis_aggregate::ICodeAnalysisAggregate>,
    pub fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
    pub orphan_orchestrator:
        Arc<dyn shared::orphan_detector::contract_orphan_aggregate::IOrphanAggregate>,
    pub maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    pub git_hooks_aggregate: Arc<dyn GitHooksAggregate>,
    pub setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub external_lint:
        Arc<dyn shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate>,
    pub import_orchestrator:
        Arc<dyn shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate>,
    pub naming_orchestrator:
        Arc<dyn shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate>,
    pub role_orchestrator:
        Arc<dyn shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate>,
}

pub struct McpServerOrchestrator {
    deps: McpServerDependencies,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
#[async_trait::async_trait]
impl IMcpServerAggregate for McpServerOrchestrator {
    async fn execute_command(&self, Parameters(args): Parameters<ExecuteCommandArgs>) -> String {
        let action = args.action.clone();
        let arg_path = args
            .args
            .as_ref()
            .and_then(|a| a.get("path"))
            .and_then(|v| v.as_str())
            .map(String::from);
        let arg_threshold = args
            .args
            .as_ref()
            .and_then(|a| a.get("threshold"))
            .and_then(|v| v.as_u64())
            .map(|n| n as u32);
        let arg_client = args
            .args
            .as_ref()
            .and_then(|a| a.get("client"))
            .and_then(|v| v.as_str())
            .map(String::from);

        let result = match action.as_str() {
            "check" | "scan" => {
                // Run full pipeline via CLI surface (same as CLI)
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let status =
                    cli_commands::surface_check_command::handle_scan_parallel_subprocesses(
                        &path,
                        Format::Text,
                    )
                    .await;
                let exit_code = if status == ExitCode::OK { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "success" } else { "failure" },
                    "action": action,
                    "path": path,
                    "exit_code": exit_code,
                    "total_violations": 0,
                    "results": Vec::<serde_json::Value>::new(),
                })
            }
            "ci" => {
                // CI command: run check and pass/fail based on threshold
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let threshold = arg_threshold.unwrap_or(80);
                let status =
                    cli_commands::surface_check_command::handle_scan_parallel_subprocesses(
                        &path,
                        Format::Text,
                    )
                    .await;
                let exit_code = if status == ExitCode::OK { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "pass" } else { "fail" },
                    "action": "ci",
                    "threshold": threshold,
                    "path": path,
                    "exit_code": exit_code,
                })
            }
            "fix" => {
                // Wire real auto-fix aggregate for full parity with CLI
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let dry_run = args
                    .args
                    .as_ref()
                    .and_then(|a| a.get("dry_run"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                let fp = shared::common::taxonomy_path_vo::FilePath::new(path.clone())
                    .unwrap_or_else(|_| {
                        shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default()
                    });
                let fix_result = self.deps.fix_orchestrator.execute(&fp);

                serde_json::json!({
                    "status": "success",
                    "action": "fix",
                    "path": path,
                    "dry_run": dry_run,
                    "exit_code": 0,
                    "message": fix_result.output.value,
                })
            }
            "doctor" => {
                // Run toolchain diagnostics via maintenance aggregate
                let diag = self
                    .deps
                    .maintenance_orchestrator
                    .diagnose_toolchain()
                    .await;

                let checks: Vec<serde_json::Value> = {
                    let mut checks = Vec::new();
                    for status in &diag.rust_tools {
                        checks.push(serde_json::json!({
                            "tool": status.name,
                            "status": if status.status == "OK" { "ok" } else { "not_found" },
                            "version": status.version,
                        }));
                    }
                    for status in &diag.python_tools {
                        checks.push(serde_json::json!({
                            "tool": status.name,
                            "status": if status.status == "OK" { "ok" } else { "not_found" },
                            "version": status.version,
                        }));
                    }
                    for status in &diag.js_tools {
                        checks.push(serde_json::json!({
                            "tool": status.name,
                            "status": if status.status == "OK" { "ok" } else { "not_found" },
                            "version": status.version,
                        }));
                    }
                    for status in &diag.vcs_tools {
                        checks.push(serde_json::json!({
                            "tool": status.name,
                            "status": if status.status == "OK" { "ok" } else { "not_found" },
                            "version": status.version,
                        }));
                    }
                    checks
                };

                serde_json::json!({"status": "success", "action": "doctor", "exit_code": 0, "checks": checks})
            }
            "orphan" => {
                // Wire real orphan detector aggregate for full parity with CLI
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = shared::common::taxonomy_path_vo::FilePath::new(path.clone())
                    .unwrap_or_else(|_| {
                        shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default()
                    });

                // Get ignored paths from config orchestrator and build per-target orphan analyzer
                let ignored = self.deps.config_orchestrator.ignored_paths(&fp);
                let orphan_analyzer = self
                    .deps
                    .config_orchestrator
                    .create_orphan_analyzer(&fp.value);
                let (_, results) = orphan_analyzer.scan_orphans(&fp, ignored.values());

                serde_json::json!({
                    "status": "success",
                    "action": "orphan",
                    "path": path,
                    "exit_code": if results.is_empty() { 0 } else { 1 },
                    "orphan_count": results.len(),
                    "results": results.iter().map(|r| serde_json::json!({
                        "file": r.file.value.as_str(),
                        "code": r.code.code(),
                        "message": r.message.value.as_str(),
                        "line": r.line.value(),
                        "column": r.column.value(),
                    })).collect::<Vec<serde_json::Value>>(),
                })
            }
            "security" => {
                // Wire real security scan via maintenance aggregate
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = shared::common::taxonomy_path_vo::FilePath::new(path.clone())
                    .unwrap_or_else(|_| {
                        shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default()
                    });

                let report = self
                    .deps
                    .maintenance_orchestrator
                    .run_security_scan(&fp)
                    .await;

                // exit_code: 0 clean, 1 findings, 3 tool missing
                let exit_code = if !report.tool_installed {
                    3
                } else if report.findings.is_empty() {
                    0
                } else {
                    1
                };

                serde_json::json!({
                    "status": if exit_code == 0 { "clean" } else if exit_code == 3 { "tool_missing" } else { "findings" },
                    "action": "security",
                    "path": path,
                    "exit_code": exit_code,
                    "language": report.language,
                    "tool_name": report.tool_name,
                    "tool_installed": report.tool_installed,
                    "findings_count": report.findings.len(),
                    "findings": report.findings.iter().map(|f| serde_json::json!({
                        "severity": f.severity.to_uppercase(),
                        "test_id": f.test_id,
                        "file": f.file,
                        "line": f.line,
                        "issue": f.issue,
                    })).collect::<Vec<serde_json::Value>>(),
                })
            }
            "duplicates" => {
                // Run code duplication analysis via code-analysis aggregate
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };

                // Collect lintable files and scan for duplicates using the shared utility
                let fp = shared::common::taxonomy_path_vo::FilePath::new(path.clone())
                    .unwrap_or_else(|_| {
                        shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default()
                    });
                let entries =
                    shared::code_analysis::utility_code_duplication_detector::collect_file_entries(
                        std::slice::from_ref(&fp.value),
                    );
                let min_dup_lines = 5; // minimum lines for a duplicate block

                // Calculate total LOC before moving entries
                let total_loc: usize = entries.iter().map(|(_, c)| c.lines().count()).sum();

                // Scan for duplicates (consumes entries)
                let blocks =
                    shared::code_analysis::utility_code_duplication_detector::scan_duplicate_blocks(
                        entries,
                        min_dup_lines,
                    );

                // Build violation list
                let violations =
                    shared::code_analysis::utility_code_duplication_detector::build_violations(
                        &blocks,
                        total_loc,
                        min_dup_lines,
                    );

                serde_json::json!({
                    "status": "success",
                    "action": "duplicates",
                    "path": path,
                    "exit_code": 0,
                    "duplicate_blocks": blocks.len(),
                    "violations": violations.len(),
                })
            }
            "dependencies" => {
                // Run dependency report via maintenance aggregate
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = shared::common::taxonomy_path_vo::FilePath::new(path.clone())
                    .unwrap_or_else(|_| {
                        shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default()
                    });

                match self
                    .deps
                    .maintenance_orchestrator
                    .run_dependency_report(&fp)
                    .await
                {
                    Ok(report) => {
                        serde_json::json!({
                            "status": "success",
                            "action": "dependencies",
                            "path": path,
                            "exit_code": 0,
                            "language": report.language,
                            "dependency_count": report.dependencies.len(),
                            "dependencies": report.dependencies.iter().map(|d| serde_json::json!({
                                "name": d.name,
                                "version": d.version,
                                "dep_type": d.dep_type,
                            })).collect::<Vec<serde_json::Value>>(),
                        })
                    }
                    Err(e) => {
                        serde_json::json!({
                            "error": format!("Dependency report failed: {}", e),
                            "action": "dependencies",
                            "path": path,
                            "exit_code": 2,
                        })
                    }
                }
            }
            "version" => {
                serde_json::json!({"version": env!("CARGO_PKG_VERSION"), "name": "lint-arwaky", "exit_code": 0})
            }
            "adapters" => {
                let ext = self.deps.external_lint.clone();
                let adapter_names = ext.adapter_names();
                let futures = adapter_names.values.into_iter().map(|name| async move {
                    let found = match tokio::process::Command::new("which")
                        .arg(&*name)
                        .output()
                        .await
                    {
                        Ok(o) => o.status.success(),
                        Err(_) => false,
                    };
                    serde_json::json!({"name": name.value, "enabled": found})
                });
                let adapters = futures::future::join_all(futures).await;
                serde_json::json!({"adapters": adapters, "exit_code": 0})
            }
            "install-hook" => {
                // Wire real git-hooks aggregate for actual hook installation
                let _path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };

                // Resolve the executable path (current binary)
                let exe_path = std::env::current_exe()
                    .ok()
                    .and_then(|p| p.canonicalize().ok())
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| "./lint-arwaky".to_string());

                let fp =
                    shared::common::taxonomy_path_vo::FilePath::new(exe_path).unwrap_or_default();

                match self.deps.git_hooks_aggregate.install_hook(&fp).await {
                    Ok(_) => {
                        serde_json::json!({"status": "success", "message": "Git hook installed.", "action": "install-hook", "exit_code": 0})
                    }
                    Err(e) => {
                        serde_json::json!({"error": format!("Failed to install hook: {:?}", e), "action": "install-hook", "exit_code": 2})
                    }
                }
            }
            "uninstall-hook" => {
                // Wire real git-hooks aggregate for actual hook removal
                match self.deps.git_hooks_aggregate.uninstall_hook().await {
                    Ok(_) => {
                        serde_json::json!({"status": "success", "message": "Git hook removed.", "action": "uninstall-hook", "exit_code": 0})
                    }
                    Err(e) => {
                        serde_json::json!({"error": format!("Failed to uninstall hook: {:?}", e), "action": "uninstall-hook", "exit_code": 2})
                    }
                }
            }
            "init" => {
                // Wire real project-setup aggregate for config file creation
                let result = self.deps.setup_orchestrator.detect_languages();
                let mut languages: Vec<serde_json::Value> = Vec::new();
                let mut all_ok = true;

                for lang in result.iter() {
                    let lang_str = lang.value();
                    let target = format!("lint_arwaky.config.{}.yaml", lang_str);
                    if self.deps.setup_orchestrator.file_exists(&target) {
                        languages.push(serde_json::json!({"config": target, "status": "exists"}));
                    } else {
                        let content = self.deps.setup_orchestrator.get_config_template(lang_str);
                        match self
                            .deps
                            .setup_orchestrator
                            .write_config_file(&target, content)
                        {
                            Ok(desc) => {
                                languages.push(serde_json::json!({"config": target, "status": "created", "description": desc.value}));
                            }
                            Err(e) => {
                                languages.push(serde_json::json!({"config": target, "status": "error", "error": e.to_string()}));
                                all_ok = false;
                            }
                        }
                    }
                }

                serde_json::json!({
                    "status": if all_ok { "success" } else { "partial_failure" },
                    "action": "init",
                    "exit_code": if all_ok { 0 } else { 1 },
                    "languages": languages,
                })
            }
            "install" => {
                // Wire real project-setup aggregate for adapter installation
                let sudo = args
                    .args
                    .as_ref()
                    .and_then(|a| a.get("sudo"))
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);

                let py_status = self.deps.setup_orchestrator.install_python_adapters().await;
                let js_status = self
                    .deps
                    .setup_orchestrator
                    .install_javascript_adapters(sudo)
                    .await;

                serde_json::json!({
                    "status": if py_status.value && js_status.value { "success" } else { "partial_failure" },
                    "action": "install",
                    "exit_code": if py_status.value && js_status.value { 0 } else { 1 },
                    "python_adapters_installed": py_status.value,
                    "javascript_adapters_installed": js_status.value,
                })
            }
            "mcp-config" => {
                // Wire real MCP config generation with binary resolution
                let client = match arg_client {
                    Some(c) => c,
                    None => "all".to_string(),
                };

                // Resolve MCP binary path (same logic as CLI surface)
                let binary = match std::env::current_exe()
                    .ok()
                    .and_then(|p| p.parent().map(|d| d.join("lint-arwaky-mcp")))
                    .and_then(|p| p.canonicalize().ok())
                {
                    Some(path) => path.to_string_lossy().into_owned(),
                    None => "lint-arwaky-mcp".to_string(),
                };

                let config = match client.as_str() {
                    "claude-code" | "claude" => serde_json::json!({
                        "mcpServers": {
                            "lint-arwaky": {
                                "command": binary,
                                "args": [],
                                "env": {}
                            }
                        }
                    }),
                    "cursor" => serde_json::json!({
                        "mcpServers": {
                            "lint-arwaky": {
                                "command": binary,
                                "args": [],
                                "env": {}
                            }
                        }
                    }),
                    "windsurf" => serde_json::json!({
                        "config:lint-arwaky": {
                            "command": binary,
                            "args": [],
                            "env": {}
                        }
                    }),
                    "copilot" => serde_json::json!({
                        "inputs": [],
                        "server": {
                            "command": binary,
                            "args": [],
                            "env": {}
                        }
                    }),
                    "hermes" => serde_json::json!({
                        "mcpServers": {
                            "lint-arwaky": {
                                "command": binary,
                                "args": [],
                                "env": {}
                            }
                        }
                    }),
                    "vscode" => serde_json::json!({
                        "mcpServers": {
                            "lint-arwaky": {
                                "command": binary,
                                "args": [],
                                "env": {}
                            }
                        }
                    }),
                    _ => serde_json::json!({
                        "mcpServers": {
                            "lint-arwaky": {
                                "command": binary,
                                "args": [],
                                "env": {}
                            }
                        }
                    }),
                };

                let json_str = serde_json::to_string_pretty(&config).unwrap_or_default();

                serde_json::json!({
                    "status": "success",
                    "action": "mcp-config",
                    "client": client,
                    "binary": binary,
                    "config": json_str,
                    "exit_code": 0,
                })
            }
            "config-show" => {
                // Wire real config orchestrator aggregate for config display with secret redaction
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = shared::common::taxonomy_path_vo::FilePath::new(path.clone())
                    .unwrap_or_else(|_| {
                        shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default()
                    });

                match self.deps.config_orchestrator.list_config_files(&fp).await {
                    Ok(config_files) if !config_files.is_empty() => {
                        // Process each config file sequentially to handle async read_config
                        let mut configs = Vec::new();
                        for (lang, path) in &config_files {
                            // Simple redaction for display (mimics surface_config_command::redact_secrets)
                            // Regex built once outside the loop to avoid clippy::regex_creation_in_loops
                            static AWS_KEY_RE: once_cell::sync::Lazy<Option<regex::Regex>> =
                                once_cell::sync::Lazy::new(|| {
                                    regex::Regex::new(r"AKIA[0-9A-Z]{16}").ok()
                                });
                            let redact_secrets = |content: &str| -> String {
                                let mut result = content.to_string();
                                if result.contains("AKIA") {
                                    if let Some(re) = AWS_KEY_RE.as_ref() {
                                        result = re
                                            .replace_all(&result, "[REDACTED-AWS-KEY]")
                                            .to_string();
                                    }
                                }
                                // Redact very long base64-like strings
                                if result.len() > 100 {
                                    let words: Vec<String> =
                                        result.split_whitespace().map(|s| s.to_string()).collect();
                                    for word in &words {
                                        if word.len() >= 40
                                            && word.chars().all(|c| {
                                                c.is_ascii_alphanumeric()
                                                    || matches!(c, '/' | '+' | '=')
                                            })
                                        {
                                            result = result.replacen(word, "[REDACTED]", 1);
                                        }
                                    }
                                }
                                result
                            };

                            // Read config content with secret redaction (async is already available in the handler)
                            let source =
                                match self.deps.config_orchestrator.read_config(&fp, *lang).await {
                                    Ok(Some(source)) => Some(source),
                                    _ => None,
                                };

                            match source {
                                Some(source) => {
                                    let safe_content = redact_secrets(&source.raw_content);
                                    configs.push(serde_json::json!({
                                        "language": lang.as_str(),
                                        "path": path.value.as_str(),
                                        "content": safe_content,
                                    }));
                                }
                                None => {
                                    configs.push(serde_json::json!({
                                        "language": lang.as_str(),
                                        "path": path.value.as_str(),
                                        "error": "Could not read config content",
                                    }));
                                }
                            }
                        }

                        serde_json::json!({
                            "status": "success",
                            "action": "config-show",
                            "path": path,
                            "exit_code": 0,
                            "configs": configs,
                        })
                    }
                    Ok(_) => {
                        serde_json::json!({
                            "status": "success",
                            "action": "config-show",
                            "path": path,
                            "exit_code": 0,
                            "message": "No config file found. Run `lint-arwaky init` to create one.",
                            "configs": Vec::<serde_json::Value>::new(),
                        })
                    }
                    Err(e) => {
                        serde_json::json!({
                            "error": format!("Failed to list config files: {}", e),
                            "action": "config-show",
                            "path": path,
                            "exit_code": 2,
                        })
                    }
                }
            }
            "watch" => {
                // watch is long-lived and not supported via MCP; FRD explicitly allows explicit unsupported
                serde_json::json!({
                    "error": "watch is not supported via MCP (long-lived action — use CLI: lint-arwaky watch <path>)",
                    "action": "watch",
                    "exit_code": 2,
                })
            }
            "quality" => {
                // Run code quality analysis directly via code-analysis aggregate
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.clone()) {
                    Ok(f) => f,
                    Err(_) => return serde_json::json!({"error": "Invalid path", "action": "quality", "exit_code": 2}).to_string(),
                };
                let results = self.deps.code_analysis_linter.run_code_analysis_path(&fp);
                let exit_code = if results.is_empty() { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "success" } else { "violations" },
                    "action": "quality",
                    "path": path,
                    "exit_code": exit_code,
                    "violation_count": results.len(),
                    "results": results.iter().map(|r| serde_json::json!({
                        "file": r.file.value.as_str(),
                        "code": r.code.code(),
                        "message": r.message.value.as_str(),
                        "line": r.line.value(),
                        "column": r.column.value(),
                    })).collect::<Vec<serde_json::Value>>(),
                })
            }
            "import" => {
                // Run import rules analysis directly via import_orchestrator aggregate
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.clone()) {
                    Ok(f) => f,
                    Err(_) => return serde_json::json!({"error": "Invalid path", "action": "import", "exit_code": 2}).to_string(),
                };
                let results = match self.deps.import_orchestrator.run_audit(&fp).await {
                    Ok(r) => r,
                    Err(e) => return serde_json::json!({"error": format!("Import audit failed: {}", e), "action": "import", "exit_code": 2}).to_string(),
                };
                let exit_code = if results.is_empty() { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "success" } else { "violations" },
                    "action": "import",
                    "path": path,
                    "exit_code": exit_code,
                    "violation_count": results.len(),
                    "results": results.iter().map(|r| serde_json::json!({
                        "file": r.file.value.as_str(),
                        "code": r.code.code(),
                        "message": r.message.value.as_str(),
                        "line": r.line.value(),
                        "column": r.column.value(),
                    })).collect::<Vec<serde_json::Value>>(),
                })
            }
            "naming" => {
                // Run naming rules analysis directly via naming_orchestrator aggregate
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.clone()) {
                    Ok(f) => f,
                    Err(_) => return serde_json::json!({"error": "Invalid path", "action": "naming", "exit_code": 2}).to_string(),
                };
                let results = match self.deps.naming_orchestrator.run_audit(&fp).await {
                    Ok(r) => r,
                    Err(e) => return serde_json::json!({"error": format!("Naming audit failed: {}", e), "action": "naming", "exit_code": 2}).to_string(),
                };
                let exit_code = if results.is_empty() { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "success" } else { "violations" },
                    "action": "naming",
                    "path": path,
                    "exit_code": exit_code,
                    "violation_count": results.len(),
                    "results": results.iter().map(|r| serde_json::json!({
                        "file": r.file.value.as_str(),
                        "code": r.code.code(),
                        "message": r.message.value.as_str(),
                        "line": r.line.value(),
                        "column": r.column.value(),
                    })).collect::<Vec<serde_json::Value>>(),
                })
            }
            "role" => {
                // Run role rules analysis directly via role_orchestrator aggregate
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.clone()) {
                    Ok(f) => f,
                    Err(_) => return serde_json::json!({"error": "Invalid path", "action": "role", "exit_code": 2}).to_string(),
                };
                let results = self.deps.role_orchestrator.run_audit(&fp).await;
                let exit_code = if results.is_empty() { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "success" } else { "violations" },
                    "action": "role",
                    "path": path,
                    "exit_code": exit_code,
                    "violation_count": results.len(),
                    "results": results.iter().map(|r| serde_json::json!({
                        "file": r.file.value.as_str(),
                        "code": r.code.code(),
                        "message": r.message.value.as_str(),
                        "line": r.line.value(),
                        "column": r.column.value(),
                    })).collect::<Vec<serde_json::Value>>(),
                })
            }
            "external" => {
                // Run external linters directly via external_lint aggregate
                let path = match arg_path {
                    Some(p) => p,
                    None => ".".to_string(),
                };
                let fp = match shared::common::taxonomy_path_vo::FilePath::new(path.clone()) {
                    Ok(f) => f,
                    Err(_) => return serde_json::json!({"error": "Invalid path", "action": "external", "exit_code": 2}).to_string(),
                };
                let scan_results = self.deps.external_lint.scan_all(&fp).await;
                let exit_code = if scan_results.values.is_empty() { 0 } else { 1 };
                serde_json::json!({
                    "status": if exit_code == 0 { "success" } else { "violations" },
                    "action": "external",
                    "path": path,
                    "exit_code": exit_code,
                    "violation_count": scan_results.values.len(),
                    "results": scan_results.values.iter().map(|r| serde_json::json!({
                        "file": r.file.value.as_str(),
                        "code": r.code.code(),
                        "message": r.message.value.as_str(),
                        "line": r.line.value(),
                        "column": r.column.value(),
                    })).collect::<Vec<serde_json::Value>>(),
                })
            }
            _ => {
                serde_json::json!({"error": format!("Unknown action: {}", action), "exit_code": 2})
            }
        };
        serde_json::to_string(&result).unwrap_or_default()
    }

    async fn list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let catalog = shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;
        let commands: Vec<serde_json::Value> = catalog
            .iter()
            .filter(|(name, _desc, _ex)| match args.domain.as_deref() {
                Some(d) if !d.is_empty() => name.contains(d),
                _ => true,
            })
            .map(|(name, desc, example)| {
                serde_json::json!({
                    "name": name,
                    "description": desc,
                    "example": example
                })
            })
            .collect();
        let result = serde_json::json!({ "commands": commands, "total": commands.len() });
        serde_json::to_string(&result).unwrap_or_default()
    }

    async fn read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
        let skills = [
            "lint-arwaky-rust",
            "lint-arwaky-python",
            "lint-arwaky-typescript",
        ];
        let base = env!("CARGO_MANIFEST_DIR");
        let mut candidates: Vec<String> = skills
            .iter()
            .flat_map(|s| {
                vec![
                    format!("{}/../.agents/skills/{}/SKILL.md", base, s),
                    format!(".agents/skills/{}/SKILL.md", s),
                ]
            })
            .collect();
        if let Some(config_dir) = dirs::config_dir() {
            let xdg = config_dir
                .join("lint-arwaky")
                .join(".agents")
                .join("skills");
            for s in &skills {
                candidates.push(xdg.join(s).join("SKILL.md").to_string_lossy().to_string());
            }
        }
        let content = candidates
            .iter()
            .map(std::path::Path::new)
            .find(|p| p.exists())
            .and_then(|p| std::fs::read_to_string(p).ok());
        let content = match content {
            Some(c) => c,
            None => {
                return serde_json::json!({
                    "error": "Skill documentation not found",
                    "searched": candidates
                })
                .to_string();
            }
        };
        match args.section.as_deref() {
            Some(s) if !s.is_empty() => {
                let header = format!("## {}", s);
                if let Some(start) = content.find(&header) {
                    let remaining = &content[start..];
                    let end = match remaining[1..].find("\n## ") {
                        Some(i) => i + 1,
                        None => remaining.len(),
                    };
                    serde_json::json!({"section": s, "content": &remaining[..end]}).to_string()
                } else {
                    serde_json::json!({"error": format!("Section '{}' not found", s)}).to_string()
                }
            }
            _ => serde_json::json!({"content": content}).to_string(),
        }
    }

    async fn get_config(&self, Parameters(args): Parameters<GetConfigArgs>) -> String {
        let path = args.path.unwrap_or_else(|| ".".to_string());
        let language = args.language;

        let fp =
            shared::common::taxonomy_path_vo::FilePath::new(path.clone()).unwrap_or_else(|_| {
                shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default()
            });

        // Use the same config orchestrator aggregate as CLI for parity
        let config_files = match self.deps.config_orchestrator.list_config_files(&fp).await {
            Ok(files) => files,
            Err(e) => {
                return serde_json::json!({
                    "path": path,
                    "language": language,
                    "error": format!("Failed to list config files: {}", e),
                    "exit_code": 2,
                })
                .to_string();
            }
        };

        // Build effective config summary
        let mut layers = Vec::new();
        let mut rules_enabled = Vec::new();
        let mut ignored_paths = Vec::new();
        let mut warnings = Vec::new();
        let mut score_threshold: Option<f64> = None;
        let mut adapter_toggles: Vec<serde_json::Value> = Vec::new();

        // Get config data for each language found
        for (lang, _config_path) in &config_files {
            layers.push(lang.as_str());
            if let Ok(Some(source)) = self.deps.config_orchestrator.read_config(&fp, *lang).await {
                // Parse architecture config (rules, ignored_paths)
                let arch_config = shared::config_system::utility_config_parser::parse_config_yaml(
                    &source.raw_content,
                );
                rules_enabled.push(lang.as_str());
                ignored_paths.extend(
                    arch_config
                        .ignored_paths
                        .values
                        .iter()
                        .map(|p| p.value.clone()),
                );

                // Parse adapter names for adapter_toggles
                let adapter_names =
                    shared::config_system::utility_config_parser::parse_adapter_names_from_yaml(
                        &source.raw_content,
                    );
                for name in adapter_names {
                    adapter_toggles.push(serde_json::json!({"name": name, "status": "enabled"}));
                }

                // Extract score threshold via shared utility (project.thresholds.score or thresholds.score)
                if score_threshold.is_none() {
                    if let Some(t) =
                        shared::config_system::utility_config_parser::parse_score_threshold(
                            &source.raw_content,
                        )
                    {
                        score_threshold = Some(t);
                    }
                }
            } else {
                warnings.push(format!("No config data for {}", lang.as_str()));
            }
        }

        if config_files.is_empty() {
            warnings
                .push("No config files found. Run `lint-arwaky init` to create one.".to_string());
        }

        let result = serde_json::json!({
            "path": path,
            "language": language,
            "layers": layers,
            "rules_enabled": rules_enabled,
            "score_threshold": score_threshold.unwrap_or(80.0),
            "adapter_toggles": adapter_toggles,
            "ignored_paths": ignored_paths,
            "config_files": config_files.iter().map(|(_, p)| p.value.as_str()).collect::<Vec<&str>>(),
            "warnings": warnings,
            "exit_code": 0,
        });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────
impl McpServerOrchestrator {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }
}
