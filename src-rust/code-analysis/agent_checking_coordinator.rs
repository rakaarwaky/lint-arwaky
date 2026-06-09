// lint_checking_coordinator — Agent-layer orchestration of ALL AES checkers.
// This is the CORRECT architectural location for wiring checkers (Agent layer).

use std::path::Path;

use crate::code_analysis::capabilities_class_checker::ArchClassChecker;
use crate::code_analysis::capabilities_line_checker::ArchLineChecker;
use crate::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use crate::code_analysis::contract_line_protocol::ILineCheckerProtocol;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::di_containers::contract_service_aggregate::ServiceContainerAggregate;
use crate::layer_rules::capabilities_compliance_analyzer::ArchComplianceAnalyzer;
use crate::layer_rules::capabilities_cycle_analyzer::detect_cycle_edges;
use crate::layer_rules::capabilities_cycle_analyzer::DependencyEdge;
use crate::layer_rules::capabilities_hierarchy_checker::SurfaceHierarchyChecker;
use crate::layer_rules::capabilities_import_checker::ArchImportRuleChecker;
use crate::layer_rules::capabilities_internal_checker::ArchInternalChecker;
use crate::layer_rules::capabilities_layer_checker::ArchLayerChecker;
use crate::naming_rules::capabilities_naming_checker::ArchNamingChecker;
use crate::orphan_detector::capabilities_orphan_analyzer::OrphanGraphResolver;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::role_rules::capabilities_contractrole_checker::ContractRoleChecker;
use crate::role_rules::capabilities_taxonomyrole_checker::TaxonomyRoleChecker;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::shared_common::taxonomy_violationrs_constant::{
    aes015_unused_import, aes016_dead_inheritance, aes027_mandatory_inheritance,
    AES014_BYPASS_COMMENT, AES014_PANIC, AES014_UNWRAP_EXPECT, AES017_ORPHAN_CODE,
    AES020_CIRCULAR_IMPORT, AES022_SURFACE_ROLE_VIOLATION, AES025_MCP_SCHEMA,
    AES031_SINGLE_BOTTLENECK, AES032_MISSING_VO,
};
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct LintCheckingCoordinator {}

impl LintCheckingCoordinator {
    pub fn new() -> Self {
        Self {}
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
        let _container_ref: Option<&dyn ServiceContainerAggregate> = None;
        let analyzer = ArchComplianceAnalyzer::new(config.clone());
        let mut violations: Vec<LintResult> = Vec::new();
        let import_checker = ArchImportRuleChecker::new();
        let line_checker = ArchLineChecker::new();
        let class_checker = ArchClassChecker::new();
        let taxonomy_checker = TaxonomyRoleChecker::new();
        let contract_checker = ContractRoleChecker::new();
        let naming_checker = ArchNamingChecker::new();
        let internal_checker = ArchInternalChecker::new();
        let layer_checker = ArchLayerChecker::new();
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
            Self::check_bypass_comments(file, &c, &mut violations);
            Self::check_unused_imports(file, &c, &mut violations);
            Self::check_dead_inheritance(file, &c, &mut violations);
            Self::check_agent_any_bypass(file, &c, &mut violations);
            Self::check_mcp_schema(file, &c, &mut violations);
            Self::check_mandatory_inheritance(file, &c, &mut violations);

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
                // AES012: barrel completeness check on barrel files
                let b_layer = analyzer.detect_layer(file, root_dir);
                let b_def = b_layer.as_ref().and_then(|l| analyzer.get_layer_def(l));
                if let Some(bd) = b_def {
                    internal_checker.check_internal_rules(
                        file,
                        filename,
                        Some(bd),
                        &mut violations,
                    );
                }
                continue;
            }
            let layer = match analyzer.detect_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };
            let def = match analyzer.get_layer_def(&layer) {
                Some(d) => d,
                None => continue,
            };
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            // Layer-dependent inline checks (prefix-based, FRD v1.1)
            Self::check_agent_role(file, &c, &layer, &mut violations);
            Self::check_surface_role(file, &c, &layer, &mut violations);
            Self::check_single_bottleneck(file, &c, &layer, &mut violations);
            Self::check_missing_vo(file, &c, &layer, &mut violations);

            // Layer-rule checks (delegated to layer-rules/)
            layer_checker.check_surface_imports(file, &c, &layer, &mut violations);
            layer_checker.check_capability_routing(file, &c, &layer, &mut violations);
            import_checker.check_mandatory_imports(file, def, &mut violations);
            import_checker.check_forbidden_imports(file, &layer, def, &mut violations);
            import_checker.check_legacy_import_rules(file, &layer, config, &mut violations);
            line_checker.check_line_counts(file, Some(def), &mut violations);

            taxonomy_checker.check_entity(file, &c, &mut violations);
            taxonomy_checker.check_error(file, &c, &mut violations);
            taxonomy_checker.check_event(file, &c, &mut violations);
            taxonomy_checker.check_constant(file, &mut violations);
            contract_checker.check_aggregate(file, &c, def, &mut violations);
            class_checker.check_mandatory_class_definition(file, Some(def), &mut violations);
            naming_checker.check_file_naming(
                file,
                filename,
                &Some(layer.clone()),
                Some(def),
                config,
                &mut violations,
            );
            naming_checker.check_domain_suffixes(
                file,
                filename,
                Some(def),
                &Some(layer.clone()),
                &mut violations,
            );
            internal_checker.check_internal_rules(file, filename, Some(def), &mut violations);
        }

        let mut rl = LintResultList::new(violations);
        let root_fp = FilePath::new(root_dir.to_string()).unwrap_or_default();
        SurfaceHierarchyChecker::new().check_surface_hierarchy(&file_paths, &root_fp, &mut rl);
        let ce: Vec<DependencyEdge> = import_edges
            .iter()
            .map(|(s, t)| DependencyEdge::new(s.clone(), t.clone()))
            .collect();
        if !detect_cycle_edges(&ce).is_empty() {
            rl.push(Self::mk(
                "",
                0,
                "AES020",
                Severity::CRITICAL,
                AES020_CIRCULAR_IMPORT,
            ));
        }
        let orphan = OrphanGraphResolver::new();
        let ctx = orphan.build_graph_context(files, root_dir);
        let eps = orphan.identify_entry_points(files);
        for (fp, imps) in &ctx.import_graph.mapping {
            if imps.is_empty()
                && !eps.contains(fp)
                && !fp.ends_with("mod.rs")
                && !fp.ends_with("__init__.py")
                && !fp.ends_with("/index.ts")
                && !fp.ends_with("/index.js")
            {
                rl.push(Self::mk(
                    fp,
                    0,
                    "AES017",
                    Severity::HIGH,
                    AES017_ORPHAN_CODE,
                ));
            }
        }
        // Wire role orchestrator for agent and surface role checks
        let role_orch = crate::role_rules::agent_role_orchestrator::RoleOrchestrator::new(
            Box::new(crate::role_rules::agent_role_mixin::RoleAggregateImpl::new()),
        );
        role_orch.run_all_role_checks(files, &mut rl.values);

        rl.values
    }

    fn mk(file: &str, line: usize, code: &str, sev: Severity, msg: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // INLINE CHECKER METHODS (AES007, 014, 015, 016, 021, 022, 024, 025)
    // ─────────────────────────────────────────────────────────────────────────

    fn check_bypass_comments(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let markers = [
            ("H", "noqa"),
            ("H", "type: ignore"),
            ("H", "pylint: disable"),
            ("S", "eslint-disable"),
            ("A", "ts-ignore"),
            ("A", "ts-expect-error"),
            ("S", "NOLINT"),
        ];
        let mkc = |p, k| match p {
            "H" => format!("#{}", k),
            "S" => format!("//{}", k),
            "A" => format!("//@{}", k),
            _ => String::new(),
        };
        let patterns: Vec<String> = markers.iter().map(|&(p, k)| mkc(p, k)).collect();
        let unwrap_pat = [".", "unwrap()"].concat();
        let expect_pat = [".", "expect("].concat();
        let panic_pat = ["panic", "!("].concat();
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("#[allow(") || t.starts_with("#[expect(") {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES014",
                    Severity::CRITICAL,
                    AES014_BYPASS_COMMENT,
                ));
                continue;
            }
            for p in &patterns {
                if t.to_lowercase().contains(p.as_str()) {
                    violations.push(Self::mk(
                        file,
                        i + 1,
                        "AES014",
                        Severity::CRITICAL,
                        AES014_BYPASS_COMMENT,
                    ));
                    break;
                }
            }
            if t.contains(&unwrap_pat) || t.contains(&expect_pat) {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES014",
                    Severity::CRITICAL,
                    AES014_UNWRAP_EXPECT,
                ));
                continue;
            }
            if t.contains(&panic_pat) {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES014",
                    Severity::CRITICAL,
                    AES014_PANIC,
                ));
                continue;
            }
        }
    }

    fn check_unused_imports(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();

            let names: Vec<String> = if t.starts_with("use ") {
                let target = t.trim_end_matches(';').trim_start_matches("use ").trim();
                if target.starts_with("std::")
                    || target.starts_with("core::")
                    || target.starts_with("alloc::")
                {
                    continue;
                }
                if let Some(brace_pos) = target.find("::{") {
                    let inner = target[brace_pos + 3..].trim_end_matches('}').trim();
                    inner
                        .split(',')
                        .map(|s| {
                            s.trim()
                                .split(" as ")
                                .last()
                                .unwrap_or("")
                                .trim()
                                .to_string()
                        })
                        .filter(|n| !n.is_empty() && n != "_")
                        .collect()
                } else {
                    let name = target
                        .split("::")
                        .last()
                        .unwrap_or("")
                        .split(" as ")
                        .last()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if name.is_empty() || name == "_" || name == "*" {
                        continue;
                    }
                    vec![name]
                }
            } else if t.starts_with("import ") {
                let name = t
                    .trim_start_matches("import ")
                    .split_whitespace()
                    .next()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if name.is_empty() || name == "_" {
                    continue;
                }
                vec![name]
            } else if t.starts_with("from ") {
                let after_from = t.trim_start_matches("from ");
                let module = after_from.split_whitespace().next().unwrap_or("");
                if module.is_empty() {
                    continue;
                }
                if let Some(import_pos) = after_from.find(" import ") {
                    let names_part = after_from[import_pos + 8..].trim();
                    let extracted: Vec<String> = names_part
                        .split(',')
                        .map(|s| {
                            s.trim()
                                .split(" as ")
                                .last()
                                .unwrap_or("")
                                .trim()
                                .to_string()
                        })
                        .filter(|n| !n.is_empty() && n != "_")
                        .collect();
                    if extracted.is_empty() {
                        vec![module.to_string()]
                    } else {
                        extracted
                    }
                } else {
                    continue;
                }
            } else {
                continue;
            };

            for name in &names {
                let rest = content
                    .lines()
                    .enumerate()
                    .filter(|(j, _)| *j != i)
                    .map(|(_, l)| l)
                    .collect::<Vec<_>>()
                    .join("\n");
                if rest.contains(name) {
                    continue;
                }
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES015",
                    Severity::MEDIUM,
                    &aes015_unused_import(name),
                ));
            }
        }
    }

    fn check_dead_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();
            if t.starts_with("struct ") && t.ends_with(';') {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES016",
                    Severity::MEDIUM,
                    &aes016_dead_inheritance("unit struct"),
                ));
                i += 1;
                continue;
            }
            if t.starts_with("impl ") {
                let mut impl_str = t.to_string();
                let mut j = i;
                while !impl_str.contains(" for ") && j + 1 < lines.len() {
                    j += 1;
                    impl_str.push_str(lines[j].trim());
                }
                if impl_str.contains(" for ") {
                    if impl_str.trim().ends_with("{}") {
                        violations.push(Self::mk(
                            file,
                            i + 1,
                            "AES016",
                            Severity::MEDIUM,
                            &aes016_dead_inheritance("impl block"),
                        ));
                    } else {
                        let mut k = j;
                        while k < lines.len() && !impl_str.contains('{') {
                            k += 1;
                            if k < lines.len() {
                                impl_str.push(' ');
                                impl_str.push_str(lines[k].trim());
                            }
                        }
                        if impl_str.trim().ends_with("{}") {
                            violations.push(Self::mk(
                                file,
                                i + 1,
                                "AES016",
                                Severity::MEDIUM,
                                &aes016_dead_inheritance("impl block (multi-line)"),
                            ));
                        }
                    }
                }
            }
            i += 1;
        }
    }

    fn check_agent_any_bypass(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let filename = Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if !filename.starts_with("agent_") {
            return;
        }
        for (i, line) in content.lines().enumerate() {
            let wc1 = format!("{}*{}", ":", ":");
            let wc2 = format!("{}* {}", "::", "}");
            if line.trim().contains(&wc1) || line.trim().contains(&wc2) {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES024",
                    Severity::HIGH,
                    "AES024 AGENT_ANY_BYPASS: Wildcard import in agent layer.",
                ));
            }
        }
    }

    fn check_agent_role(file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>) {
        if layer != "agent" && !layer.starts_with("agent(") {
            return;
        }
        if content.lines().count() > 300 {
            violations.push(Self::mk(
                file,
                0,
                "AES021",
                Severity::HIGH,
                "AES021 AGENT_ROLE: Agent file exceeds 300 lines.",
            ));
        }
    }

    fn check_surface_role(
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "surfaces" && !layer.starts_with("surfaces(") {
            return;
        }
        if content.matches("fn ").count() > 15 {
            violations.push(Self::mk(
                file,
                0,
                "AES022",
                Severity::HIGH,
                AES022_SURFACE_ROLE_VIOLATION,
            ));
        }
    }

    fn check_mandatory_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let filename = Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let contract_suffix = if filename.starts_with("infrastructure_") {
            "_port"
        } else if filename.starts_with("capabilities_") {
            "_protocol"
        } else if filename.starts_with("agent_") {
            "_aggregate"
        } else {
            return;
        };
        let mut imported: Vec<String> = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("use ") && t.contains(contract_suffix) {
                if let Some(name) = t.split("::").last() {
                    let c = name.trim_end_matches(';').trim();
                    if c.starts_with('I') || c.ends_with("Protocol") || c.ends_with("Port") {
                        imported.push(c.to_string());
                    }
                }
            }
        }
        for t in &imported {
            if !content.contains(&format!("impl {} for ", t)) {
                violations.push(Self::mk(
                    file,
                    0,
                    "AES027",
                    Severity::HIGH,
                    &aes027_mandatory_inheritance(t),
                ));
            }
        }
    }

    fn check_single_bottleneck(
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        if layer != "capabilities" && !layer.starts_with("capabilities(") {
            return;
        }
        let fc = content.matches("fn ").count();
        let ic = content.matches("impl ").count();
        if fc > 30 {
            violations.push(Self::mk(
                file,
                0,
                "AES031",
                Severity::MEDIUM,
                &format!("{} Found {} functions.", AES031_SINGLE_BOTTLENECK, fc),
            ));
        }
        if ic > 5 {
            violations.push(Self::mk(
                file,
                0,
                "AES031",
                Severity::MEDIUM,
                &format!("{} Found {} impl blocks.", AES031_SINGLE_BOTTLENECK, ic),
            ));
        }
    }

    fn check_missing_vo(file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>) {
        let is_cap = layer == "capabilities" || layer.starts_with("capabilities(");
        let is_infra = layer == "infrastructure" || layer.starts_with("infrastructure(");
        if !is_cap && !is_infra {
            return;
        }
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("let ") && t.contains(" = ") {
                let rhs = t.split(" = ").nth(1).unwrap_or("").trim_end_matches(';');
                if rhs.starts_with('"') && rhs.ends_with('"') && !rhs.contains("::") {
                    violations.push(Self::mk(
                        file,
                        i + 1,
                        "AES032",
                        Severity::MEDIUM,
                        AES032_MISSING_VO,
                    ));
                } else if rhs.parse::<i64>().is_ok() || rhs.parse::<f64>().is_ok() {
                    violations.push(Self::mk(
                        file,
                        i + 1,
                        "AES032",
                        Severity::MEDIUM,
                        AES032_MISSING_VO,
                    ));
                }
            }
        }
    }

    fn check_mcp_schema(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("mcp_") && !file.contains("_schema") {
            return;
        }
        let has = content.contains("fn ")
            && (content.contains("tool") || content.contains("Tool") || content.contains("schema"));
        if !has && content.len() > 50 {
            violations.push(Self::mk(
                file,
                0,
                "AES025",
                Severity::MEDIUM,
                AES025_MCP_SCHEMA,
            ));
        }
    }
}
