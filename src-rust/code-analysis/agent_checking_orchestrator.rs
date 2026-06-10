// PURPOSE: LintCheckingOrchestrator — orchestrates ALL AES checkers via ICheckerAggregate contract

use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::code_analysis::contract_checker_aggregate::ICheckerAggregate;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violation_message_rs_error::AesViolation;
use crate::source_parsing::taxonomy_path_vo::FilePath;

static GLOBAL_CHECKER: OnceLock<Arc<dyn ICheckerAggregate>> = OnceLock::new();

/// Initialize the global checker container. Must be called before using LintCheckingOrchestrator.
pub fn init_global_checker(checker: Arc<dyn ICheckerAggregate>) {
    GLOBAL_CHECKER.set(checker).ok();
}

pub struct LintCheckingOrchestrator {
    checker: Arc<dyn ICheckerAggregate>,
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
            checker: GLOBAL_CHECKER.get().cloned().unwrap_or_else(|| {
                unreachable!(
                    "init_global_checker must be called before LintCheckingOrchestrator::new()"
                )
            }),
        }
    }

    pub fn run_all_checks(
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

            // Layer-independent checks (run on ALL files)
            self.checker
                .check_bypass_comments(file, &c, &mut violations);
            self.checker
                .check_inline_unused_imports(file, &c, &mut violations);
            self.checker
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
            let layer = match self.checker.detect_layer(file, root_dir) {
                Some(l) => l,
                None => {
                    continue;
                }
            };
            let def = match self.checker.get_layer_def(&layer) {
                Some(d) => d,
                None => continue,
            };
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            // Layer-dependent inline checks (prefix-based, FRD v1.1)
            self.checker
                .check_single_bottleneck(file, &c, &layer, &mut violations);
            self.checker
                .check_missing_vo(file, &c, &layer, &mut violations);
            self.checker
                .check_mandatory_inheritance(file, &c, &layer, config, &mut violations);

            // Layer-rule checks (delegated to layer-rules/)
            self.checker
                .check_capability_routing(file, &c, &layer, &mut violations);
            self.checker
                .check_mandatory_imports(file, &def, &mut violations);
            self.checker
                .check_forbidden_imports(file, &layer, &def, &mut violations);
            self.checker
                .check_scope_forbidden_imports(file, config, &mut violations);
            self.checker
                .check_scope_mandatory_imports(file, config, &mut violations);
            self.checker
                .check_legacy_import_rules(file, &layer, config, &mut violations);
            self.checker
                .check_line_counts(file, Some(&def), &mut violations);

            self.checker.check_entity(file, &c, &mut violations);
            self.checker.check_error(file, &c, &mut violations);
            self.checker.check_event(file, &c, &mut violations);
            self.checker.check_constant(file, &mut violations);
            self.checker
                .check_aggregate(file, &c, &def, &mut violations);
            self.checker
                .check_mandatory_class_definition(file, Some(&def), &mut violations);
            self.checker.check_file_naming(
                file,
                filename,
                &Some(layer.clone()),
                Some(&def),
                config,
                &mut violations,
            );
            self.checker.check_domain_suffixes(
                file,
                filename,
                Some(&def),
                &Some(layer.clone()),
                &mut violations,
            );
        }

        let mut rl = LintResultList::new(violations);
        let root_fp = FilePath::new(root_dir.to_string()).unwrap_or_default();
        self.checker
            .check_surface_hierarchy(&file_paths, &root_fp, &mut rl);
        let ce: Vec<(String, String)> = import_edges
            .iter()
            .map(|(s, t)| (s.clone(), t.clone()))
            .collect();
        if self.checker.detect_cycle_edges(&ce) {
            rl.push(LintResult::new_arch(
                "",
                0,
                "AES012",
                Severity::CRITICAL,
                AesViolation::CircularImport,
            ));
        }
        // Orphan check: delegated via IOrphanAggregate
        let orphan_agg = self.checker.orphan_aggregate();
        let mut orphan_results = orphan_agg.check_orphans(self.checker.as_ref(), files, root_dir);
        rl.values.append(&mut orphan_results);
        // Wire role orchestrator for agent and surface role checks
        let role_orch = crate::role_rules::agent_role_orchestrator::RoleOrchestrator::new(
            Box::new(crate::role_rules::agent_role_container::RoleAggregateImpl::new()),
        );
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
