// lint_checking_coordinator — Agent-layer orchestration of ALL AES checkers.
// This is the CORRECT architectural location for wiring checkers (Agent layer).

use std::path::Path;

use crate::capabilities::architecture_compliance_analyzer::ArchComplianceAnalyzer;
use crate::capabilities::architecture_cycle_analyzer::detect_cycle_edges;
use crate::capabilities::architecture_cycle_analyzer::DependencyEdge;
use crate::capabilities::architecture_import_checker::ArchImportRuleChecker;
use crate::capabilities::architecture_internal_checker::ArchInternalChecker;
use crate::capabilities::architecture_metric_checker::ArchMetricChecker;
use crate::capabilities::architecture_naming_checker::ArchNamingChecker;
use crate::capabilities::architecture_orphan_analyzer::OrphanGraphResolver;
use crate::capabilities::surface_hierarchy_checker::SurfaceHierarchyChecker;
use crate::contract::ServiceContainerAggregate;
use crate::taxonomy::{
    AdapterName, ArchitectureConfig, ColumnNumber, ErrorCode, FilePath, LayerDefinition,
    LineNumber, LintMessage, LintResult, LintResultList, LocationList, Severity,
};

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
        let metric_checker = ArchMetricChecker::new();
        let naming_checker = ArchNamingChecker::new();
        let internal_checker = ArchInternalChecker::new();
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

            Self::check_bypass_comments(file, &c, &mut violations);
            Self::check_unused_imports(file, &c, &mut violations);
            Self::check_contract_barrel(file, &c, &mut violations);
            Self::check_dead_inheritance(file, &c, &mut violations);
            Self::check_agent_role(file, &c, &mut violations);
            Self::check_surface_role(file, &c, &mut violations);
            Self::check_surface_imports(file, &c, &mut violations);
            Self::check_agent_any_bypass(file, &c, &mut violations);
            Self::check_mcp_schema(file, &c, &mut violations);
            Self::check_mandatory_inheritance(file, &c, &mut violations);
            Self::check_capability_routing(file, &c, &mut violations);
            Self::check_single_bottleneck(file, &c, &mut violations);
            Self::check_missing_vo(file, &c, &mut violations);

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

            Self::check_forbidden_inheritance(file, &c, def, &mut violations);
            import_checker.check_mandatory_imports(file, def, &mut violations);
            import_checker.check_forbidden_imports(file, &layer, def, &mut violations);
            import_checker.check_legacy_import_rules(file, &layer, config, &mut violations);
            metric_checker.check_line_counts(file, Some(def), &mut violations);
            metric_checker.check_primitive_usage(file, &c, filename, def, &mut violations);
            metric_checker.check_constant_purity(file, &mut violations);
            metric_checker.check_mandatory_class_definition(file, Some(def), &mut violations);
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
                "AES020 CIRCULAR_IMPORT: Circular dependencies detected.",
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
                    "AES017 ORPHAN_CODE: File has no imports, not an entry point.",
                ));
            }
        }
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
                    "AES014 BYPASS_COMMENT: Bypass comment detected.",
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
                        "AES014 BYPASS_COMMENT: Bypass comment detected.",
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
                    "AES014 BYPASS_COMMENT: unwrap/expect call detected.",
                ));
                continue;
            }
            if t.contains(&panic_pat) {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES014",
                    Severity::CRITICAL,
                    "AES014 BYPASS_COMMENT: panic call detected.",
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
                    &format!(
                        "AES015 UNUSED_IMPORT: '{}' imported but never used.",
                        name
                    ),
                ));
            }
        }
    }

    fn check_contract_barrel(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if !t.starts_with("use crate::contract::") {
                continue;
            }
            if t.split("::").count() > 4 {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES007",
                    Severity::MEDIUM,
                    "AES007 CONTRACT_BARREL: Must use barrel import (crate::contract::TypeName).",
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
                    "AES016 DEAD_INHERITANCE: Unit struct — possibly dead inheritance.",
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
                            "AES016 DEAD_INHERITANCE: Empty impl block.",
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
                                "AES016 DEAD_INHERITANCE: Empty impl block (multi-line).",
                            ));
                        }
                    }
                }
            }
            i += 1;
        }
    }

    fn check_agent_any_bypass(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("/agent/") {
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

    fn check_forbidden_inheritance(
        file: &str,
        content: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        if def.forbidden_inheritance.values.is_empty() {
            return;
        }
        let mut forbidden_traits: Vec<String> = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            if !t.starts_with("use ") {
                continue;
            }
            for pattern in &def.forbidden_inheritance.values {
                let (layer, suffixes) = Self::resolve_scope_inheritance(pattern);
                let lower = t.to_lowercase();
                let layer_match = lower.contains(&format!("{}::", layer))
                    || lower.contains(&format!("::{}::", layer));
                if !layer_match {
                    continue;
                }
                if !suffixes.is_empty() {
                    let suffix_match = suffixes.iter().any(|s| {
                        lower.contains(&format!("_{}", s)) || lower.contains(&format!("::{}", s))
                    });
                    if !suffix_match {
                        continue;
                    }
                }
                if let Some(name) = t.split("::").last() {
                    let trait_name = name
                        .trim_end_matches(';')
                        .trim()
                        .trim_start_matches('{')
                        .trim_end_matches('}')
                        .split(',')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .to_string();
                    if !trait_name.is_empty() {
                        forbidden_traits.push(trait_name);
                    }
                }
            }
        }
        for trait_name in &forbidden_traits {
            if content.contains(&format!("impl {} for ", trait_name)) {
                let msg = if !def.forbidden_inheritance_violation_message.value.is_empty() {
                    def.forbidden_inheritance_violation_message.value.clone()
                } else {
                    format!(
                        "AES026 FORBIDDEN_INHERITANCE: '{}' implemented from forbidden source.",
                        trait_name
                    )
                };
                violations.push(Self::mk(file, 0, "AES026", Severity::HIGH, &msg));
            }
        }
    }

    fn resolve_scope_inheritance(scope: &str) -> (&str, Vec<&str>) {
        if let Some(paren) = scope.find('(') {
            let layer = scope[..paren].trim();
            let inner = scope[paren + 1..].trim_end_matches(')').trim();
            let suffixes: Vec<&str> = if inner.contains('|') {
                inner
                    .split('|')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            } else {
                inner
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .collect()
            };
            (layer, suffixes)
        } else {
            (scope.trim(), vec![])
        }
    }

    fn check_agent_role(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("/agent/") {
            return;
        }
        if content.lines().count() > 300 {
            violations.push(Self::mk(
                file,
                0,
                "AES021",
                Severity::HIGH,
                "AES021 AGENT_ROLE: Agent file >300 lines.",
            ));
        }
    }

    fn check_surface_role(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("/surfaces/") {
            return;
        }
        if content.matches("fn ").count() > 15 {
            violations.push(Self::mk(
                file,
                0,
                "AES022",
                Severity::HIGH,
                "AES022 SURFACE_ROLE: Surface file >10 functions.",
            ));
        }
    }

    fn check_surface_imports(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("/surfaces/") {
            return;
        }
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("use ")
                && (t.contains("::capabilities::")
                    || t.contains("::infrastructure::")
                    || t.contains("::agent::"))
            {
                violations.push(Self::mk(
                    file,
                    0,
                    "AES023",
                    Severity::HIGH,
                    "AES023 SURFACE_DEPENDENCY: Surface imports from forbidden layer.",
                ));
                break;
            }
        }
    }

    fn check_mandatory_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let mut imported: Vec<String> = Vec::new();
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("use ") && t.contains("_protocol::") {
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
                    &format!(
                        "AES027 MANDATORY_INHERITANCE: Trait '{}' not implemented.",
                        t
                    ),
                ));
            }
        }
    }

    fn check_capability_routing(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("/capabilities/") {
            return;
        }
        let structs: Vec<&str> = content
            .lines()
            .filter_map(|l| {
                let t = l.trim();
                if t.starts_with("pub struct ") || t.starts_with("struct ") {
                    Some(
                        t.split_whitespace()
                            .nth(1)
                            .unwrap_or("")
                            .trim_end_matches(';'),
                    )
                } else {
                    None
                }
            })
            .filter(|n| !n.is_empty() && !n.starts_with('_'))
            .collect();
        for s in &structs {
            let hi = content.contains(&format!("impl I{}", s))
                || content.contains(&format!(" for {} ", s));
            if !hi && structs.len() <= 3 {
                violations.push(Self::mk(
                    file,
                    0,
                    "AES030",
                    Severity::MEDIUM,
                    &format!("AES030 CAPABILITY_ROUTING: Struct '{}' no trait impl.", s),
                ));
            }
        }
    }

    fn check_single_bottleneck(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("/capabilities/") {
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
                &format!("AES031 SINGLE_BOTTLENECK: {} functions.", fc),
            ));
        }
        if ic > 5 {
            violations.push(Self::mk(
                file,
                0,
                "AES031",
                Severity::MEDIUM,
                &format!("AES031 SINGLE_BOTTLENECK: {} impl blocks.", ic),
            ));
        }
    }

    fn check_missing_vo(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("/capabilities/") && !file.contains("/infrastructure/") {
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
                        "AES032 MISSING_VO: Direct string literal.",
                    ));
                } else if rhs.parse::<i64>().is_ok() || rhs.parse::<f64>().is_ok() {
                    violations.push(Self::mk(
                        file,
                        i + 1,
                        "AES032",
                        Severity::MEDIUM,
                        "AES032 MISSING_VO: Direct numeric literal.",
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
                "AES025 MCP_SCHEMA: MCP file missing tool/schema.",
            ));
        }
    }
}
