// PURPOSE: ArchNamingChecker — INamingCheckerProtocol for AES101 (naming convention) and AES102 (suffix/prefix rules)

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::config_system::taxonomy_config_vo::ArchitectureConfig;
use shared::import_rules::contract_rule_protocol::{IAnalyzer, INamingCheckerProtocol};
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

use regex::Regex;
use shared::naming_rules::taxonomy_naming_violation_vo::NamingViolation;

#[derive(Clone)]
pub struct ArchNamingChecker {}

impl Default for ArchNamingChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchNamingChecker {
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

        println!("[debug] check_file_naming: filename = {}, layer = {:?}, def is some: {}", filename, layer_name, definition.is_some());
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
            return;
        }

        if layer_name.is_none() {
            let stem = filename.split('.').next().unwrap_or(filename);
            let actual_prefix = stem.split('_').next().unwrap_or("").to_string();
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
            // No underscore at all or no known prefix — regular naming violation
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
        let def = match definition {
            Some(d) => d,
            None => return,
        };
        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        let stem = filename.split('.').next().unwrap_or("");
        // Unlimited words: at least `prefix_suffix` (2 words), any concept words in between
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

    /// Check domain suffix rules per layer (AES102: suffix/prefix rules).
    pub fn check_domain_suffixes(
        &self,
        file: &str,
        filename: &str,
        definition: Option<&LayerDefinition>,
        _layer_name: &Option<String>,
        violations: &mut Vec<LintResult>,
    ) {
        if Self::is_barrel_file(filename) || Self::is_entry_point(filename) {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };

        if def.exceptions.values.contains(&filename.to_string()) {
            return;
        }

        let stem = match Self::get_stem(filename) {
            Some(s) => s,
            None => return,
        };

        let suffix = Self::get_suffix(&stem);

        // AES102: Forbidden suffix check
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

        // AES102: Strict suffix policy check
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
impl INamingCheckerProtocol for ArchNamingChecker {
    async fn check_file_naming(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let keys: Vec<String> = analyzer.layer_map().values.keys().map(|k| k.value().to_string()).collect();
        println!("[debug] layer_map keys: {:?}", keys);
        for f in &files.values {
            let f_str = f.to_string();
            let filename = f.rsplit('/').next().unwrap_or(&f_str);
            let layer = analyzer
                .detect_layer(f, root_dir)
                .map(|l| l.value().to_string());
            let def = layer.as_ref().and_then(|l| {
                analyzer
                    .layer_map()
                    .values
                    .get(&LayerNameVO::new(l.as_str()))
            });
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
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            let filename = f.rsplit('/').next().unwrap_or(&f_str);
            let layer = analyzer
                .detect_layer(f, root_dir)
                .map(|l| l.value().to_string());
            let def = layer.as_ref().and_then(|l| {
                analyzer
                    .layer_map()
                    .values
                    .get(&LayerNameVO::new(l.as_str()))
            });
            self.check_domain_suffixes(&f_str, filename, def, &layer, &mut results.values);
        }
    }
}
