// PURPOSE: McpServerOrchestrator — thin wrapper calling dispatcher
// All business logic moved to dispatcher::surface_mcp_action
use rmcp::handler::server::wrapper::Parameters;
use shared::auto_fix::LintFixOrchestratorAggregate;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::config_system::IConfigOrchestratorAggregate;
use shared::git_hooks::GitHooksAggregate;
use shared::maintenance::MaintenanceCommandsAggregate;
use shared::mcp_server::IMcpServerAggregate;
use shared::mcp_server::{ExecuteCommandArgs, GetConfigArgs, ListCommandsArgs, ReadSkillArgs};
use shared::project_setup::SetupManagementAggregate;
use shared::external_lint::IExternalLintAggregate;
use shared::import_rules::IImportRunnerAggregate;
use shared::naming_rules::INamingRunnerAggregate;
use shared::orphan_rules::IOrphanAggregate;
use shared::quality_rules::ICodeAnalysisAggregate;
use shared::role_rules::IRoleRunnerAggregate;
use shared::filesystem::contract_filesystem_aggregate::IFilesystemAggregate;
use std::sync::Arc;

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

    fn to_deps(&self) -> dispatcher::surface_mcp_action::McpDeps {
        dispatcher::surface_mcp_action::McpDeps {
            code_analysis: self.deps.code_analysis_linter.clone(),
            import_orchestrator: self.deps.import_orchestrator.clone(),
            naming_orchestrator: self.deps.naming_orchestrator.clone(),
            role_orchestrator: self.deps.role_orchestrator.clone(),
            orphan_orchestrator: self.deps.orphan_orchestrator.clone(),
            external_lint: self.deps.external_lint.clone(),
            config_orchestrator: self.deps.config_orchestrator.clone(),
            filesystem: self.deps.filesystem.clone(),
            fix_orchestrator: self.deps.fix_orchestrator.clone(),
            maintenance: self.deps.maintenance_orchestrator.clone(),
            git_hooks: self.deps.git_hooks_aggregate.clone(),
            setup: self.deps.setup_orchestrator.clone(),
        }
    }
}

impl IMcpServerAggregate for McpServerOrchestrator {
    fn execute_command(&self, Parameters(args): Parameters<ExecuteCommandArgs>) -> String {
        let action = args.action.clone();
        let arg_path = args.args.as_ref().and_then(|a| a.get("path")).and_then(|v| v.as_str()).map(String::from);
        let arg_threshold = args.args.as_ref().and_then(|a| a.get("threshold")).and_then(|v| v.as_u64());
        let arg_client = args.args.as_ref().and_then(|a| a.get("client")).and_then(|v| v.as_str()).map(String::from);
        let dry_run = args.args.as_ref().and_then(|a| a.get("dry_run")).and_then(|v| v.as_bool()).unwrap_or(false);

        let path = arg_path.unwrap_or_else(|| ".".to_string());
        let deps = self.to_deps();

        let result = match action.as_str() {
            "check" | "scan" => dispatcher::surface_mcp_action::execute_check(&deps, &path),
            "ci" => {
                let threshold = arg_threshold.unwrap_or(80);
                dispatcher::surface_mcp_action::execute_ci(&deps, &path, threshold)
            }
            "fix" => dispatcher::surface_mcp_action::execute_fix(&deps, &path, dry_run),
            "doctor" => dispatcher::surface_mcp_action::execute_doctor(&deps),
            "orphan" => dispatcher::surface_mcp_action::execute_orphan(&deps, &path),
            "security" => dispatcher::surface_mcp_action::execute_security(&deps, &path),
            "quality" => dispatcher::surface_mcp_action::execute_quality(&deps, &path),
            "import" => dispatcher::surface_mcp_action::execute_import(&deps, &path),
            "naming" => dispatcher::surface_mcp_action::execute_naming(&deps, &path),
            "role" => dispatcher::surface_mcp_action::execute_role(&deps, &path),
            "external" => dispatcher::surface_mcp_action::execute_external(&deps, &path),
            "dependencies" => dispatcher::surface_mcp_action::execute_dependencies(&deps, &path),
            "version" => serde_json::json!({"version": env!("CARGO_PKG_VERSION"), "name": "lint-arwaky", "exit_code": 0}),
            "watch" => serde_json::json!({"error": "watch is not supported via MCP", "exit_code": 2}),
            _ => serde_json::json!({"error": format!("Unknown action: {}", action), "exit_code": 2}),
        };
        serde_json::to_string(&result).unwrap_or_default()
    }

    fn health_check(&self) -> String {
        let health = self.deps.maintenance_orchestrator.health_check();
        let adapters: Vec<serde_json::Value> = health.adapters.iter().map(|a| {
            serde_json::json!({"name": a.name, "language": a.language, "status": if a.available { "available" } else { "not_installed" }})
        }).collect();
        let available = adapters.iter().filter(|a| a["status"] == "available").count();
        let result = serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "adapters_available": available,
            "adapters_total": adapters.len(),
            "adapters": adapters,
            "exit_code": 0,
        });
        serde_json::to_string_pretty(&result).unwrap_or_default()
    }

    fn list_commands(&self, Parameters(args): Parameters<ListCommandsArgs>) -> String {
        let catalog = shared::cli_commands::taxonomy_command_catalog_vo::COMMAND_CATALOG;
        let commands: Vec<serde_json::Value> = catalog.iter()
            .filter(|(name, _desc, _ex)| match args.domain.as_deref() {
                Some(d) if !d.is_empty() => name.contains(d),
                _ => true,
            })
            .map(|(name, desc, example)| serde_json::json!({"name": name, "description": desc, "example": example}))
            .collect();
        let result = serde_json::json!({ "commands": commands, "total": commands.len() });
        serde_json::to_string(&result).unwrap_or_default()
    }

    fn read_skill(&self, Parameters(args): Parameters<ReadSkillArgs>) -> String {
        let skills = ["lint-arwaky-rust", "lint-arwaky-python", "lint-arwaky-typescript"];
        let base = env!("CARGO_MANIFEST_DIR");
        let mut candidates: Vec<String> = skills.iter().flat_map(|s| {
            vec![
                format!("{}/../.agents/skills/{}/SKILL.md", base, s),
                format!(".agents/skills/{}/SKILL.md", s),
            ]
        }).collect();
        if let Some(config_dir) = dirs::config_dir() {
            let xdg = config_dir.join("lint-arwaky").join(".agents").join("skills");
            for s in &skills {
                candidates.push(xdg.join(s).join("SKILL.md").to_string_lossy().to_string());
            }
        }
        let content = candidates.iter().map(std::path::Path::new).find(|p| p.exists()).and_then(|p| std::fs::read_to_string(p).ok());
        let content = match content {
            Some(c) => c,
            None => return serde_json::json!({"error": "Skill documentation not found", "searched": candidates}).to_string(),
        };
        match args.section.as_deref() {
            Some(s) if !s.is_empty() => {
                let header = format!("## {}", s);
                if let Some(start) = content.find(&header) {
                    let remaining = &content[start..];
                    let end = match remaining[1..].find("\n## ") { Some(i) => i + 1, None => remaining.len() };
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
        let fp = shared::common::taxonomy_path_vo::FilePath::new(path.clone()).unwrap_or_else(|_| {
            shared::common::taxonomy_path_vo::FilePath::new(".").unwrap_or_default()
        });

        let config_files = match self.deps.config_orchestrator.list_config_files(&fp) {
            Ok(files) => files,
            Err(e) => return serde_json::json!({"path": path, "language": language, "error": format!("Failed to list config files: {}", e), "exit_code": 2}).to_string(),
        };

        let mut layers = Vec::new();
        let mut rules_enabled = Vec::new();
        let mut ignored_paths = Vec::new();
        let mut warnings = Vec::new();
        let mut score_threshold: Option<f64> = None;
        let mut adapter_toggles: Vec<serde_json::Value> = Vec::new();

        for (lang, _config_path) in &config_files {
            layers.push(lang.as_str());
            if let Ok(Some(source)) = self.deps.config_orchestrator.read_config(&fp, *lang) {
                let arch_config = config_system::utility_config_parser::parse_config_yaml(&source.raw_content);
                rules_enabled.push(lang.as_str());
                ignored_paths.extend(arch_config.ignored_paths.values.iter().map(|p| p.value.clone()));
                let adapter_names = config_system::utility_config_parser::parse_adapter_names_from_yaml(&source.raw_content);
                for name in adapter_names {
                    adapter_toggles.push(serde_json::json!({"name": name, "status": "enabled"}));
                }
                if score_threshold.is_none() {
                    if let Some(t) = config_system::utility_config_parser::parse_score_threshold(&source.raw_content) {
                        score_threshold = Some(t);
                    }
                }
            } else {
                warnings.push(format!("No config data for {}", lang.as_str()));
            }
        }

        if config_files.is_empty() {
            warnings.push("No config files found. Run `lint-arwaky init` to create one.".to_string());
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
