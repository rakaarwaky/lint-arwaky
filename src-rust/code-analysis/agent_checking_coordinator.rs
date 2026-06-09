// lint_checking_coordinator — Agent-layer orchestration of ALL AES checkers.
// This is the CORRECT architectural location for wiring checkers (Agent layer).

use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::code_analysis::capabilities_bypass_inspector::{
    check_agent_any_bypass, check_bypass_comments,
};
use crate::code_analysis::capabilities_checker_helpers::mk_result;
use crate::code_analysis::capabilities_inheritance_inspector::{
    check_dead_inheritance, check_mandatory_inheritance,
};
use crate::code_analysis::capabilities_layer_inspector::{
    check_agent_role, check_missing_vo, check_single_bottleneck, check_surface_role,
};
use crate::code_analysis::capabilities_unused_import_inspector::check_unused_imports;
use crate::code_analysis::contract_checker_aggregate::ICheckerAggregate;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violationrs_constant::AES012_CIRCULAR_IMPORT;
use crate::source_parsing::taxonomy_path_vo::FilePath;

static GLOBAL_CHECKER: OnceLock<Arc<dyn ICheckerAggregate>> = OnceLock::new();

/// Initialize the global checker container. Must be called before using LintCheckingCoordinator.
pub fn init_global_checker(checker: Arc<dyn ICheckerAggregate>) {
    GLOBAL_CHECKER.set(checker).ok();
}

pub struct LintCheckingCoordinator {
    checker: Arc<dyn ICheckerAggregate>,
}

impl Default for LintCheckingCoordinator {
    fn default() -> Self {
        Self::new()
    }
}

impl LintCheckingCoordinator {
    /// Create a new coordinator. Panics if init_global_checker has not been called.
    pub fn new() -> Self {
        Self {
            checker: GLOBAL_CHECKER.get().cloned()
                .unwrap_or_else(|| unreachable!("init_global_checker must be called before LintCheckingCoordinator::new()")),
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
            check_bypass_comments(file, &c, &mut violations);
            check_unused_imports(file, &c, &mut violations);
            check_dead_inheritance(file, &c, &mut violations);
            check_agent_any_bypass(file, &c, &mut violations);
            check_mandatory_inheritance(file, &c, &mut violations);

            for line in c.lines() {
                let t = line.trim();
                if let Some(imp) = t.strip_prefix("use ") {
                    let target = imp.trim_end_matches(';').trim();
                    if !target.is_empty() {
                        import_edges.push((file.to_string(), target.to_string()));
                        // Resolve crate:: imports to file paths for cycle detection
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
            }
            if matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js") {
                continue;
            }
            let layer = match self.checker.detect_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };
            let def = match self.checker.get_layer_def(&layer) {
                Some(d) => d,
                None => continue,
            };
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            // Layer-dependent inline checks (prefix-based, FRD v1.1)
            check_agent_role(file, &c, &layer, &mut violations);
            check_surface_role(file, &c, &layer, &mut violations);
            check_single_bottleneck(file, &c, &layer, &mut violations);
            check_missing_vo(file, &c, &layer, &mut violations);

            // Layer-rule checks (delegated to layer-rules/)
            self.checker
                .check_surface_imports(file, &c, &layer, &mut violations);
            self.checker
                .check_capability_routing(file, &c, &layer, &mut violations);
            self.checker
                .check_mandatory_imports(file, &def, &mut violations);
            self.checker
                .check_forbidden_imports(file, &layer, &def, &mut violations);
            self.checker
                .check_scope_forbidden_imports(file, config, &mut violations);
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
            rl.push(mk_result(
                "",
                0,
                "AES012",
                Severity::CRITICAL,
                AES012_CIRCULAR_IMPORT,
            ));
        }
        // Inline orphan check: prefix/suffix based per-layer logic with barrel resolution
        let ctx = self.checker.build_orphan_graph_context(files, root_dir);
        let eps = self.checker.identify_orphan_entry_points(files);

        for fp in files {
            if eps.contains(fp)
                || fp.ends_with("mod.rs")
                || fp.ends_with("__init__.py")
                || fp.ends_with("/index.ts")
                || fp.ends_with("/index.js")
            {
                continue;
            }
            let basename = fp.split('/').next_back().unwrap_or("");
            let prefix = basename.split('_').next().unwrap_or("");

            // Taxonomy and Contract are standalone types/interfaces — skip orphan check
            if prefix == "taxonomy" || prefix == "contract" {
                continue;
            }

            // Infra/Cap/Agent: check wiring in container/aggregate/registry files
            if prefix == "infrastructure" || prefix == "capabilities" || prefix == "agent" {
                let stem = basename.replace(".rs", "").replace(".py", "");
                let pascal_stem: String = stem
                    .split('_')
                    .filter(|s| !s.is_empty())
                    .map(|s| {
                        let mut c = s.chars();
                        c.next()
                            .map(|f| f.to_uppercase().to_string() + c.as_str())
                            .unwrap_or_default()
                    })
                    .collect();
                let mut wired = false;
                for cf in files {
                    let cb = cf.split('/').next_back().unwrap_or("");
                    let csuffix = cb.rsplit('_').next().unwrap_or("").replace(".rs", "");
                    if csuffix != "container" && csuffix != "aggregate" && csuffix != "registry" {
                        continue;
                    }
                    if let Ok(c) = std::fs::read_to_string(cf) {
                        if c.contains(&stem)
                            || c.contains(&format!("mod {}", stem))
                            || c.contains(&pascal_stem)
                        {
                            wired = true;
                            break;
                        }
                    }
                }
                if !wired {
                    rl.push(mk_result(
                        fp,
                        0,
                        "AES030",
                        Severity::HIGH,
                        &format!("{} '{}' not wired in container.", prefix, stem),
                    ));
                }
                continue;
            }

            // Surface: reachability check
            if prefix == "surface" {
                let imps = ctx.import_graph.mapping.get(fp);
                if imps.map(std::vec::Vec::is_empty).unwrap_or(true) {
                    rl.push(mk_result(
                        fp,
                        0,
                        "AES030",
                        Severity::MEDIUM,
                        "Surface unreachable.",
                    ));
                }
                continue;
            }
        }
        // Wire role orchestrator for agent and surface role checks
        let role_orch = crate::role_rules::agent_role_orchestrator::RoleOrchestrator::new(
            Box::new(crate::role_rules::agent_role_mixin::RoleAggregateImpl::new()),
        );
        role_orch.run_all_role_checks(files, &mut rl.values);

        rl.values
    }
}
