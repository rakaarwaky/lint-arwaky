// PURPOSE: ArchImportForbiddenChecker — AES201: enforce forbidden import rules via layer definition and scoped rules
// AES201 rule: Each architectural layer defines which other layers it must NOT import from.
// This checker enforces both (1) global layer-level forbidden rules and (2) per-scope rule-level
// forbidden rules from the config. If a layer's `forbidden` list is empty, surfaces layer defaults
// to forbidding imports from agent, infrastructure, and capabilities (non-recursive default).

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_paths_vo::FilePathList;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_protocol::IImportParserProtocol;
use shared::import_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProtocol, IArchRuleProtocol,
};
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use std::sync::Arc;

/// Returns the inner `FilePath` if `result` is `Ok`, otherwise returns `FilePath::default()`.
/// Private helper — uses `.unwrap_or_else` which is safe (AES304 only forbids bare `.unwrap()`,
/// not fallback variants like `.unwrap_or_else`/`.unwrap_or`/`.unwrap_or_default`).
fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

/// Enforces AES201 forbidden import rules — both layer-level and scope-level rules.
///
/// Workflow (layer-level):
///   1. Check if the file is in the layer definition's exception list — skip if yes.
///   2. Determine the forbidden list: either from the layer definition or a default
///      (surfaces → {agent, infrastructure, capabilities}).
///   3. Parse import lines from the file via the parser port.
///   4. For each import, resolve each segment and compare against the forbidden layer list.
///   5. If a match is found, build a LintResult with the allowed alternatives.
///
/// Workflow (scope-level):
///   - Iterate all config rules and match files by scope (e.g., "agent(container)").
///   - Apply each rule's forbidden list to matching files.
pub struct ArchImportForbiddenChecker {
    parser: Arc<dyn IImportParserProtocol>,
}

impl ArchImportForbiddenChecker {
    pub fn new(parser: Arc<dyn IImportParserProtocol>) -> Self {
        Self { parser }
    }

    /// Check forbidden imports from layer definition (global layer rules).
    ///
    /// Steps:
    ///   1. Get file basename; skip if in the definition's exception list.
    ///   2. If no explicit forbidden list exists, only surfaces layer gets a default forbidden
    ///      list (agent, infrastructure, capabilities). Non-surface layers without forbidden are skipped.
    ///   3. Parse all import lines from the file via `read_import_lines`.
    ///   4. For each import line:
    ///      a. Extract the module path and split into segments.
    ///      b. For each forbidden layer, resolve its scope (layer + optional suffixes).
    ///      c. Check if any segment matches the forbidden layer (exact or suffix-based).
    ///      d. If forbidden, build a violation with allowed alternatives from the definition.
    pub fn check_forbidden_imports(
        &self,
        file: &str,
        layer_name: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Skip files in the exception list
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let basename = file_path.basename();
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        // Step 2: Determine forbidden list (default for surfaces if not explicitly set)
        let is_surfaces = layer_name == "surfaces" || layer_name.starts_with("surfaces(");
        if definition.forbidden.values.is_empty() && !is_surfaces {
            return;
        }

        let forbidden_list: Vec<String> = if !definition.forbidden.values.is_empty() {
            definition.forbidden.values.clone()
        } else {
            vec![
                "agent".to_string(),
                "infrastructure".to_string(),
                "capabilities".to_string(),
            ]
        };

        // Step 3: Parse all import lines from the source file
        let import_lines = self.parser.read_import_lines(&file_path);
        let layer_name_vo = LayerNameVO::new(layer_name);

        // Step 4: Scan each import line for forbidden layers
        for (line_num, line) in &import_lines {
            if let Some(module) = self.parser.extract_module_from_line(line) {
                // Step 4a: Split module path into segments
                let segments: Vec<&str> = module
                    .value()
                    .split([':', '.', '/', '\\'])
                    .filter(|s| !s.is_empty())
                    .collect();

                // Step 4b-c: Check each forbidden layer against all segments
                for forbidden in &forbidden_list {
                    let forbidden_identity = Identity::new(forbidden);
                    let (layer, suffixes) = self.parser.resolve_scope(&forbidden_identity);
                    let is_forbidden = if suffixes.is_empty() {
                        // Exact layer match: check if any segment IS the forbidden layer
                        segments.iter().any(|seg| {
                            let cleaned = seg.trim_end_matches(';').trim();
                            let cleaned_identity = Identity::new(cleaned);
                            match self.parser.extract_layer_from_import(&cleaned_identity) {
                                Some(l) => l == layer,
                                None => false,
                            }
                        })
                    } else {
                        // Suffix-based match: check import scope (e.g., "infrastructure(adapter)")
                        self.parser.import_matches_scope(line, &layer, &suffixes)
                    };

                    // Step 4d: Build violation with allowed alternatives
                    if is_forbidden {
                        let allowed: Vec<LayerNameVO> = definition
                            .allowed
                            .values
                            .iter()
                            .map(|s| {
                                LayerNameVO::new(
                                    self.parser
                                        .resolve_scope(&Identity::new(s))
                                        .0
                                        .value()
                                        .to_string(),
                                )
                            })
                            .collect();
                        violations.push(LintResult::new_arch(
                            file,
                            line_num.value() as usize,
                            "AES201",
                            Severity::CRITICAL,
                            AesImportViolation::ForbiddenImport {
                                source_layer: layer_name_vo.clone(),
                                forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                allowed,
                                reason: None,
                            }
                            .to_string(),
                        ));
                    }
                }
            }
        }
    }

    /// Check forbidden imports from per-rule scope definitions (fine-grained, per-suffix rules).
    ///
    /// Steps:
    ///   1. Get file stem (name without extension) and its last suffix (e.g., "command", "adapter").
    ///   2. Skip special Rust entry files (mod.rs, lib.rs, main.rs).
    ///   3. Parse import lines from the file — skip if empty (no imports to check).
    ///   4. Iterate all config rules:
    ///      a. Skip if file is in the rule's exception list.
    ///      b. Resolve the rule's scope to get (layer, suffixes).
    ///      c. Check if the file's stem starts with the layer prefix AND matches the suffix.
    ///      d. If scope matches, iterate each import line and check against the rule's forbidden list.
    ///      e. For each forbidden match, build a CRITICAL LintResult with allowed alternatives.
    pub fn check_scope_forbidden_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Extract file stem and its last underscore suffix
        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        // Step 2: Skip Rust entry files
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = basename.rsplit('.').next_back().map_or(basename, |s| s);
        let suffix = stem.rsplit('_').next().map_or("", |s| s);

        // Step 3: Parse import lines
        let import_lines = self.parser.read_import_lines(&file_path);
        if import_lines.is_empty() {
            return;
        }

        // Step 4: Check each rule against this file
        for rule in &config.rules {
            // Step 4a: Skip exceptions
            if rule.exceptions.values.contains(&basename.to_string()) {
                continue;
            }
            // Step 4b: Resolve scope → (layer, suffixes)
            let scope_identity = Identity::new(&rule.scope.value);
            let (rule_layer, rule_suffixes) = self.parser.resolve_scope(&scope_identity);
            let rule_layer_str = rule_layer.value();

            // Step 4c: Check if file matches layer prefix AND suffix
            let layer_match = stem.starts_with(&format!("{}_", rule_layer_str));
            if !layer_match {
                continue;
            }
            if !rule_suffixes.is_empty() {
                let suffix_match = rule_suffixes.iter().any(|s| s.value() == suffix);
                if !suffix_match {
                    continue;
                }
            }

            // Step 4d-e: Scan imports against the rule's forbidden list
            for (line_num, line) in &import_lines {
                if let Some(module) = self.parser.extract_module_from_line(line) {
                    let segments: Vec<&str> = module
                        .value()
                        .split([':', '.', '/', '\\'])
                        .filter(|s| !s.is_empty())
                        .collect();
                    for forbidden in &rule.forbidden.values {
                        let forbidden_identity = Identity::new(forbidden);
                        let (forbidden_layer, forbidden_suffixes) =
                            self.parser.resolve_scope(&forbidden_identity);
                        let is_forbidden = if forbidden_suffixes.is_empty() {
                            segments.iter().any(|seg| {
                                let cleaned = seg.trim_end_matches(';').trim();
                                let cleaned_identity = Identity::new(cleaned);
                                match self.parser.extract_layer_from_import(&cleaned_identity) {
                                    Some(l) => l == forbidden_layer,
                                    None => false,
                                }
                            })
                        } else {
                            self.parser.import_matches_scope(
                                line,
                                &forbidden_layer,
                                &forbidden_suffixes,
                            )
                        };
                        if is_forbidden {
                            let allowed: Vec<LayerNameVO> = rule
                                .allowed
                                .values
                                .iter()
                                .map(|s| {
                                    LayerNameVO::new(
                                        self.parser
                                            .resolve_scope(&Identity::new(s))
                                            .0
                                            .value()
                                            .to_string(),
                                    )
                                })
                                .collect();
                            violations.push(LintResult::new_arch(
                                file,
                                line_num.value() as usize,
                                "AES201",
                                Severity::CRITICAL,
                                AesImportViolation::ForbiddenImport {
                                    source_layer: rule_layer.clone(),
                                    forbidden_layer: LayerNameVO::new(forbidden.clone()),
                                    allowed,
                                    reason: None,
                                }
                                .to_string(),
                            ));
                        }
                    }
                }
            }
        }
    }
}

impl IArchRuleProtocol for ArchImportForbiddenChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES201")
    }
}

#[async_trait]
impl IArchImportProtocol for ArchImportForbiddenChecker {
    async fn check_mandatory_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // AES201 only handles forbidden imports — no mandatory import checks.
    }

    /// Run both layer-level and scope-level forbidden import checks on every file.
    ///
    /// Steps:
    ///   1. Iterate all project files.
    ///   2. Check if the file is a rule-level exception (AES201 exception list) — skip if yes.
    ///   3. Detect the file's architectural layer via the analyzer.
    ///   4. Look up the layer definition from the layer map and run `check_forbidden_imports`.
    ///   5. Run `check_scope_forbidden_imports` for all matching per-rule scope definitions.
    async fn check_forbidden_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let basename = f.basename();

            // Step 2: Check Rule Exception directly (avoid LayerDefinition overwrite bugs)
            let mut is_exception = false;
            for r in &analyzer.config().rules {
                if r.name.value.as_str() == "AES201" && r.exceptions.values.contains(&basename) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            // Step 3-4: Detect layer and run layer-level forbidden check
            if let Some(layer) = analyzer.detect_layer(f, root_dir) {
                let layer_str = layer.value();
                if let Some(def) = analyzer.layer_map().values.get(&layer) {
                    self.check_forbidden_imports(&f_str, layer_str, def, &mut results.values);
                }
            }
            // Step 5: Run scope-level forbidden check (per-rule definitions)
            self.check_scope_forbidden_imports(&f_str, analyzer.config(), &mut results.values);
        }
    }
}
