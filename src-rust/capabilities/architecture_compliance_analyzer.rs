// arch_compliance_analyzer — Core layer-detection and compliance orchestration.
// 1:1 Rust port of capabilities/arch_compliance_analyzer.py

use std::collections::HashMap;
use std::path::Path;

use super::architecture_import_checker::ArchImportRuleChecker;
use super::architecture_internal_checker::ArchInternalChecker;
use super::architecture_metric_checker::ArchMetricChecker;
use super::architecture_naming_checker::ArchNamingChecker;
use super::surface_hierarchy_checker::SurfaceHierarchyChecker;
use crate::taxonomy::{
    AdapterName, ArchitectureConfig, ArchitectureRule, ColumnNumber, ErrorCode, FilePath,
    LayerDefinition, LayerNameVO, LineNumber, LintMessage, LintResult, LintResultList, LocationList,
    Severity,
};

pub struct ArchComplianceAnalyzer {
    pub config: ArchitectureConfig,
}

impl ArchComplianceAnalyzer {
    pub fn new(mut config: ArchitectureConfig) -> Self {
        // Group rules by layer name (including global/empty scope)
        let mut rules_by_layer: HashMap<String, Vec<&ArchitectureRule>> = HashMap::new();
        for rule in &config.rules {
            let scope = rule.scope.to_string();
            let layer = if scope.is_empty() {
                String::new()
            } else {
                scope.split('(').next().unwrap_or(&scope).to_string()
            };
            rules_by_layer.entry(layer).or_default().push(rule);
        }

        let mut new_layers: HashMap<LayerNameVO, LayerDefinition> = HashMap::new();
        for (lname, mut ldef) in config.layers {
            let lstr = lname.to_string();
            // Apply global rules (empty scope) to all layers, then layer-specific rules
            for key in ["", &lstr] {
                if let Some(rules) = rules_by_layer.get(key) {
                    for rule in rules {
                        if !rule.exceptions.values.is_empty() {
                            ldef.exceptions = rule.exceptions.clone();
                        }
                        if !rule.mandatory_import.values.is_empty() {
                            ldef.mandatory_import = rule.mandatory_import.clone();
                        }
                        if !rule.forbidden_import.values.is_empty() {
                            ldef.forbidden_import = rule.forbidden_import.clone();
                        }
                        if rule.min_lines.value > 0 {
                            ldef.min_lines = rule.min_lines.clone();
                            ldef.min_lines_violation_message = rule.min_lines_violation_message.clone();
                        }
                        if rule.max_lines.value > 0 {
                            ldef.max_lines = rule.max_lines.clone();
                            ldef.max_lines_violation_message = rule.max_lines_violation_message.clone();
                        }
                        if rule.barrel_completeness.value {
                            ldef.barrel_completeness = rule.barrel_completeness.clone();
                            ldef.barrel_completeness_violation_message = rule.barrel_completeness_violation_message.clone();
                        }
                        if rule.forbid_internal_all.value {
                            ldef.forbid_internal_all = rule.forbid_internal_all.clone();
                            ldef.forbid_internal_all_violation_message = rule.forbid_internal_all_violation_message.clone();
                        }
                        if rule.mandatory_class_definition.value {
                            ldef.mandatory_class_definition = rule.mandatory_class_definition.clone();
                            ldef.mandatory_class_definition_violation_message = rule.mandatory_class_definition_violation_message.clone();
                        }
                        if !rule.forbidden_inheritance.values.is_empty() {
                            ldef.forbidden_inheritance = rule.forbidden_inheritance.clone();
                            ldef.forbidden_inheritance_violation_message = rule.forbidden_inheritance_violation_message.clone();
                        }
                    }
                }
            }
            new_layers.insert(lname, ldef);
        }
        config.layers = new_layers;
        Self { config }
    }

    /// Orchestrate all compliance checks for the given file list under root_dir.
    /// Mirrors Python `execute()` — skips barrel files, detects layers, runs sub-checkers.
    pub fn execute(&self, files: &[String], root_dir: &str) -> Vec<LintResult> {
        if !self.config.enabled.value {
            return Vec::new();
        }

        let mut violations: Vec<LintResult> = Vec::new();
        let import_checker = ArchImportRuleChecker::new();
        let metric_checker = ArchMetricChecker::new();
        let naming_checker = ArchNamingChecker::new();
        let internal_checker = ArchInternalChecker::new();

        let mut file_paths: Vec<FilePath> = Vec::new();
        // For graph-based checks (cycles, orphans)
        let mut import_edges: Vec<(String, String)> = Vec::new();

        for file in files {
            let filename = Path::new(file)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");

            // Collect file paths for project-wide checks
            if let Ok(fp) = FilePath::new(file.to_string()) {
                file_paths.push(fp);
            }

            // Read file content once for all content-based checks
            let content = std::fs::read_to_string(file).unwrap_or_default();

            // ── AES014 — Bypass comments (noqa, #[allow], etc.) ──
            Self::check_bypass_comments(file, &content, &mut violations);

            // ── AES015 — Unused mandatory imports ──
            Self::check_unused_imports(file, &content, &mut violations);

            // ── AES006 — Primitive type usage in domain types ──
            Self::check_primitive_usage(file, &content, &mut violations);

            // ── AES007 — Contract barrel check ──
            Self::check_contract_barrel(file, &content, &mut violations);

            // ── AES016 — Dead inheritance bypass ──
            Self::check_dead_inheritance(file, &content, &mut violations);

            // ── AES024 — Agent any bypass (wildcard/broad imports) ──
            Self::check_agent_any_bypass(file, &content, &mut violations);

            // ── AES026 — Forbidden inheritance ──
            Self::check_forbidden_inheritance(file, &content, &mut violations);

            // ── AES021 — Agent role: container/orchestrator must not contain domain logic ──
            Self::check_agent_role(file, &content, &mut violations);

            // ── AES022 — Surface role: surface must not contain business logic ──
            Self::check_surface_role(file, &content, &mut violations);
            Self::check_surface_imports(file, &content, &mut violations);

            // ── AES027 — Mandatory inheritance: imported protocol must be implemented ──
            Self::check_mandatory_inheritance(file, &content, &mut violations);

            // ── AES030 — Capability method routing ──
            Self::check_capability_routing(file, &content, &mut violations);

            // ── AES031 — Single capability bottleneck ──
            Self::check_single_bottleneck(file, &content, &mut violations);

            // ── AES032 — Missing VO construction ──
            Self::check_missing_vo(file, &content, &mut violations);

            // ── AES025 — MCP tool schema check ──
            Self::check_mcp_schema(file, &content, &mut violations);

            // Build import edges for cycle detection
            for line in content.lines() {
                let trimmed = line.trim();
                if let Some(imp) = trimmed.strip_prefix("use ") {
                    let target = imp.trim_end_matches(';').trim();
                    if !target.is_empty() {
                        import_edges.push((file.to_string(), target.to_string()));
                        // Resolve crate:: imports to file paths for cycle detection
                        if target.starts_with("crate::") {
                            let path_part = target.trim_start_matches("crate::");
                            if let Some(first_break) = path_part.find("::") {
                                let module_path = &path_part[..first_break];
                                let item = path_part[first_break+2..].split("::").next().unwrap_or("");
                                let candidate = format!("{}/{}/{}.rs", root_dir.trim_end_matches('/'), module_path, item);
                                if std::path::Path::new(&candidate).exists() {
                                    import_edges.push((file.to_string(), candidate));
                                }
                            }
                        }
                    }
                } else if let Some(imp) = trimmed.strip_prefix("from ") {
                    let module = imp.split_whitespace().next().unwrap_or("");
                    import_edges.push((file.to_string(), module.to_string()));
                }
            }

            // Skip barrel / entry-point files — but still run AES012 (barrel completeness)
            if Self::is_barrel_file(filename) {
                let b_layer = self.detect_layer(file, root_dir);
                let b_def = b_layer.as_ref().and_then(|l| self.get_layer_def(l));
                if let Some(bd) = b_def {
                    internal_checker.check_internal_rules(file, filename, Some(bd), &mut violations);
                }
                continue;
            }

            let layer = match self.detect_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };

            let def = match self.get_layer_def(&layer) {
                Some(d) => d,
                None => continue,
            };

            // Respect per-layer exception list
            if def.exceptions.values.contains(&filename.to_string()) {
                continue;
            }

            // ── AES001 / AES002 — Layer import rules ──
            import_checker.check_mandatory_imports(file, def, &mut violations);
            import_checker.check_forbidden_imports(file, &layer, def, &mut violations);
            import_checker.check_legacy_import_rules(file, &layer, &self.config, &mut violations);

            // ── AES004 / AES005 — Line count metrics ──
            metric_checker.check_line_counts(file, Some(def), &mut violations);

            // ── AES033 — Constant purity ──
            metric_checker.check_constant_purity(file, &mut violations);

            // ── AES009 — Mandatory class/struct/trait/enum definition ──
            metric_checker.check_mandatory_class_definition(file, Some(def), &mut violations);

            // ── AES003 — Naming convention (3-word underscore pattern) ──
            naming_checker.check_file_naming(
                file, filename, &Some(layer.clone()), Some(def), &self.config, &mut violations,
            );

            // ── AES010 / AES011 — Domain suffix policy ──
            naming_checker.check_domain_suffixes(
                file, filename, Some(def), &Some(layer.clone()), &mut violations,
            );

            // ── AES012 / AES013 — Barrel completeness & forbid_internal_all ──
            internal_checker.check_internal_rules(file, filename, Some(def), &mut violations);
        }

        // ── AES018 / AES019 — Surface hierarchy (project-wide) ──
        let surface_checker = SurfaceHierarchyChecker::new();
        let root_fp = FilePath::new(root_dir.to_string()).unwrap_or_default();
        let mut result_list = LintResultList::new(violations);
        surface_checker.check_surface_hierarchy(&file_paths, &root_fp, &mut result_list);

        // ── AES020 — Cycle detection (project-wide) ──
        let cycle_edges: Vec<super::architecture_cycle_analyzer::DependencyEdge> = import_edges
            .iter()
            .map(|(s, t)| super::architecture_cycle_analyzer::DependencyEdge::new(s.clone(), t.clone()))
            .collect();
        let cycle_results = super::architecture_cycle_analyzer::detect_cycle_edges(&cycle_edges);
        if !cycle_results.is_empty() {
            let msg = format!("AES020 CIRCULAR_IMPORT: Detected {} circular dependencies.", cycle_results.len());
            result_list.push(LintResult {
                file: FilePath::new(root_dir.to_string()).unwrap_or_default(),
                line: LineNumber::new(0),
                column: ColumnNumber::new(0),
                code: ErrorCode::new("AES020").unwrap(),
                message: LintMessage::new(&msg),
                source: Some(AdapterName::new("architecture").unwrap()),
                severity: Severity::CRITICAL,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }

        // ── AES017 — Orphan detection (project-wide) ──
        let orphan_resolver = super::architecture_orphan_analyzer::OrphanGraphResolver::new();
        let ctx = orphan_resolver.build_graph_context(files, root_dir);
        let entry_points = orphan_resolver.identify_entry_points(files);
        for (file_path, imports) in &ctx.import_graph.mapping {
            if imports.is_empty() && !entry_points.contains(file_path) {
                let is_barrel = file_path.ends_with("mod.rs") || file_path.ends_with("__init__.py") || file_path.ends_with("/index.ts");
                if !is_barrel {
                    result_list.push(LintResult {
                        file: FilePath::new(file_path.clone()).unwrap_or_default(),
                        line: LineNumber::new(0),
                        column: ColumnNumber::new(0),
                        code: ErrorCode::new("AES017").unwrap(),
                        message: LintMessage::new("AES017 ORPHAN_CODE: File has no imports and is not an entry point — may be orphaned."),
                        source: Some(AdapterName::new("architecture").unwrap()),
                        severity: Severity::HIGH,
                        enclosing_scope: None,
                        related_locations: LocationList::new(),
                    });
                }
            }
        }

        result_list.values
    }

    // ── Inline helper checkers ──

    /// AES014: Check for bypass comments (noqa, #[allow], type:ignore, etc.)
    fn check_bypass_comments(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let markers = [
            ("#", "noqa"), ("#", "type: ignore"), ("#", "pylint: disable"),
            ("//", "eslint-disable"), ("//@", "ts-ignore"), ("//@", "ts-expect-error"),
            ("//", "NOLINT"), ("#", "NOLINT"),
        ];
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("#[allow(") || trimmed.starts_with("#[expect(") {
                violations.push(Self::make_014_result(file, i + 1, trimmed));
                continue;
            }
            for (pre, kw) in &markers {
                let pat = format!("{}{}", pre, kw);
                if trimmed.to_lowercase().contains(&pat.to_lowercase()) {
                    violations.push(Self::make_014_result(file, i + 1, trimmed));
                    break;
                }
            }
        }
    }

    fn make_014_result(file: &str, line: usize, _text: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::new("AES014").unwrap(),
            message: LintMessage::new("AES014 BYPASS_COMMENT: Bypass comment detected. #[allow(...)], noqa, and similar markers are forbidden (AES014)."),
            source: Some(AdapterName::new("architecture").unwrap()),
            severity: Severity::CRITICAL,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        }
    }

    /// AES015: Check for unused crate imports
    fn check_unused_imports(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if !trimmed.starts_with("use ") {
                continue;
            }
            let import_target = trimmed.trim_end_matches(';').trim_start_matches("use ").trim();
            // Skip crate::self and standard library
            if import_target.starts_with("std::") || import_target.starts_with("core::") || import_target.starts_with("alloc::") {
                continue;
            }
            // Extract the last segment (the type/alias name)
            let last_seg = import_target.split("::").last().unwrap_or("");
            let type_name = last_seg.split("as ").last().unwrap_or(last_seg).trim();
            if type_name.is_empty() || type_name == "_" || type_name.starts_with('{') {
                continue;
            }
            // Check if this type/name appears anywhere else in the file content
            // (excluding the import line itself)
            let mut check_content = content.lines()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, l)| l)
                .collect::<Vec<_>>()
                .join("\n");
            check_content = check_content.to_lowercase();
            let name_lower = type_name.to_lowercase();
            // Skip if it's used somewhere
            if check_content.contains(&name_lower) {
                continue;
            }
            // It's unused
            violations.push(LintResult {
                file: FilePath::new(file.to_string()).unwrap(),
                line: LineNumber::new(i as i64 + 1),
                column: ColumnNumber::new(0),
                code: ErrorCode::new("AES015").unwrap(),
                message: LintMessage::new(format!("AES015 UNUSED_IMPORT: '{}' is imported but never used.", import_target)),
                source: Some(AdapterName::new("architecture").unwrap()),
                severity: Severity::MEDIUM,
                enclosing_scope: None,
                related_locations: LocationList::new(),
            });
        }
    }

    /// AES006: Check for direct primitive type usage in struct fields
    fn check_primitive_usage(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let primitives = [
            "String", "i8", "i16", "i32", "i64", "i128", "isize",
            "u8", "u16", "u32", "u64", "u128", "usize",
            "f32", "f64", "bool", "char", "Vec<", "HashMap<", "Option<", "Result<",
        ];
        // Only check files in taxonomy layer (where primitives matter most)
        let in_taxonomy = file.contains("/taxonomy/");
        if !in_taxonomy {
            return;
        }
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            // Check struct fields: `field_name: Type,`
            if trimmed.contains(':') && (trimmed.ends_with(',') || trimmed.ends_with('}')) {
                let field_type = trimmed.split(':').nth(1).unwrap_or("").trim().trim_end_matches(',').trim_end_matches('}');
                for prim in &primitives {
                    if field_type.starts_with(prim) || field_type == prim.trim_end_matches('<') {
                        violations.push(LintResult {
                            file: FilePath::new(file.to_string()).unwrap(),
                            line: LineNumber::new(i as i64 + 1),
                            column: ColumnNumber::new(0),
                            code: ErrorCode::new("AES006").unwrap(),
                            message: LintMessage::new(format!("AES006 PRIMITIVE_USAGE: Direct primitive type '{}' used in taxonomy. Wrap in a Value Object (VO).", prim)),
                            source: Some(AdapterName::new("architecture").unwrap()),
                            severity: Severity::HIGH,
                            enclosing_scope: None,
                            related_locations: LocationList::new(),
                        });
                        break;
                    }
                }
            }
        }
    }

    /// AES007: Contract barrel check — contract imports must use barrel style
    fn check_contract_barrel(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            if !trimmed.starts_with("use crate::contract::") {
                continue;
            }
            let path = trimmed.trim_end_matches(';').trim();
            // Barrel: use crate::contract::TypeName (3 segments after crate)
            // Non-barrel: use crate::contract::submodule::TypeName (4+ segments)
            let segment_count = path.split("::").count();
            if segment_count > 4 {
                violations.push(LintResult {
                    file: FilePath::new(file.to_string()).unwrap(),
                    line: LineNumber::new(i as i64 + 1),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES007").unwrap(),
                    message: LintMessage::new("AES007 CONTRACT_BARREL: Contract import must be from barrel (use crate::contract::TypeName, not crate::contract::submodule::TypeName)."),
                    source: Some(AdapterName::new("architecture").unwrap()),
                    severity: Severity::MEDIUM,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }
    }

    /// AES016: Dead inheritance bypass — struct/enum with no body inheriting from contracts
    fn check_dead_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let mut in_impl = false;
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            // Check for `struct Foo;` (unit struct — no fields) 
            if trimmed.starts_with("struct ") && trimmed.ends_with(';') {
                violations.push(LintResult {
                    file: FilePath::new(file.to_string()).unwrap(),
                    line: LineNumber::new(i as i64 + 1),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES016").unwrap(),
                    message: LintMessage::new("AES016 DEAD_INHERITANCE: Unit struct with no fields may indicate dead inheritance bypass."),
                    source: Some(AdapterName::new("architecture").unwrap()),
                    severity: Severity::MEDIUM,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
            // Check empty impl blocks: `impl Trait for Type {}`
            if trimmed.starts_with("impl ") && trimmed.contains(" for ") && trimmed.ends_with("{}") {
                violations.push(LintResult {
                    file: FilePath::new(file.to_string()).unwrap(),
                    line: LineNumber::new(i as i64 + 1),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES016").unwrap(),
                    message: LintMessage::new("AES016 DEAD_INHERITANCE: Empty impl block indicates bypass — must implement at least one method."),
                    source: Some(AdapterName::new("architecture").unwrap()),
                    severity: Severity::MEDIUM,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
            // Track impl blocks
            if trimmed.starts_with("impl ") {
                in_impl = true;
            }
            if in_impl && (trimmed == "}" || trimmed.starts_with("fn ")) {
                in_impl = false;
            }
        }
    }

    /// AES024: Agent any bypass — check for broad/wildcard patterns in agent layer
    fn check_agent_any_bypass(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let in_agent = file.contains("/agent/");
        if !in_agent {
            return;
        }
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            // Broad re-exports: `pub use crate::*` or `use crate::*`
            if trimmed.contains("::*;") || trimmed.contains("::* }") {
                violations.push(LintResult {
                    file: FilePath::new(file.to_string()).unwrap(),
                    line: LineNumber::new(i as i64 + 1),
                    column: ColumnNumber::new(0),
                    code: ErrorCode::new("AES024").unwrap(),
                    message: LintMessage::new("AES024 AGENT_ANY_BYPASS: Wildcard/broad import detected in agent layer. Agent must use explicit imports."),
                    source: Some(AdapterName::new("architecture").unwrap()),
                    severity: Severity::HIGH,
                    enclosing_scope: None,
                    related_locations: LocationList::new(),
                });
            }
        }
    }

    /// AES026: Forbidden inheritance — struct derives from forbidden base types
    fn check_forbidden_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let forbidden_bases = ["Clone", "Copy", "Default", "Debug", "PartialEq", "Eq"];
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            // Check derive macros: #[derive(Clone, Debug)]
            if trimmed.starts_with("#[derive(") {
                let derives = trimmed.trim_start_matches("#[derive(").trim_end_matches(')').trim_end_matches(']');
                for base in &forbidden_bases {
                    if derives.contains(base) {
                        violations.push(LintResult {
                            file: FilePath::new(file.to_string()).unwrap(),
                            line: LineNumber::new(i as i64 + 1),
                            column: ColumnNumber::new(0),
                            code: ErrorCode::new("AES026").unwrap(),
                            message: LintMessage::new(format!("AES026 FORBIDDEN_INHERITANCE: Deriving '{}' is forbidden — must implement manually.", base)),
                            source: Some(AdapterName::new("architecture").unwrap()),
                            severity: Severity::HIGH,
                            enclosing_scope: None,
                            related_locations: LocationList::new(),
                        });
                    }
                }
            }
        }
    }

    /// AES021: Agent role — agent files must not contain domain logic
    fn check_agent_role(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let in_agent = file.contains("/agent/");
        if !in_agent { return; }
        let lines: Vec<&str> = content.lines().collect();
        let total_lines = lines.len();
        // Agent files should be relatively small (thin wiring)
        if total_lines > 300 {
            violations.push(Self::make_rule_result(file, 0, "AES021", Severity::HIGH,
                "AES021 AGENT_ROLE: Agent file is too large (>300 lines) — may contain domain logic."));
        }
        // Check for business logic keywords that shouldn't be in agent
        let domain_keywords = ["business", "domain", "calculate", "compute", "validate_", "process_"];
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            for kw in &domain_keywords {
                if trimmed.to_lowercase().contains(kw) {
                    violations.push(Self::make_rule_result(file, i + 1, "AES021", Severity::HIGH,
                        &format!("AES021 AGENT_ROLE: Agent file contains '{}' — domain logic must be in Capabilities.", kw)));
                    break;
                }
            }
        }
    }

    /// AES022: Surface role — surface files must be thin, delegate to agent
    fn check_surface_role(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let in_surface = file.contains("/surfaces/");
        if !in_surface { return; }
        // Surface files should not implement complex logic
        if content.matches("fn ").count() > 10 {
            violations.push(Self::make_rule_result(file, 0, "AES022", Severity::HIGH,
                "AES022 SURFACE_ROLE: Surface file has too many functions (>10) — should delegate to Agent."));
        }
    }

    /// AES023: Surface dependency — surface must not import from capabilities/infrastructure/agent
    fn check_surface_imports(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        if !file.contains("/surfaces/") { return; }
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("use ")
                && (trimmed.contains("::capabilities::")
                    || trimmed.contains("::infrastructure::")
                    || trimmed.contains("::agent::"))
            {
                violations.push(Self::make_rule_result(file, 0, "AES023", Severity::HIGH,
                    "AES023 SURFACE_DEPENDENCY: Surface imports from forbidden layer."));
                break;
            }
        }
    }

    /// AES027: Mandatory inheritance — imported protocol must be implemented
    fn check_mandatory_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let mut imported_traits: Vec<String> = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            // Extract traits imported from contract protocol
            if trimmed.starts_with("use ") && trimmed.contains("_protocol::") {
                if let Some(type_name) = trimmed.split("::").last() {
                    let clean = type_name.trim_end_matches(';').trim();
                    if clean.starts_with('I') || clean.ends_with("Protocol") || clean.ends_with("Port") {
                        imported_traits.push(clean.to_string());
                    }
                }
            }
            // Extract trait names from contract(protocol) barrel imports  
            if trimmed.starts_with("use crate::contract::") && !trimmed.contains("::") {
                let type_name = trimmed.trim_end_matches(';').split("::").last().unwrap_or("").trim();
                if type_name.starts_with('I') || type_name.ends_with("Protocol") || type_name.ends_with("Port") {
                    imported_traits.push(type_name.to_string());
                }
            }
        }
        // Check each imported trait has an `impl Trait for ...` block
        for trait_name in &imported_traits {
            let impl_pattern = format!("impl {} for ", trait_name);
            if !content.contains(&impl_pattern) {
                violations.push(Self::make_rule_result(file, 0, "AES027", Severity::HIGH,
                    &format!("AES027 MANDATORY_INHERITANCE: Imported trait '{}' is not implemented. Must add `impl {} for ...`.", trait_name, trait_name)));
            }
        }
    }

    /// AES030: Capability method routing — check method calls route to correct handler
    fn check_capability_routing(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let in_cap = file.contains("/capabilities/");
        if !in_cap { return; }
        // Check for structs that don't implement any trait/interface
        let mut struct_names: Vec<String> = Vec::new();
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ") {
                let name = trimmed.split_whitespace().nth(1).unwrap_or("").trim_end_matches(';');
                if !name.is_empty() && !name.starts_with('_') {
                    struct_names.push(name.to_string());
                }
            }
        }
        for s in &struct_names {
            let _impl_pattern = format!("impl {} for {}", "", s);
            // Check if struct has any trait implementation
            let has_trait_impl = content.contains(&format!("impl I{}", s))
                || content.contains(&format!(" for {} ", s))
                || content.contains(&format!(" for {}<", s));
            if !has_trait_impl && struct_names.len() <= 3 {
                violations.push(Self::make_rule_result(file, 0, "AES030", Severity::MEDIUM,
                    &format!("AES030 CAPABILITY_ROUTING: Struct '{}' has no trait implementation — may not be routable.", s)));
            }
        }
    }

    /// AES031: Single capability bottleneck — check for overloaded structs
    fn check_single_bottleneck(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let in_cap = file.contains("/capabilities/");
        if !in_cap { return; }
        // A file with too many impl blocks for the same struct may be a bottleneck
        let mut impl_count = 0;
        let mut fn_count = 0;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("impl ") {
                impl_count += 1;
            }
            if trimmed.starts_with("fn ") || trimmed.starts_with("pub fn ") {
                fn_count += 1;
            }
        }
        if fn_count > 30 {
            violations.push(Self::make_rule_result(file, 0, "AES031", Severity::MEDIUM,
                &format!("AES031 SINGLE_BOTTLENECK: Capability file has {} functions — consider splitting into multiple checkers.", fn_count)));
        }
        if impl_count > 5 {
            violations.push(Self::make_rule_result(file, 0, "AES031", Severity::MEDIUM,
                &format!("AES031 SINGLE_BOTTLENECK: Capability file has {} impl blocks — consider separating concerns.", impl_count)));
        }
    }

    /// AES032: Missing VO construction — check direct primitive field access
    fn check_missing_vo(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let in_cap = file.contains("/capabilities/") || file.contains("/infrastructure/");
        if !in_cap { return; }
        // Check for direct string/number assignments without VO wrapping
        let _direct_patterns = [
            ".to_string()", "String::from(", "\"hello", "\"world", "\"error",
            "0i32", "0u32", "0i64", "0usize",
        ];
        for (i, line) in content.lines().enumerate() {
            let trimmed = line.trim();
            let _lower = trimmed.to_lowercase();
            // Look for patterns like `let x = "literal"` or `let x = 42`
            if trimmed.starts_with("let ") && trimmed.contains(" = ") {
                let rhs = trimmed.split(" = ").nth(1).unwrap_or("");
                let rhs_trimmed = rhs.trim_end_matches(';');
                // Check if RHS is a raw primitive/literal
                if rhs_trimmed.starts_with('"') && rhs_trimmed.ends_with('"') && !rhs_trimmed.contains("::") {
                    violations.push(Self::make_rule_result(file, i + 1, "AES032", Severity::MEDIUM,
                        "AES032 MISSING_VO: Direct string literal assigned — should wrap in a Value Object."));
                } else if rhs_trimmed.parse::<i64>().is_ok() || rhs_trimmed.parse::<f64>().is_ok() {
                    violations.push(Self::make_rule_result(file, i + 1, "AES032", Severity::MEDIUM,
                        "AES032 MISSING_VO: Direct numeric literal assigned — should wrap in a Value Object."));
                }
            }
        }
    }

    /// AES025: MCP tool schema check — MCP files must have proper schema definitions
    fn check_mcp_schema(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let is_mcp = file.contains("mcp_") || file.contains("_schemas") || file.contains("_schema");
        if !is_mcp {
            return;
        }
        // MCP files should define at least one tool handler or schema
        let has_tool = content.contains("fn ")
            && (content.contains("tool") || content.contains("Tool") || content.contains("schema") || content.contains("Schema"));
        if !has_tool && content.len() > 50 {
            violations.push(Self::make_rule_result(file, 0, "AES025", Severity::MEDIUM,
                "AES025 MCP_SCHEMA: MCP-related file missing tool handler or schema definition."));
        }
    }

    fn make_rule_result(file: &str, line: usize, code: &str, sev: Severity, msg: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap(),
            line: LineNumber::new(line as i64),
            column: ColumnNumber::new(0),
            code: ErrorCode::new(code).unwrap(),
            message: LintMessage::new(msg),
            source: Some(AdapterName::new("architecture").unwrap()),
            severity: sev,
            enclosing_scope: None,
            related_locations: LocationList::new(),
        }
    }

    /// Determine which layer (if any) a file belongs to.
    ///
    /// Layers are tried in order from most-specific path (longest) to least-specific.
    /// Specialized layers (e.g. `capabilities(command)`) are excluded from the base-layer
    /// scan and resolved in a second pass via `resolve_specialized_layer`.
    pub fn detect_layer(&self, file_path: &str, root_dir: &str) -> Option<String> {
        let rel = Self::get_relative_path(file_path, root_dir);

        // Sort by path-length descending so longer (more specific) paths win.
        let mut sorted_layers: Vec<(&LayerNameVO, &LayerDefinition)> =
            self.config.layers.iter().collect();
        sorted_layers.sort_by(|a, b| b.1.path.value.len().cmp(&a.1.path.value.len()));

        for (name, def) in &sorted_layers {
            // Skip already-specialised entries like "capabilities(command)"
            if name.value.contains('(') {
                continue;
            }

            let is_match = if def.recursive.value {
                Self::match_layer_recursive(&rel, &def.path.value)
            } else {
                Self::match_layer_nonrecursive(&rel, &def.path.value)
            };

            if is_match {
                return Some(self.resolve_specialized_layer(&name.value, file_path));
            }
        }

        None
    }

    /// Determine which layer a dotted module path belongs to.
    ///
    /// Two strategies (Python parity):
    ///   1. Direct segment match against layer names.
    ///   2. Path-based match: module-path-as-filesystem-path contains the layer path.
    pub fn detect_module_layer(&self, module: &str) -> Option<String> {
        let meaningful_parts: Vec<&str> = module.split('.').filter(|p| !p.is_empty()).collect();

        if meaningful_parts.is_empty() {
            return None;
        }

        // 1. Direct match with layer names (ignoring specialisation suffix).
        for (name, _) in &self.config.layers {
            let base_name = name.value.split('(').next().unwrap_or(&name.value);
            if meaningful_parts.contains(&base_name) {
                return Some(self.refine_module_layer(base_name, &meaningful_parts));
            }
        }

        // 2. Match with definition paths (e.g. "src/capabilities" → "capabilities").
        let module_as_path = module.replace('.', "/");
        for (name, def) in &self.config.layers {
            let def_path = def.path.value.trim_matches('/');
            if !def_path.is_empty() && module_as_path.contains(def_path) {
                let base_name = name.value.split('(').next().unwrap_or(&name.value);
                return Some(self.refine_module_layer(base_name, &meaningful_parts));
            }
        }

        None
    }

    /// If the file's stem ends with a suffix that corresponds to a specialised layer
    /// (e.g. `user_command.py` → `capabilities(command)`), return that specialised name.
    /// Otherwise return `base_layer` unchanged.
    fn resolve_specialized_layer(&self, base_layer: &str, file_path: &str) -> String {
        let basename = Path::new(file_path)
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("");

        if let Some(underscore_pos) = basename.rfind('_') {
            let suffix = &basename[underscore_pos + 1..];
            if !suffix.is_empty() {
                let specialized = format!("{}({})", base_layer, suffix);
                let key = LayerNameVO::new(specialized.as_str());
                if self.config.layers.contains_key(&key) {
                    return specialized;
                }
            }
        }

        base_layer.to_string()
    }

    /// Given a known base-layer name and the dotted-module parts, try to find a more
    /// specific specialised layer by inspecting the segment immediately after the base name.
    ///
    /// E.g. parts = ["capabilities", "user_command", "UserCommand"] and base = "capabilities"
    ///      → next part is "user_command", suffix = "command"
    ///      → checks if "capabilities(command)" exists in layers
    fn refine_module_layer(&self, base_name: &str, parts: &[&str]) -> String {
        if let Some(idx) = parts.iter().position(|&p| p == base_name) {
            if idx + 1 < parts.len() {
                let next_part = parts[idx + 1];
                if let Some(underscore_pos) = next_part.rfind('_') {
                    let suffix = &next_part[underscore_pos + 1..];
                    let specialized = format!("{}({})", base_name, suffix);
                    let key = LayerNameVO::new(specialized.as_str());
                    if self.config.layers.contains_key(&key) {
                        return specialized;
                    }
                }
            }
        }
        base_name.to_string()
    }

    /// Recursive match: the relative path starts with the layer path prefix,
    /// or starts with the last path segment of that prefix.
    fn match_layer_recursive(rel: &str, path_def: &str) -> bool {
        let last_segment = path_def.rsplit('/').next().unwrap_or(path_def);
        rel.starts_with(path_def) || rel.starts_with(last_segment)
    }

    /// Non-recursive match: the *parent directory* of the relative path equals the layer path.
    ///
    /// Also handles the case where the analysis is run from inside the layer directory
    /// (rel is just a filename, parent is ".").
    fn match_layer_nonrecursive(rel: &str, path_def: &str) -> bool {
        let norm_path_def = path_def.trim_end_matches('/');

        let parent_dir = match Path::new(rel).parent().and_then(|p| p.to_str()) {
            Some(p) if p.is_empty() => ".",
            Some(p) => p.trim_end_matches('/'),
            None => ".",
        };

        // Case 1: Standard match (rel is "src/capabilities/foo.py", path_def is "src/capabilities")
        if parent_dir == norm_path_def {
            return true;
        }

        // Case 2: Running analysis from inside the layer directory
        // (rel is "foo.py", parent is ".", layer path is "" or ".")
        if parent_dir == "." && (norm_path_def.is_empty() || norm_path_def == ".") {
            return true;
        }

        // Case 3: rel has no directory but layer path appears as a suffix of rel
        if parent_dir == "." && rel.ends_with(norm_path_def) {
            return true;
        }

        // Case 4: File at scan root (no subdirectory) matches non-recursive layer.
        // This case is needed because detect_source_dir descends into src-rust/ before
        // scanning, so a root file like "root_violation.rs" has no "src-rust/" prefix
        // in its relative path. Without this, root layer files never match any layer
        // definition and are skipped entirely by the architectural checkers.
        if parent_dir == "." && !norm_path_def.is_empty() {
            return true;
        }

        false
    }

    /// Look up a `LayerDefinition` by its layer name string.
    pub fn get_layer_def(&self, layer: &str) -> Option<&LayerDefinition> {
        self.config.layers.get(&LayerNameVO::new(layer))
    }

    /// Returns true for conventional barrel / re-export files.
    fn is_barrel_file(filename: &str) -> bool {
        matches!(filename, "__init__.py" | "mod.rs" | "index.ts" | "index.js")
    }

    /// Compute the path of `file_path` relative to `root_dir`.
    /// Falls back to the normalised absolute path when no prefix match is found.
    fn get_relative_path(file_path: &str, root_dir: &str) -> String {
        let normalized_file = file_path.replace('\\', "/");
        let normalized_root = root_dir.trim_end_matches('/').replace('\\', "/");
        if normalized_file.starts_with(&normalized_root) {
            normalized_file[normalized_root.len()..]
                .trim_start_matches('/')
                .to_string()
        } else {
            normalized_file
        }
    }

}
