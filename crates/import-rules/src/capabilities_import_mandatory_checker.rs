// PURPOSE: ArchImportMandatoryChecker — AES202: enforce mandatory import rules per layer definition and scope rules
// AES202 rule: Each architectural layer (or scoped sub-layer) may declare a set of mandatory imports.
// Files belonging to that layer MUST import at least one symbol from each required scope.
// Two paths: (1) layer-definition mandatory list, (2) per-rule scope mandatory conditions.

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::{
    IAnalyzer, IArchImportProtocol, IArchRuleProtocol,
};
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_layer_vo::{FileContentVO, Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

/// Enforces AES202 mandatory import rules — both layer-level and scope-level.
///
/// Workflow (layer-level):
///   1. If the layer definition has no mandatory list, skip.
///   2. Skip Python __init__.py files and files in the exception list.
///   3. Read file content and parse all import lines.
///   4. For each required scope: check if any import line matches
///      (exact layer string match or suffix-based scope match).
///   5. Report each missing required import as an AES202 HIGH violation.
///
/// Workflow (scope-level):
///   - Same logic but reads mandatory from per-rule scope definitions instead.
pub struct ArchImportMandatoryChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl ArchImportMandatoryChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    /// Check mandatory imports from layer definition (global layer rules).
    ///
    /// Steps:
    ///   1. Return early if the definition has no mandatory imports.
    ///   2. Skip Python __init__.py files (are implicit re-export modules).
    ///   3. Skip files in the exception list.
    ///   4. Read file content and parse import lines.
    ///   5. Derive the source layer name from the file stem.
    ///   6. For each required scope:
    ///      a. Resolve the scope into (layer, optional suffixes).
    ///      b. If no suffixes: check if any import line contains the layer string.
    ///      c. If suffixes: check if any import matches the scope (layer + suffixes).
    ///      d. If missing, emit an AES202 HIGH violation with the required scope name.
    pub fn check_mandatory_imports(
        &self,
        file: &str,
        definition: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Skip if no mandatory imports defined
        if definition.mandatory.values.is_empty() {
            return;
        }

        // Step 2-3: Skip special files and exceptions
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        if basename == "__init__.py" {
            return;
        }
        if definition.exceptions.values.contains(&basename.to_string()) {
            return;
        }

        // Step 4: Read file and parse import lines
        let Ok(content_msg) = self.parser.read_file_to_message(&file_path) else {
            return;
        };
        let content = content_msg.value().to_string();
        let file_content = FileContentVO::new(content);
        let import_lines = self.parser.parse_import_lines(&file_content);

        // Step 5: Derive source layer from filename (first prefix segment)
        let stem = match basename.rsplit('.').next_back() {
            Some(s) => s,
            None => basename,
        };
        let source_layer = match stem.split('_').next() {
            Some(s) => s,
            None => "unknown",
        };

        // Step 6: Check each required scope against actual imports
        for required in &definition.mandatory.values {
            let required_identity = Identity::new(required);
            let (layer, suffixes) = self.parser.resolve_scope(&required_identity);
            let layer_str = layer.value();

            // Step 6a-c: Check if any import line matches the required scope
            let is_present = if suffixes.is_empty() {
                import_lines
                    .iter()
                    .any(|(_, l)| l.value().contains(layer_str))
            } else {
                import_lines
                    .iter()
                    .any(|(_, l)| self.parser.import_matches_scope(l, &layer, &suffixes))
            };

            // Step 6d: Report missing import
            if !is_present {
                violations.push(LintResult::new_arch(
                    file,
                    0,
                    "AES202",
                    Severity::HIGH,
                    AesImportViolation::MissingImport {
                        source_layer: LayerNameVO::new(source_layer.to_string()),
                        required: SymbolName::new(required.clone()),
                        reason: None,
                    }
                    .to_string(),
                ));
            }
        }
    }

    /// Check mandatory imports from per-rule scope definitions (fine-grained, per-suffix rules).
    /// This is the primary path — reads mandatory from each rule's scope configuration.
    ///
    /// Steps:
    ///   1. Get file stem (name without extension) and its last underscore suffix.
    ///   2. Skip Rust entry files (mod.rs, lib.rs, main.rs).
    ///   3. Parse import lines from the file.
    ///   4. Iterate all config rules:
    ///      a. Skip rules with empty mandatory lists.
    ///      b. Resolve the rule's scope into (layer, suffixes).
    ///      c. Check if file's stem starts with the layer prefix AND matches the suffix.
    ///      d. For each required scope in the rule:
    ///         - Resolve the required scope into (layer, suffixes).
    ///         - Check if any import line matches (exact layer string or suffix-based).
    ///         - If missing, emit an AES202 HIGH violation.
    pub fn check_scope_mandatory_imports(
        &self,
        file: &str,
        config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Extract file stem and suffix
        let file_path = FilePath::new(file.to_string()).unwrap_or_default();
        let basename_identity = self.parser.get_basename(&file_path);
        let basename = basename_identity.value();
        // Step 2: Skip Rust entry files
        if basename == "mod.rs" || basename == "lib.rs" || basename == "main.rs" {
            return;
        }
        let stem = match basename.rsplit('.').next_back() {
            Some(s) => s,
            None => basename,
        };
        let suffix = match stem.rsplit('_').next_back() {
            Some(s) => s,
            None => "",
        };

        // Step 3: Parse import lines
        let import_lines = self.parser.read_import_lines(&file_path);

        // Step 4: Check each rule against this file
        for rule in &config.rules {
            // Step 4a: Skip rules without mandatory imports
            if rule.mandatory.values.is_empty() {
                continue;
            }

            // Step 4b-c: Check scope match
            let scope_identity = Identity::new(&rule.scope.value);
            let (rule_layer, rule_suffixes) = self.parser.resolve_scope(&scope_identity);
            let rule_layer_str = rule_layer.value();
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

            // Step 4d: Check each required import
            for required in &rule.mandatory.values {
                let required_identity = Identity::new(required);
                let (req_layer, req_suffixes) = self.parser.resolve_scope(&required_identity);
                let req_layer_str = req_layer.value();

                let is_present = if req_suffixes.is_empty() {
                    if import_lines.is_empty() {
                        false
                    } else {
                        import_lines
                            .iter()
                            .any(|(_, l)| l.value().contains(req_layer_str))
                    }
                } else {
                    import_lines.iter().any(|(_, l)| {
                        self.parser
                            .import_matches_scope(l, &req_layer, &req_suffixes)
                    })
                };

                if !is_present {
                    violations.push(LintResult::new_arch(
                        file,
                        0,
                        "AES202",
                        Severity::HIGH,
                        AesImportViolation::MissingImport {
                            source_layer: rule_layer.clone(),
                            required: SymbolName::new(required.clone()),
                            reason: None,
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }
}

impl IArchRuleProtocol for ArchImportMandatoryChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES202")
    }
}

#[async_trait]
impl IArchImportProtocol for ArchImportMandatoryChecker {
    /// Run both layer-level and scope-level mandatory import checks on every file.
    ///
    /// Steps:
    ///   1. Iterate all project files.
    ///   2. Check if the file is a rule-level exception (AES202 exception list) — skip if yes.
    ///   3. Detect the file's architectural layer via the analyzer.
    ///   4. Look up the layer definition and run layer-level `check_mandatory_imports`.
    ///   5. Run scope-level `check_scope_mandatory_imports` for all matching per-rule definitions.
    async fn check_mandatory_imports(
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
                if r.name.value.as_str() == "AES202" && r.exceptions.values.contains(&basename) {
                    is_exception = true;
                    break;
                }
            }
            if is_exception {
                continue;
            }

            // Step 3-4: Detect layer and run layer-level mandatory check
            if let Some(layer) = analyzer.detect_layer(f, root_dir) {
                if let Some(def) = analyzer.layer_map().values.get(&layer) {
                    self.check_mandatory_imports(&f_str, def, &mut results.values);
                }
            }
            // Step 5: Run scope-level mandatory check
            self.check_scope_mandatory_imports(&f_str, analyzer.config(), &mut results.values);
        }
    }

    async fn check_forbidden_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // AES202 only handles mandatory imports — no forbidden import checks.
    }
}
