// PURPOSE: LintCheckingOrchestrator — orchestrates ALL AES checkers by calling protocols directly via CheckerContainer

use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::IBypassCheckerProtocol;
use crate::IMandatoryClassProtocol;
use crate::IDeadInheritanceProtocol;
use crate::IInlineUnusedProtocol;
use crate::ILayerDetectionAggregate;
use crate::ILineCheckerProtocol;
use crate::IMandatoryInheritanceProtocol;
use crate::ArchitectureConfig;
use crate::CheckerContainer;
use crate::LintResult;
use crate::LintResultList;
use crate::IRoleAggregate;
use crate::RoleOrchestrator;
use shared::taxonomy_source_vo::{ContentString, SourceContentVO};
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;

static GLOBAL_CONTAINER: OnceLock<Arc<CheckerContainer>> = OnceLock::new();

/// Initialize the global checker container. Must be called before using LintCheckingOrchestrator.
pub fn init_global_checker(container: Arc<CheckerContainer>) {
    GLOBAL_CONTAINER.set(container).ok();
}

pub struct LintCheckingOrchestrator {
    container: Arc<CheckerContainer>,
}

impl Default for LintCheckingOrchestrator {
    fn default() -> Self {
        Self::new()
    }
}

impl LintCheckingOrchestrator {
    /// Create a new orchestrator. Panics if init_global_checker has not been called.
    pub fn new() -> Self {
        Self {
            container: GLOBAL_CONTAINER.get().cloned().unwrap_or_else(|| {
                unreachable!(
                    "init_global_checker must be called before LintCheckingOrchestrator::new()"
                )
            }),
        }
    }

    pub async fn run_all_checks(
        &self,
        config: &ArchitectureConfig,
        files: &[String],
        root_dir: &str,
    ) -> Vec<LintResult> {
        if !config.enabled.value {
            return Vec::new();
        }
        let mut violations: Vec<LintResult> = Vec::new();
        let mut file_paths: Vec<FilePath> = Vec::new();
        let mut import_edges: Vec<(String, String)> = Vec::new();

        for file in files {
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            if let Ok(fp) = FilePath::new(file.to_string()) {
                file_paths.push(fp);
            }
            let c = std::fs::read_to_string(file).unwrap_or_default();

            // Layer-independent checks (run on ALL files) — call protocols directly
            self.container
                .bypass_checker()
                .check_bypass_comments(file, &c, &mut violations);
            self.container
                .inline_unused_checker()
                .check_unused_imports(file, &c, &mut violations);
            self.container
                .dead_inheritance_checker()
                .check_dead_inheritance(file, &c, &mut violations);

            for line in c.lines() {
                let t = line.trim();
                // Rust: `use crate::module::item;`
                if let Some(imp) = t.strip_prefix("use ") {
                    let target = imp.trim_end_matches(';').trim();
                    if !target.is_empty() {
                        import_edges.push((file.to_string(), target.to_string()));
                        if target.starts_with("crate::") {
                            let path_part = target.trim_start_matches("crate::");
                            if let Some(first_break) = path_part.find("::") {
                                let module_path = &path_part[..first_break];
                                let item = path_part[first_break + 2..]
                                    .split("::")
                                    .next()
                                    .unwrap_or("");
                                let candidate = format!(
                                    "{}/{}/{}.rs",
                                    root_dir.trim_end_matches('/'),
                                    module_path,
                                    item
                                );
                                if std::path::Path::new(&candidate).exists() {
                                    import_edges.push((file.to_string(), candidate));
                                }
                            }
                        }
                    }
                }
                // Python: `from module import name` or `import module`
                if t.starts_with("from ") && t.contains(" import ") {
                    let module = t
                        .strip_prefix("from ")
                        .unwrap_or("")
                        .split_whitespace()
                        .next()
                        .unwrap_or("");
                    if !module.is_empty() {
                        import_edges.push((file.to_string(), module.to_string()));
                        // Resolve relative Python imports to file paths
                        if module.starts_with('.') {
                            let parent = std::path::Path::new(file)
                                .parent()
                                .unwrap_or(std::path::Path::new("."));
                            let module_file = module.trim_start_matches('.').replace('.', "/");
                            for ext in &[".py", "/__init__.py"] {
                                let candidate = parent.join(format!("{}{}", module_file, ext));
                                if candidate.exists() {
                                    import_edges.push((
                                        file.to_string(),
                                        candidate.to_string_lossy().to_string(),
                                    ));
                                }
                            }
                        }
                    }
                } else if t.starts_with("import ") && !t.contains(" from ") {
                    let module = t
                        .strip_prefix("import ")
                        .unwrap_or("")
                        .split_whitespace()
                        .next()
                        .unwrap_or("")
                        .trim_end_matches(';');
                    if !module.is_empty() {
                        import_edges.push((file.to_string(), module.to_string()));
                    }
                }
                // JS/TS: `import { name } from "module"` or `import name from "module"`
                if t.starts_with("import ") && t.contains(" from ") {
                    if let Some(from_part) = t.split(" from ").last() {
                        let module = from_part
                            .trim()
                            .trim_matches('"')
                            .trim_matches('\'')
                            .trim_end_matches(';');
                        if !module.is_empty() {
                            import_edges.push((file.to_string(), module.to_string()));
                            // Resolve relative JS imports to file paths
                            if module.starts_with('.') {
                                let parent = std::path::Path::new(file)
                                    .parent()
                                    .unwrap_or(std::path::Path::new("."));
                                let module_path = module.trim_start_matches('.');
                                for ext in &[".ts", ".tsx", ".js", ".jsx", "/index.ts", "/index.js"]
                                {
                                    let candidate = parent.join(format!(
                                        "{}{}",
                                        module_path.trim_start_matches('/'),
                                        ext
                                    ));
                                    if candidate.exists() {
                                        import_edges.push((
                                            file.to_string(),
                                            candidate.to_string_lossy().to_string(),
                                        ));
                                    }
                                }
                            }
                        }
                    }
                }
            }
            if matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js") {
                continue;
            }

            // Layer detection via container (ILayerDetectionAggregate)
            let layer = match self.container.detect_layer(file, root_dir) {
                Some(l) => l,
                None => {
                    continue;
                }
            };
            let def = match self.container.get_layer_def(&layer) {
                Some(d) => d,
                None => continue,
            };
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            // Layer-dependent inline checks — call protocols directly
            // check_mandatory_inheritance disabled per user request (primitif info cukup di contract)
            // self.container
            //     .mandatory_inheritance_checker()
            //     .check_mandatory_inheritance(file, &c, &layer, config, &mut violations);
            self.container
                .mandatory_inheritance_checker()
                .check_contract_implementation(file, &c, files, &mut violations);

            let fp = FilePath::new(file.to_string()).unwrap_or_default();
            let content_vo = ContentString::new(c.clone());
            let language = if file.ends_with(".rs") {
                "rust"
            } else if file.ends_with(".py") {
                "python"
            } else if file.ends_with(".js")
                || file.ends_with(".ts")
                || file.ends_with(".jsx")
                || file.ends_with(".tsx")
            {
                "javascript"
            } else {
                "unknown"
            };
            let source_vo = SourceContentVO::new(fp, content_vo, language);

            // Layer-rule checks — call protocols directly
            self.container
                .capabilities_role_checker()
                .check_capability_routing(&source_vo, &layer, &mut violations);
            self.container
                .line_checker()
                .check_line_counts(file, Some(&def), &mut violations);

            // Taxonomy & contract role checks — call protocols directly
            self.container
                .taxonomy_checker()
                .check_entity(&source_vo, &mut violations);
            self.container
                .taxonomy_checker()
                .check_error(&source_vo, &mut violations);
            self.container
                .taxonomy_checker()
                .check_event(&source_vo, &mut violations);
            self.container
                .taxonomy_checker()
                .check_constant(&source_vo, &mut violations);
            self.container
                .contract_checker()
                .check_aggregate(&source_vo, &def, &mut violations);
            self.container
                .class_checker()
                .check_mandatory_class_definition(file, Some(&def), &mut violations);
        }

        let mut rl = LintResultList::new(violations);
        let root_fp = FilePath::new(root_dir.to_string()).unwrap_or_default();
        let file_paths_vo: Vec<FilePath> = files
            .iter()
            .map(|f| FilePath::new(f.to_string()).unwrap_or_default())
            .collect();
        let files_list_vo = FilePathList::new(file_paths_vo.clone());

        // Naming checks
        self.container
            .naming_checker()
            .check_file_naming(self.container.analyzer(), &files_list_vo, &root_fp, &mut rl)
            .await;
        self.container
            .naming_checker()
            .check_domain_suffixes(self.container.analyzer(), &files_list_vo, &root_fp, &mut rl)
            .await;

        // Import checks
        self.container
            .import_mandatory_checker()
            .check_mandatory_imports(self.container.analyzer(), &files_list_vo, &root_fp, &mut rl)
            .await;
        self.container
            .import_intent_checker()
            .check_mandatory_imports(self.container.analyzer(), &files_list_vo, &root_fp, &mut rl)
            .await;
        self.container
            .import_forbidden_checker()
            .check_forbidden_imports(self.container.analyzer(), &files_list_vo, &root_fp, &mut rl)
            .await;
        self.container
            .import_forbidden_checker()
            .check_legacy_import_rules(self.container.analyzer(), &files_list_vo, &root_fp, &mut rl)
            .await;

        // Cycle detection
        self.container
            .cycle_analyzer()
            .check_cycles(self.container.analyzer(), &files_list_vo, &root_fp, &mut rl)
            .await;

        // Surface hierarchy check — call protocol directly
        self.container
            .surface_checker()
            .check_surface_hierarchy(&file_paths_vo, &root_fp, &mut rl);

        // Orphan check (AES030) - enabled in config
        let orphan_agg = self.container.orphan_aggregate();
        let mut orphan_results = orphan_agg.check_orphans(self.container.as_ref(), files, root_dir);
        rl.values.append(&mut orphan_results);

        // Wire role orchestrator for agent and surface role checks
        let role_agg: Arc<dyn IRoleAggregate> = Arc::new(()); // Placeholder
        let role_orch = RoleOrchestrator::new(role_agg);
        let max_lines = config
            .rules
            .iter()
            .find(|r| r.name.value == "AES0305")
            .map(|r| r.max_lines.value() as usize)
            .unwrap_or(1000);
        role_orch.run_all_role_checks(files, max_lines, &mut rl.values);

        rl.values
    }
}