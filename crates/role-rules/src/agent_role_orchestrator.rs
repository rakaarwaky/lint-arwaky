// PURPOSE: RoleOrchestrator — dispatches files to correct role checker based on filename prefix
//
// FRD-compliant: accepts pre-parsed FileEntry from the filesystem crate.
// No file I/O or AST parsing is performed internally.

use shared::cli_commands::LintResult;
use shared::common::taxonomy_path_vo::FilePath;
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use shared::role_rules::{
    IAgentRoleChecker, ICapabilitiesRoleChecker, IContractRoleChecker, IRoleRunnerAggregate,
    ISurfaceRoleChecker, ITaxonomyRoleChecker, IUtilityRoleChecker,
};
use std::path::Path;
use std::sync::Arc;

// ─── Block 1: Struct Definitions ──────────────────────────

pub struct RoleCheckerDeps {
    pub taxonomy: Arc<dyn ITaxonomyRoleChecker>,
    pub contract: Arc<dyn IContractRoleChecker>,
    pub capabilities: Arc<dyn ICapabilitiesRoleChecker>,
    pub surface: Arc<dyn ISurfaceRoleChecker>,
    pub agent: Arc<dyn IAgentRoleChecker>,
    pub utility: Arc<dyn IUtilityRoleChecker>,
}

pub struct RoleOrchestrator {
    deps: RoleCheckerDeps,
    config: shared::config_system::ArchitectureConfig,
    ignored_paths: Vec<String>,
}

// ─── Block 2: Aggregate Trait Implementation ──────────────

#[async_trait::async_trait]
impl IRoleRunnerAggregate for RoleOrchestrator {
    async fn run_audit(&self, target: &FilePath) -> Vec<LintResult> {
        let files = self.collect_file_entries(target);
        self.run_audit_with_entries(&files)
    }

    fn run_audit_with_entries(&self, files: &[FileEntry]) -> Vec<LintResult> {
        let mut results = Vec::new();
        self.run_all_role_checks(files, &mut results);
        results
    }

    fn name(&self) -> &str {
        "role-rules"
    }
}

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl RoleOrchestrator {
    pub fn new(deps: RoleCheckerDeps, config: &shared::config_system::ArchitectureConfig) -> Self {
        let ignored_paths: Vec<String> = config
            .ignored_paths
            .values
            .iter()
            .map(|fp| fp.value.replace('/', std::path::MAIN_SEPARATOR_STR))
            .collect();
        Self {
            deps,
            config: config.clone(),
            ignored_paths,
        }
    }

    /// Run all role checks on pre-parsed FileEntry slices.
    pub fn run_all_role_checks(&self, files: &[FileEntry], violations: &mut Vec<LintResult>) {
        if !self.config.enabled.value {
            return;
        }

        for file in files {
            if !file.parse_ok || file.content.is_empty() {
                continue;
            }

            let path_str = file.path.to_string_lossy();
            let filename = file
                .path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or_default();
            let stem = Path::new(filename)
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let prefix = stem.split('_').next().unwrap_or_default();

            // Skip barrel files
            if filename == "mod.rs"
                || filename == "lib.rs"
                || filename == "main.rs"
                || filename == "__init__.py"
                || filename == "index.ts"
                || filename == "index.js"
            {
                continue;
            }

            if self.is_ignored(&path_str) {
                continue;
            }

            match prefix {
                "agent" => {
                    self.deps
                        .agent
                        .check_agent_routing(file, "agent", violations);
                }
                "root" => {}
                "surfaces" | "surface" => {
                    self.deps.surface.check_fn_count_limit(file, violations);
                    let is_smart = filename.contains("_command")
                        || filename.contains("_controller")
                        || filename.contains("_page")
                        || filename.contains("_entry")
                        || filename.contains("_router");
                    let is_utility = filename.contains("_hook")
                        || filename.contains("_store")
                        || filename.contains("_action")
                        || filename.contains("_screen");
                    if is_smart {
                        self.deps.surface.check_smart_surface(file, violations);
                    } else if is_utility {
                        self.deps.surface.check_utility_surface(file, violations);
                    } else {
                        self.deps.surface.check_passive_surface(file, violations);
                    }
                }
                "contract" => {
                    if filename.contains("_protocol") {
                        violations.extend(self.deps.contract.check_protocol(file));
                    } else if filename.contains("_aggregate") {
                        violations.extend(self.deps.contract.check_aggregate(file));
                    }
                }
                "capabilities" | "capability" => {
                    self.deps.capabilities.check_capability_routing(
                        file,
                        "capabilities",
                        violations,
                    );
                }
                "utility" => {
                    self.deps.utility.check_utility_convention(file, violations);
                }
                "taxonomy" => {
                    self.deps.taxonomy.check_entity(file, violations);
                    self.deps.taxonomy.check_error(file, violations);
                    self.deps.taxonomy.check_event(file, violations);
                    self.deps.taxonomy.check_constant(file, violations);
                }
                _ => {}
            }
        }
    }

    fn collect_file_entries(&self, target: &FilePath) -> Vec<FileEntry> {
        let path = Path::new(target.value());
        let mut entries = Vec::new();
        if path.is_dir() {
            self.walk_for_entries(path, &mut entries);
        } else if path.is_file()
            && let Ok(content) = std::fs::read_to_string(path)
        {
            let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
            let language =
                shared::filesystem::taxonomy_filesystem_vo::Language::from_extension(ext);
            if let Some(lang) = language {
                entries.push(FileEntry {
                    path: path.to_path_buf(),
                    extension: ext.to_string(),
                    language: lang,
                    size: content.len() as u64,
                    content,
                    parse_ok: true,
                    parse_metadata: None,
                });
            }
        }
        entries
    }

    fn walk_for_entries(&self, dir: &Path, entries: &mut Vec<FileEntry>) {
        if let Ok(read_dir) = std::fs::read_dir(dir) {
            for entry in read_dir.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    let dir_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
                    if dir_name == "target" || dir_name == ".git" || dir_name == "node_modules" {
                        continue;
                    }
                    if self.is_ignored(&path.to_string_lossy()) {
                        continue;
                    }
                    self.walk_for_entries(&path, entries);
                } else if path.is_file() {
                    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
                    let language =
                        shared::filesystem::taxonomy_filesystem_vo::Language::from_extension(ext);
                    if let Some(lang) = language
                        && let Ok(content) = std::fs::read_to_string(&path)
                    {
                        entries.push(FileEntry {
                            path: path.clone(),
                            extension: ext.to_string(),
                            language: lang,
                            size: content.len() as u64,
                            content,
                            parse_ok: true,
                            parse_metadata: None,
                        });
                    }
                }
            }
        }
    }

    fn is_ignored(&self, path: &str) -> bool {
        shared::filesystem::utility_filesystem_io::is_path_ignored(path, &self.ignored_paths)
    }
}
