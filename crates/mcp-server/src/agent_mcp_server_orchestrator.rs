// PURPOSE: McpServerOrchestrator — agent that implements IMcpServerAggregate
//
// The MCP orchestrator is the AI-agent entry point. It delegates all scan
// operations to aggregates via DI and returns JSON responses.
// All methods are sync — async bridging happens in the surface layer.

use rmcp::handler::server::wrapper::Parameters;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::common::taxonomy_path_vo::FilePath;
use shared::config_system::contract_config_orchestrator_aggregate::IConfigOrchestratorAggregate;
use shared::config_system::IConfigParserProtocol;
use shared::config_system::IConfigReaderProtocol;
use shared::external_lint::contract_external_lint_aggregate::IExternalLintAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::import_rules::contract_import_runner_aggregate::IImportRunnerAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::mcp_server::IMcpServerAggregate;
use shared::mcp_server::{ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs, ReadSkillArgs};
use shared::naming_rules::contract_naming_runner_aggregate::INamingRunnerAggregate;
use shared::orphan_rules::contract_orphan_aggregate::IOrphanAggregate;
use shared::project_setup::SetupManagementAggregate;
use shared::role_rules::contract_role_runner_aggregate::IRoleRunnerAggregate;
use std::sync::Arc;

// ─── Block 1: Struct Definition ───────────────────────────
/// Dependencies injected into the MCP server orchestrator.
/// All aggregates are wired from the MCP container for full CLI parity.
pub struct McpServerDependencies {
    pub code_analysis_linter: Arc<dyn ICodeAnalysisAggregate>,
    pub fix_orchestrator: Arc<dyn LintFixOrchestratorAggregate>,
    pub orphan_orchestrator: Arc<dyn IOrphanAggregate>,
    pub maintenance_orchestrator: Arc<dyn MaintenanceCommandsAggregate>,
    pub git_hooks_aggregate: Arc<dyn GitHooksAggregate>,
    pub setup_orchestrator: Arc<dyn SetupManagementAggregate>,
    pub config_orchestrator: Arc<dyn IConfigOrchestratorAggregate>,
    pub external_lint: Arc<dyn IExternalLintAggregate>,
    pub import_orchestrator: Arc<dyn IImportRunnerAggregate>,
    pub naming_orchestrator: Arc<dyn INamingRunnerAggregate>,
    pub role_orchestrator: Arc<dyn IRoleRunnerAggregate>,
    pub filesystem: Arc<dyn IFilesystemAggregate>,
}

pub struct McpServerOrchestrator {
    deps: McpServerDependencies,
}

impl McpServerOrchestrator {
    pub fn new(deps: McpServerDependencies) -> Self {
        Self { deps }
    }
}

// ─── Block 2: Aggregate Trait Implementation ──────────────
impl IMcpServerAggregate for McpServerOrchestrator {
    fn execute_command(&self, Parameters(args): Parameters<ExecuteCommandArgs>) -> String {
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
            "check" | "scan" => self.cmd_check_scan(arg_path),
            "ci" => self.cmd_ci(arg_path, arg_threshold),
            "fix" => self.cmd_fix(arg_path, &args),
            "doctor" => self.cmd_doctor(),
            "orphan" => self.cmd_orphan(arg_path),
            "security" => self.cmd_security(arg_path),
            "duplicates" => self.cmd_duplicates(arg_path),
            "dependencies" => self.cmd_dependencies(arg_path),
            "version" => {
                serde_json::json!({"version": env!("CARGO_PKG_VERSION"), "name": "lint-arwaky", "exit_code": 0})
            }
            "adapters" => self.cmd_adapters(),
            "install-hook" => self.cmd_install_hook(),
            "uninstall-hook" => self.cmd_uninstall_hook(),
            "init" => self.cmd_init(),
            "install" => self.cmd_install(&args),
            "mcp-config" => self.cmd_mcp_config(arg_client),
            "config-show" => self.cmd_config_show(arg_path),
            "watch" => {
                serde_json::json!({
                    "error": "watch is not supported via MCP (long-lived action — use CLI: lint-arwaky watch <path>)",
                    "action": "watch",
                    "exit_code": 2,
                })
            }
            "quality" => self.cmd_quality(arg_path),
            "import" => self.cmd_import(arg_path),
            "naming" => self.cmd_naming(arg_path),
            "role" => self.cmd_role(arg_path),
            "external" => self.cmd_external(arg_path),
            _ => {
                serde_json::json!({"error": format!("Unknown action: {}", action), "exit_code": 2})
            }
        };
        serde_json::to_string(&result).unwrap_or_default()
    }

    fn list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let commands: Vec<serde_json::Value> = COMMAND_CATALOG
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

    fn read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
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
            .and_then(|p| self.deps.filesystem.read_file(p));
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

    fn get_config(&self, Parameters(args): Parameters<GetConfigArgs>) -> String {
        let path = args.path.unwrap_or_else(|| ".".to_string());
        let language = args.language;

        let fp =
            FilePath::new(path.clone()).unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());

        let config_files = self.deps.config_orchestrator.list_config_files(&fp) {
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

        let mut layers = Vec::new();
        let mut rules_enabled = Vec::new();
        let mut ignored_paths = Vec::new();
        let mut warnings = Vec::new();
        let mut adapter_toggles: Vec<serde_json::Value> = Vec::new();

        for (lang, _config_path) in &config_files {
            layers.push(lang.as_str());
            if let Ok(Some(source)) = self.deps.config_orchestrator.read_config(&fp, *lang) {
                rules_enabled.push(lang.as_str());
                let (arch_config, parse_warnings) = self
                    .deps
                    .config_orchestrator
                    .parse_config_yaml_with_warnings(&source.raw_content);
                ignored_paths.extend(arch_config.ignored_paths.values.iter().cloned());
                warnings.extend(parse_warnings);

                let adapter_entries = self
                    .deps
                    .config_orchestrator
                    .parse_adapter_entries_from_yaml(&source.raw_content);
                for entry in &adapter_entries {
                    adapter_toggles.push(serde_json::json!({
                        "name": entry.name.value(),
                        "status": entry.status.as_str(),
                    }));
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
            "adapter_toggles": adapter_toggles,
            "ignored_paths": ignored_paths,
            "warnings": warnings,
            "exit_code": 0,
        });
        serde_json::to_string(&result).unwrap_or_default()
    }
}

// ─── Block 3: Private action handlers ─────────────────────
impl McpServerOrchestrator {
    // ── check / scan ─────────────────────────────────────────
    fn cmd_check_scan(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp = match FilePath::new(path.clone()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "action": "scan", "exit_code": 2});
            }
        };

        let mut all_results: Vec<serde_json::Value> = Vec::new();

        // 1. Code analysis (quality)
        let quality = self.deps.code_analysis_linter.run_code_analysis_path(&fp);
        for r in &quality {
            all_results.push(serde_json::json!({
                "file": r.file.value.as_str(),
                "code": r.code.code(),
                "message": r.message.value.as_str(),
                "line": r.line.value(),
                "column": r.column.value(),
                "linter": "quality",
            }));
        }

        // 2. Import rules
        if let Ok(import_results) = self.deps.import_orchestrator.run_audit(&fp) {
            for r in &import_results {
                all_results.push(serde_json::json!({
                    "file": r.file.value.as_str(),
                    "code": r.code.code(),
                    "message": r.message.value.as_str(),
                    "line": r.line.value(),
                    "column": r.column.value(),
                    "linter": "import",
                }));
            }
        }

        // 3. Naming rules
        if let Ok(naming_results) = self.deps.naming_orchestrator.run_audit(&fp) {
            for r in &naming_results {
                all_results.push(serde_json::json!({
                    "file": r.file.value.as_str(),
                    "code": r.code.code(),
                    "message": r.message.value.as_str(),
                    "line": r.line.value(),
                    "column": r.column.value(),
                    "linter": "naming",
                }));
            }
        }

        // 4. Orphan detector
        let ignored = self.deps.config_orchestrator.ignored_paths(&fp);
        let (_, orphan_results) = self
            .deps
            .orphan_orchestrator
            .scan_orphans(&fp, &ignored.values);
        for r in &orphan_results {
            all_results.push(serde_json::json!({
                "file": r.file.value.as_str(),
                "code": r.code.code(),
                "message": r.message.value.as_str(),
                "line": r.line.value(),
                "column": r.column.value(),
                "linter": "orphan",
            }));
        }

        // 5. External linters (blocking bridge)
        let scan_results = self.deps.external_lint.scan_all(&fp);
        for r in &scan_results.values {
            all_results.push(serde_json::json!({
                "file": r.file.value.as_str(),
                "code": r.code.code(),
                "message": r.message.value.as_str(),
                "line": r.line.value(),
                "column": r.column.value(),
                "linter": "external",
            }));
        }

        // 6. Parse warnings from filesystem
        let scan_root = std::path::PathBuf::from(&path);
        let ignored_strs: Vec<String> = ignored.values.iter().cloned().collect();
        self.deps
            .filesystem
            .discover_source_files(&scan_root, &ignored_strs);
        let parse_warnings: Vec<serde_json::Value> = self
            .deps
            .filesystem
            .file_list()
            .iter()
            .filter(|e| !e.parse_ok)
            .map(|e| {
                serde_json::json!({
                    "file": e.path.to_string_lossy(),
                    "message": "parse error",
                })
            })
            .collect();

        let total = all_results.len();
        let exit_code = if total == 0 { 0 } else { 1 };
        serde_json::json!({
            "status": if exit_code == 0 { "success" } else { "failure" },
            "action": "scan",
            "path": path,
            "exit_code": exit_code,
            "total_violations": total,
            "results": all_results,
            "parse_warnings": parse_warnings,
        })
    }

    // ── ci ───────────────────────────────────────────────────
    fn cmd_ci(&self, arg_path: Option<String>, arg_threshold: Option<u32>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let _threshold = arg_threshold.unwrap_or(80);
        let fp = match FilePath::new(path.clone()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "action": "ci", "exit_code": 2});
            }
        };

        let quality = self.deps.code_analysis_linter.run_code_analysis_path(&fp);
        let total = quality.len();
        let exit_code = if total == 0 { 0 } else { 1 };
        serde_json::json!({
            "status": if exit_code == 0 { "pass" } else { "fail" },
            "action": "ci",
            "path": path,
            "exit_code": exit_code,
            "total_violations": total,
        })
    }

    // ── fix ──────────────────────────────────────────────────
    fn cmd_fix(&self, arg_path: Option<String>, args: &ExecuteCommandArgs) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let dry_run = args
            .args
            .as_ref()
            .and_then(|a| a.get("dry_run"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let fp =
            FilePath::new(path.clone()).unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());

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

    // ── doctor ───────────────────────────────────────────────
    fn cmd_doctor(&self) -> serde_json::Value {
        let diag =
        self.deps.maintenance_orchestrator.diagnose_toolchain();

        let checks: Vec<serde_json::Value> = diag
            .rust_tools
            .iter()
            .chain(diag.python_tools.iter())
            .chain(diag.js_tools.iter())
            .chain(diag.vcs_tools.iter())
            .map(|status| {
                serde_json::json!({
                    "tool": status.name,
                    "status": if status.status == "OK" { "ok" } else { "not_found" },
                    "version": status.version,
                })
            })
            .collect();

        serde_json::json!({"status": "success", "action": "doctor", "exit_code": 0, "checks": checks})
    }

    // ── orphan ───────────────────────────────────────────────
    fn cmd_orphan(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp =
            FilePath::new(path.clone()).unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());

        let ignored = self.deps.config_orchestrator.ignored_paths(&fp);
        let (_, results) = self
            .deps
            .orphan_orchestrator
            .scan_orphans(&fp, &ignored.values);

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

    // ── security ─────────────────────────────────────────────
    fn cmd_security(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp =
            FilePath::new(path.clone()).unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());

        let report =
        self.deps.maintenance_orchestrator.run_security_scan(&fp);

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

    // ── duplicates ───────────────────────────────────────────
    fn cmd_duplicates(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp =
            FilePath::new(path.clone()).unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());

        let entries = self
            .deps
            .code_analysis_linter
            .collect_file_entries(&[fp.value().to_string()]);
        let min_dup_lines = 5;
        let total_loc: usize = entries.iter().map(|(_, c)| c.lines().count()).sum();
        let blocks = self
            .deps
            .code_analysis_linter
            .scan_duplicate_blocks(entries, min_dup_lines);
        let violations =
        self.deps
                .code_analysis_linter
                .build_violations(&blocks, total_loc, min_dup_lines);

        serde_json::json!({
            "status": "success",
            "action": "duplicates",
            "path": path,
            "exit_code": 0,
            "duplicate_blocks": blocks.len(),
            "violations": violations.len(),
        })
    }

    // ── dependencies ─────────────────────────────────────────
    fn cmd_dependencies(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp =
            FilePath::new(path.clone()).unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());

        self.deps
                .maintenance_orchestrator
                .run_dependency_report(&fp),
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

    // ── adapters ─────────────────────────────────────────────
    fn cmd_adapters(&self) -> serde_json::Value {
        let adapter_names = self.deps.external_lint.adapter_names();
        let adapters: Vec<serde_json::Value> = adapter_names
            .values
            .iter()
            .map(|name| {
                let found = std::process::Command::new("which")
                    .arg(&name.value)
                    .output()
                    .map(|o| o.status.success())
                    .unwrap_or(false);
                serde_json::json!({"name": name.value, "enabled": found})
            })
            .collect();
        serde_json::json!({"adapters": adapters, "exit_code": 0})
    }

    // ── install-hook ─────────────────────────────────────────
    fn cmd_install_hook(&self) -> serde_json::Value {
        let exe_path = std::env::current_exe()
            .ok()
            .and_then(|p| p.canonicalize().ok())
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| "./lint-arwaky".to_string());

        let fp = FilePath::new(exe_path).unwrap_or_default();

        self.deps.git_hooks_aggregate.install_hook_sync(&fp) {
            Ok(_) => {
                serde_json::json!({"status": "success", "message": "Git hook installed.", "action": "install-hook", "exit_code": 0})
            }
            Err(e) => {
                serde_json::json!({"error": format!("Failed to install hook: {:?}", e), "action": "install-hook", "exit_code": 2})
            }
        }
    }

    // ── uninstall-hook ───────────────────────────────────────
    fn cmd_uninstall_hook(&self) -> serde_json::Value {
        self.deps.git_hooks_aggregate.uninstall_hook_sync() {
            Ok(_) => {
                serde_json::json!({"status": "success", "message": "Git hook removed.", "action": "uninstall-hook", "exit_code": 0})
            }
            Err(e) => {
                serde_json::json!({"error": format!("Failed to uninstall hook: {:?}", e), "action": "uninstall-hook", "exit_code": 2})
            }
        }
    }

    // ── init ─────────────────────────────────────────────────
    fn cmd_init(&self) -> serde_json::Value {
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

    // ── install ──────────────────────────────────────────────
    fn cmd_install(&self, args: &ExecuteCommandArgs) -> serde_json::Value {
        let sudo = args
            .args
            .as_ref()
            .and_then(|a| a.get("sudo"))
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let py_status =
            self.deps.setup_orchestrator.install_python_adapters();
        let js_status = self
            .deps
            .setup_orchestrator
            .install_javascript_adapters(sudo);

        serde_json::json!({
            "status": if py_status.value && js_status.value { "success" } else { "partial_failure" },
            "action": "install",
            "exit_code": if py_status.value && js_status.value { 0 } else { 1 },
            "python_adapters_installed": py_status.value,
            "javascript_adapters_installed": js_status.value,
        })
    }

    // ── mcp-config ───────────────────────────────────────────
    fn cmd_mcp_config(&self, arg_client: Option<String>) -> serde_json::Value {
        let client = arg_client.unwrap_or_else(|| "all".to_string());

        let binary = match std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.join("lint-arwaky-mcp")))
            .and_then(|p| p.canonicalize().ok())
        {
            Some(path) => path.to_string_lossy().into_owned(),
            None => "lint-arwaky-mcp".to_string(),
        };

        let config = match client.as_str() {
            "claude-code" | "claude" | "cursor" | "hermes" | "vscode" => serde_json::json!({
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

    // ── config-show ──────────────────────────────────────────
    fn cmd_config_show(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp =
            FilePath::new(path.clone()).unwrap_or_else(|_| FilePath::new(".").unwrap_or_default());

        self.deps.config_orchestrator.list_config_files(&fp) {
            Ok(config_files) if !config_files.is_empty() => {
                let mut configs = Vec::new();
                for (lang, config_path) in &config_files {
                    self.deps.config_orchestrator.read_config(&fp, *lang) {
                        Ok(Some(source)) => {
                            let safe_content = redact_secrets(&source.raw_content);
                            configs.push(serde_json::json!({
                                "language": lang.as_str(),
                                "path": config_path.value.as_str(),
                                "content": safe_content,
                            }));
                        }
                        _ => {
                            configs.push(serde_json::json!({
                                "language": lang.as_str(),
                                "path": config_path.value.as_str(),
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

    // ── quality ──────────────────────────────────────────────
    fn cmd_quality(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp = match FilePath::new(path.clone()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "action": "quality", "exit_code": 2});
            }
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

    // ── import ───────────────────────────────────────────────
    fn cmd_import(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp = match FilePath::new(path.clone()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "action": "import", "exit_code": 2});
            }
        };
        let results = self.deps.import_orchestrator.run_audit(&fp) {
            Ok(r) => r,
            Err(e) => {
                return serde_json::json!({"error": format!("Import audit failed: {}", e), "action": "import", "exit_code": 2});
            }
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

    // ── naming ───────────────────────────────────────────────
    fn cmd_naming(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp = match FilePath::new(path.clone()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "action": "naming", "exit_code": 2});
            }
        };
        let results = self.deps.naming_orchestrator.run_audit(&fp) {
            Ok(r) => r,
            Err(e) => {
                return serde_json::json!({"error": format!("Naming audit failed: {}", e), "action": "naming", "exit_code": 2});
            }
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

    // ── role ─────────────────────────────────────────────────
    fn cmd_role(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp = match FilePath::new(path.clone()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "action": "role", "exit_code": 2});
            }
        };

        // Role checker requires FileEntry objects from the filesystem pipeline
        let source_files = self
            .deps
            .filesystem
            .discover_source_files(std::path::Path::new(fp.value()), &[]);
        let entries: Vec<shared::filesystem::taxonomy_filesystem_vo::FileEntry> = source_files
            .iter()
            .filter_map(|s| {
                let path = std::path::PathBuf::from(s);
                let extension = path.extension()?.to_str()?.to_string();
                Some(shared::filesystem::taxonomy_filesystem_vo::FileEntry {
                    path,
                    extension,
                    ..Default::default()
                })
            })
            .collect();
        let results = self.deps.role_orchestrator.run_audit_with_entries(&entries);
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

    // ── external ─────────────────────────────────────────────
    fn cmd_external(&self, arg_path: Option<String>) -> serde_json::Value {
        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let fp = match FilePath::new(path.clone()) {
            Ok(f) => f,
            Err(_) => {
                return serde_json::json!({"error": "Invalid path", "action": "external", "exit_code": 2});
            }
        };
        let scan_results = self.deps.external_lint.scan_all(&fp);
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
}

// ─── Helper functions ──────────────────────────────────────

/// Simple secret redaction for config display.
fn redact_secrets(content: &str) -> String {
    static AWS_KEY_RE: once_cell::sync::Lazy<Option<regex::Regex>> =
        once_cell::sync::Lazy::new(|| regex::Regex::new(r"AKIA[0-9A-Z]{16}").ok());

    let mut result = content.to_string();
    if result.contains("AKIA") {
        if let Some(re) = AWS_KEY_RE.as_ref() {
            result = re.replace_all(&result, "[REDACTED-AWS-KEY]").to_string();
        }
    }
    // Redact very long base64-like strings
    if result.len() > 100 {
        let words: Vec<String> = result.split_whitespace().map(|s| s.to_string()).collect();
        for word in &words {
            if word.len() >= 40
                && word
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || matches!(c, '/' | '+' | '='))
            {
                result = result.replacen(word, "[REDACTED]", 1);
            }
        }
    }
    result
}
