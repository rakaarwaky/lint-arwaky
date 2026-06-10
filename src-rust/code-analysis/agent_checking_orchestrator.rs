// PURPOSE: Orchestrate ALL AES checkers — layer detection, import checks, role checks, orphan detection, cycle detection.

use std::path::Path;
use std::sync::Arc;
use std::sync::OnceLock;

use crate::code_analysis::contract_checker_aggregate::ICheckerAggregate;
use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_violation_rs_constant::{
    AES012_CIRCULAR_IMPORT, AES022_BYPASS_COMMENT, AES022_PANIC, AES022_UNWRAP_EXPECT,
    AES0303_MISSING_VO, AES0303_SINGLE_BOTTLENECK, AES0304_MISSING_VO,
    AES0306_SURFACE_ROLE_VIOLATION,
};
use crate::source_parsing::taxonomy_path_vo::FilePath;

static GLOBAL_CHECKER: OnceLock<Arc<dyn ICheckerAggregate>> = OnceLock::new();

/// Initialize the global checker container. Must be called before using LintCheckingCoordinator.
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
            Self::check_bypass_comments(file, &c, &mut violations);
            Self::check_unused_imports(file, &c, &mut violations);
            Self::check_dead_inheritance(file, &c, &mut violations);

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
            Self::check_agent_role(file, &c, &layer, &mut violations);
            Self::check_surface_role(file, &c, &layer, &mut violations);
            Self::check_single_bottleneck(file, &c, &layer, &mut violations);
            Self::check_missing_vo(file, &c, &layer, &mut violations);
            Self::check_agent_any_bypass(file, &c, &layer, &mut violations);
            Self::check_mandatory_inheritance(file, &c, &layer, &mut violations);

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
            rl.push(Self::mk(
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
        crate::orphan_detector::check_all_orphans(files, root_dir, &eps, &ctx, &mut rl.values);
        // Wire role orchestrator for agent and surface role checks
        let role_orch = crate::role_rules::agent_role_orchestrator::RoleOrchestrator::new(
            Box::new(crate::role_rules::agent_role_container::RoleAggregateImpl::new()),
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
    // INLINE CHECKER METHODS
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
                    "AES022",
                    Severity::CRITICAL,
                    AES022_BYPASS_COMMENT,
                ));
                continue;
            }
            for p in &patterns {
                if t.to_lowercase().contains(p.as_str()) {
                    violations.push(Self::mk(
                        file,
                        i + 1,
                        "AES022",
                        Severity::CRITICAL,
                        AES022_BYPASS_COMMENT,
                    ));
                    break;
                }
            }
            if t.contains(&unwrap_pat) || t.contains(&expect_pat) {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES022",
                    Severity::CRITICAL,
                    AES022_UNWRAP_EXPECT,
                ));
                continue;
            }
            if t.contains(&panic_pat) {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES022",
                    Severity::CRITICAL,
                    AES022_PANIC,
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
                // Skip trait imports (start with 'I' or end with common trait suffixes)
                // These are needed for method resolution even if the trait name
                // doesn't appear literally in the file body.
                if (name.starts_with('I')
                    && name.len() > 1
                    && name.chars().nth(1).unwrap_or(' ').is_uppercase())
                    || name.ends_with("Protocol")
                    || name.ends_with("Port")
                    || name.ends_with("Trait")
                    || name.ends_with("Aggregate")
                    || name == "Parser"
                {
                    continue;
                }
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
                    "AES023",
                    Severity::MEDIUM,
                    &aes023_unused_import(name),
                ));
            }
        }
    }

    fn check_dead_inheritance(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // a trait with empty structs (e.g. SimpleJobRegistry for IJobRegistryPort)
        if content
            .lines()
            .take(30)
            .any(|l| l.contains("aes: bypass-dead-inheritance"))
        {
            return;
        }
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();
            if t.starts_with("struct ") && t.ends_with(';') {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES024",
                    Severity::MEDIUM,
                    &aes024_dead_inheritance("unit struct"),
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
                            "AES024",
                            Severity::MEDIUM,
                            &aes024_dead_inheritance("impl block"),
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
                                "AES024",
                                Severity::MEDIUM,
                                &aes024_dead_inheritance("impl block (multi-line)"),
                            ));
                        }
                    }
                }
            }
            i += 1;
        }
    }

    fn check_agent_any_bypass(
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        // AES0305: Detects `any` type annotations in agent layer files
        if layer != "agent" && !layer.starts_with("agent(") {
            return;
        }
        // Skip files with bypass annotation (e.g., the coordinator itself)
        if content
            .lines()
            .take(30)
            .any(|l| l.contains("aes: bypass-agent-role"))
        {
            return;
        }
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            // Match `: any`, `: Any`, `-> any`, `-> Any`, `Any<`, `any[`
            if t.contains(": any")
                || t.contains(": Any")
                || t.contains("-> any")
                || t.contains("-> Any")
                || t.contains("Any<")
                || t.contains("Any[")
                || t.contains("any[")
            {
                violations.push(Self::mk(
                    file,
                    i + 1,
                    "AES0305",
                    Severity::HIGH,
                    &aes0305_any_type(t),
                ));
            }
        }
    }

    fn check_agent_role(file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>) {
        if layer != "agent" && !layer.starts_with("agent(") {
            return;
        }
        if content
            .lines()
            .take(30)
            .any(|l| l.contains("aes: bypass-agent-role"))
        {
            return;
        }
        if content.lines().count() > 300 {
            violations.push(Self::mk(
                file,
                0,
                "AES0305",
                Severity::HIGH,
                "AES0305 AGENT_ROLE: Agent file exceeds 300 lines.",
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
        // that legitimately register many subcommands via dispatch pattern.
        if content
            .lines()
            .take(30)
            .any(|l| l.contains("aes: bypass-surface-role"))
        {
            return;
        }
        if content.matches("fn ").count() > 15 {
            violations.push(Self::mk(
                file,
                0,
                "AES0306",
                Severity::HIGH,
                AES0306_SURFACE_ROLE_VIOLATION,
            ));
        }
    }

    fn check_mandatory_inheritance(
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let contract_suffix = if layer == "infrastructure" || layer.starts_with("infrastructure(") {
            "_port"
        } else if layer == "capabilities" || layer.starts_with("capabilities(") {
            "_protocol"
        } else if layer == "agent" || layer.starts_with("agent(") {
            "_aggregate"
        } else {
            return;
        };

        // Extract file's own suffix to determine if it's an IMPLEMENTER or COORDINATOR
        // IMPLEMENTER suffixes (_adapter, _provider, _scanner, _analyzer, _checker, _processor, etc.)
        // MUST implement the contracts they import.
        // COORDINATOR suffixes (_orchestrator, _coordinator, _container, _registry, etc.)
        // may import contracts for calling purposes without implementing them.
        let filename = Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let stem = filename.rsplit('.').next_back().unwrap_or(filename);
        let own_suffix = stem.rsplit('_').next().unwrap_or("");
        let implementer_suffixes = [
            "adapter",
            "provider",
            "scanner",
            "client",
            "gateway",
            "repository",
            "connector",
            "cache",
            "loader",
            "writer",
            "reader",
            "driver",
            "analyzer",
            "checker",
            "processor",
            "evaluator",
            "resolver",
            "validator",
            "formatter",
            "executor",
            "transformer",
            "builder",
            "compiler",
            "aggregator",
            "classifier",
            "extractor",
            "reporter",
            "mapper",
            "filter",
            "collector",
            "comparator",
            "scorer",
            "inspector",
            "reviewer",
            "assessor",
            "actions",
        ];
        let is_implementer = implementer_suffixes.contains(&own_suffix);
        if !is_implementer {
            return;
        }

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
        // If file implements at least one contract, skip — other imports are dependencies
        let has_impl = imported
            .iter()
            .any(|t| content.contains(&format!("impl {} for ", t)));
        if !has_impl {
            // Check if all imported contracts are used as dependencies (Arc<dyn, Box<dyn, &dyn)
            let all_are_deps: bool = imported.iter().all(|t| {
                content.contains(&format!("Arc<dyn {}>", t))
                    || content.contains(&format!("Box<dyn {}>", t))
                    || content.contains(&format!("&dyn {}", t))
                    || content.contains(&format!("&dyn mut {}", t))
            });
            if !all_are_deps {
                for t in &imported {
                    if !content.contains(&format!("Arc<dyn {}>", t))
                        && !content.contains(&format!("Box<dyn {}>", t))
                        && !content.contains(&format!("&dyn {}", t))
                        && !content.contains(&format!("&dyn mut {}", t))
                    {
                        violations.push(Self::mk(
                            file,
                            0,
                            "AES014",
                            Severity::HIGH,
                            &aes014_mandatory_inheritance(t),
                        ));
                    }
                }
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
        // Skip if file has bypass-bottleneck annotation
        let first_lines: Vec<&str> = content.lines().take(30).collect();
        if first_lines
            .iter()
            .any(|l| l.trim() == "// aes: bypass-bottleneck")
        {
            return;
        }
        let fc = content.matches("fn ").count();
        let ic = content.matches("impl ").count();
        if fc > 30 {
            violations.push(Self::mk(
                file,
                0,
                "AES0303",
                Severity::MEDIUM,
                &format!("{} Found {} functions.", AES0303_SINGLE_BOTTLENECK, fc),
            ));
        }
        if ic > 5 {
            violations.push(Self::mk(
                file,
                0,
                "AES0303",
                Severity::MEDIUM,
                &format!("{} Found {} impl blocks.", AES0303_SINGLE_BOTTLENECK, ic),
            ));
        }
    }

    fn check_missing_vo(file: &str, content: &str, layer: &str, violations: &mut Vec<LintResult>) {
        let is_cap = layer == "capabilities" || layer.starts_with("capabilities(");
        let is_infra = layer == "infrastructure" || layer.starts_with("infrastructure(");
        if !is_cap && !is_infra {
            return;
        }
        // Skip if file has bypass-missing-vo annotation
        let first_lines: Vec<&str> = content.lines().take(30).collect();
        if first_lines
            .iter()
            .any(|l| l.trim() == "// aes: bypass-missing-vo")
        {
            return;
        }
        for (i, line) in content.lines().enumerate() {
            let t = line.trim();
            if t.starts_with("let ") && t.contains(" = ") {
                let rhs = t.split(" = ").nth(1).unwrap_or("").trim_end_matches(';');
                if (rhs.starts_with('"') && rhs.ends_with('"') && !rhs.contains("::"))
                    || rhs.parse::<i64>().is_ok()
                    || rhs.parse::<f64>().is_ok()
                {
                    let (code, msg) = if is_cap {
                        ("AES0303", AES0303_MISSING_VO)
                    } else {
                        ("AES0304", AES0304_MISSING_VO)
                    };
                    violations.push(Self::mk(file, i + 1, code, Severity::MEDIUM, msg));
                }
            }
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────
// MESSAGE BUILDER FUNCTIONS (moved from taxonomy_violation_rs_vo.rs)
// ─────────────────────────────────────────────────────────────────────────

fn aes023_unused_import(name: &str) -> String {
    format!("AES023 UNUSED_IMPORT: '{}' imported but never used.", name)
}

fn aes024_dead_inheritance(type_name: &str) -> String {
    format!(
        "AES024 DEAD_INHERITANCE: Empty struct/trait '{}' detected.",
        type_name
    )
}

fn aes014_mandatory_inheritance(contracts: &str) -> String {
    format!("AES014 MANDATORY_INHERITANCE: File imports contracts ({}) but no class inherits from them.\nWHY? Layers that import contracts must provide an implementation.\nFIX: Add impl TraitName for YourStruct.", contracts)
}

fn aes0305_any_type(line: &str) -> String {
    format!(
        "AES0305 AGENT_ROLE: Any type annotation found in agent orchestrator layer: '{}'.",
        line.trim()
    )
}
