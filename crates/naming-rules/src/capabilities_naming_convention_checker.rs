// PURPOSE: NamingConventionChecker — Handles AES101 naming convention checks (lowercase, underscore, min 2 words)
use async_trait::async_trait;
use regex::Regex;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_adapter_name_vo::AdapterName;
use shared::taxonomy_common_vo::ColumnNumber;
use shared::taxonomy_common_vo::LineNumber;
use shared::taxonomy_definition_vo::LayerDefinition;
use shared::taxonomy_error_vo::ErrorCode;
use shared::taxonomy_layer_vo::LayerNameVO;
use shared::taxonomy_lint_vo::LocationList;
use shared::taxonomy_lint_vo::ScopeRef;
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_suggestion_vo::DescriptionVO;

#[derive(Clone)]
pub struct NamingConventionChecker {}

impl Default for NamingConventionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl NamingConventionChecker {
    pub fn new() -> Self {
        Self {}
    }

    fn make_result(file: &str, code: &str, msg: impl Into<String>, sev: Severity) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(1),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw(code),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: sev,
            enclosing_scope: Some(ScopeRef {
                name: DescriptionVO::new(String::new()),
                kind: DescriptionVO::new(String::new()),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    fn is_barrel_file(filename: &str) -> bool {
        matches!(
            filename,
            "__init__.py" | "mod.rs" | "index.ts" | "index.js" | "index.tsx" | "index.jsx"
        )
    }

    fn is_entry_point(filename: &str) -> bool {
        matches!(
            filename,
            "__init__.py"
                | "main.py"
                | "py.typed"
                | "app.py"
                | "lib.rs"
                | "main.rs"
                | "index.ts"
                | "index.js"
                | "index.tsx"
                | "index.jsx"
                | "main.ts"
                | "main.js"
                | "app.ts"
                | "app.js"
        )
    }

    /// Check file naming conventions (AES101: pattern validation — lowercase, underscore, min 2 words).
    pub fn check_file_naming(
        &self,
        file: &str,
        filename: &str,
        layer_name: &Option<String>,
        definition: Option<&LayerDefinition>,
        _config: &ArchitectureConfig,
        violations: &mut Vec<LintResult>,
    ) {
        // AES layer prefixes (must match extract_layer_from_prefix in LayerDetectionAnalyzer)
        const LAYER_PREFIXES: &[&str] = &[
            "taxonomy_",
            "contract_",
            "capabilities_",
            "infrastructure_",
            "agent_",
            "surface_",
            "root_",
        ];

        // Step 1: Skip naming verification for barrel files (e.g. __init__.py, mod.rs, index.ts) and entry point files (e.g. main.rs, app.py).
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
            return;
        }

        // Step 2: Handle cases where the layer could not be determined.
        if layer_name.is_none() {
            let stem = filename.split('.').next().unwrap_or(filename);
            let actual_prefix = stem.split('_').next().unwrap_or("").to_string();

            // Check if the file starts with an unrecognized/invalid prefix (not corresponding to a standard AES layer).
            if !actual_prefix.is_empty() && !LAYER_PREFIXES.iter().any(|p| stem.starts_with(p)) {
                let allowed: Vec<String> = LAYER_PREFIXES
                    .iter()
                    .map(|p| p.trim_end_matches('_').to_string())
                    .collect();
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::UnknownPrefix {
                        prefix: actual_prefix,
                        allowed,
                        reason: None,
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
                return;
            }

            // If the prefix is recognized or is empty, but there is no underscore or does not meet basic naming requirements.
            violations.push(Self::make_result(
                file,
                "AES101",
                NamingViolation::NamingConvention {
                    min_words: 2,
                    separator: "_".to_string(),
                    reason: None,
                }
                .to_string(),
                Severity::HIGH,
            ));
            return;
        }

        // Step 3: Retrieve the layer definition. If it does not exist, abort.
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        // Step 4: Skip validation if the file name is explicitly listed in the layer's exception config.
        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        // Step 5: Validate the file stem pattern using a regular expression.
        // It must consist of lowercase letters and digits separated by underscores (e.g., prefix_concept_suffix).
        let stem = filename.split('.').next().unwrap_or("");
        let naming_regex = r"^[a-z0-9]+(_[a-z0-9]+)+$";

        if let Ok(re) = Regex::new(naming_regex) {
            if !re.is_match(stem) {
                violations.push(Self::make_result(
                    file,
                    "AES101",
                    NamingViolation::NamingConvention {
                        min_words: 2,
                        separator: "_".to_string(),
                        reason: None,
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
            }
        }
    }
}

#[async_trait]
impl INamingCheckerProtocol for NamingConventionChecker {
    // Implement check_file_naming from INamingCheckerProtocol trait to perform checks on multiple files.
    async fn check_file_naming(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        // Step 1: Iterate over each file path in the checklist.
        for f in &files.values {
            let f_str = f.to_string();
            // Step 2: Extract the raw filename from the path.
            let filename = f.rsplit('/').next().unwrap_or(&f_str);
            // Step 3: Determine the architectural layer for the file.
            let layer = analyzer
                .detect_layer(f, root_dir)
                .map(|l| l.value().to_string());
            // Step 4: Fetch layer-specific definition properties.
            let def = layer.as_ref().and_then(|l| {
                analyzer
                    .layer_map()
                    .values
                    .get(&LayerNameVO::new(l.as_str()))
            });
            // Step 5: Execute the naming checker function.
            self.check_file_naming(
                &f_str,
                filename,
                &layer,
                def,
                analyzer.config(),
                &mut results.values,
            );
        }
    }

    async fn check_domain_suffixes(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // No-op for naming convention checker
    }
}
