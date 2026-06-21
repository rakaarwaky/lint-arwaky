// PURPOSE: SuffixPrefixChecker — Handles AES102 suffix/prefix rules (allowed, forbidden, mandatory strict)
use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
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
pub struct SuffixPrefixChecker {}

impl Default for SuffixPrefixChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl SuffixPrefixChecker {
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

    fn get_stem(filename: &str) -> Option<String> {
        if let Some(pos) = filename.rfind('.') {
            Some(filename[..pos].to_string())
        } else {
            Some(filename.to_string())
        }
    }

    fn get_suffix(stem: &str) -> Option<String> {
        stem.rfind('_').map(|pos| stem[pos + 1..].to_string())
    }

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules).
    pub fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        _layer_name: &Option<String>,
        violations: &mut Vec<LintResult>,
    ) {
        // Step 1: Skip checking for barrel files and system entry point files.
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
            return;
        }

        // Step 2: Retrieve the layer definition config.
        let def = match definition {
            Some(d) => d,
            None => return,
        };

        // Step 3: Skip validation if the filename matches an exception rule.
        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        // Step 4: Extract the file stem (name without extension) and get the suffix (word after the last underscore).
        let stem = match Self::get_stem(filename) {
            Some(s) => s,
            None => return,
        };

        let suffix = Self::get_suffix(&stem);

        // Step 5: Check if the suffix is explicitly forbidden for the current layer.
        if let Some(ref suf) = suffix {
            if def.naming.forbidden_suffix.values.contains(suf) {
                let layer_display = _layer_name
                    .as_ref()
                    .map(|l| l.as_str().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::SuffixForbidden {
                        layer_name: layer_display,
                        forbidden_suffix: suf.clone(),
                        reason: None,
                    }
                    .to_string(),
                    Severity::HIGH,
                ));
                return;
            }
        }

        // Step 6: If the layer configuration enforces a strict suffix policy, ensure the suffix matches the allowed list.
        if def.naming.suffix_policy.value == "strict" {
            let valid = suffix
                .as_ref()
                .map(|s| def.naming.allowed_suffix.values.contains(s))
                .unwrap_or(false);
            if !valid {
                let allowed_list = def.naming.allowed_suffix.values.clone();
                let layer_display = _layer_name
                    .as_ref()
                    .map(|l| l.as_str().to_string())
                    .unwrap_or_else(|| "unknown".to_string());
                violations.push(Self::make_result(
                    file,
                    "AES102",
                    NamingViolation::SuffixMismatch {
                        layer_name: layer_display,
                        allowed: allowed_list,
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
impl INamingCheckerProtocol for SuffixPrefixChecker {
    async fn check_file_naming(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
        // No-op for suffix/prefix checker
    }

    // Implement check_domain_suffixes from INamingCheckerProtocol trait to perform checks on multiple files.
    async fn check_domain_suffixes(
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
            // Step 5: Execute the suffix checker function.
            self.check_domain_suffixes(&f_str, filename, def, &layer, &mut results.values);
        }
    }
}
