// mandatory_inheritance_checker — AES027: mandatory contract inheritance.
// Checks that files in agent/capabilities/infrastructure that import contracts
// also have a class inheriting from them.

use crate::config_system::taxonomy_config_vo::ArchitectureConfig;
use crate::layer_rules::contract_inheritance_protocol::IArchInheritanceProtocol;
use crate::layer_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_result_vo::LintResultList;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_error_vo::ErrorCode;
use crate::shared_common::taxonomy_layer_vo::LayerNameVO;
use crate::shared_common::taxonomy_lint_vo::LocationList;
use crate::shared_common::taxonomy_lint_vo::ScopeRef;
use crate::shared_common::taxonomy_message_vo::LintMessage;
use crate::shared_common::taxonomy_adapter_name_vo::AdapterName;
use crate::shared_common::taxonomy_violationrs_constant::aes034_mandatory_inheritance;
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;
use regex::Regex;
use std::fs;

// Map layer name → required contract suffix pattern
const LAYER_CONTRACT_SUFFIX: &[(&str, &str)] = &[
    ("infrastructure", "_port"),
    ("capabilities", "_protocol"),
    ("agent", "_aggregate"),
];

/// Check that files in agent/capabilities/infrastructure that import contracts
/// also have a class inheriting from them (AES027).
pub struct MandatoryInheritanceChecker {
    config: ArchitectureConfig,
}

impl MandatoryInheritanceChecker {
    pub fn new(config: ArchitectureConfig) -> Self {
        Self { config }
    }

    fn make_result(file: &str, msg: &str) -> LintResult {
        LintResult {
            file: FilePath::new(file.to_string()).unwrap_or_default(),
            line: LineNumber::new(0),
            column: ColumnNumber::new(0),
            code: ErrorCode::raw("AES034"),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::CRITICAL,
            enclosing_scope: Some(ScopeRef {
                name: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(
                    String::new(),
                ),
                kind: crate::shared_common::taxonomy_suggestion_vo::DescriptionVO::new(
                    String::new(),
                ),
                file: None,
                start_line: None,
                end_line: None,
            }),
            related_locations: LocationList::new(),
        }
    }

    fn detect_file_layer(&self, file: &str, root_dir: &str) -> Option<String> {
        let rel = file
            .strip_prefix(root_dir)
            .unwrap_or(file)
            .trim_start_matches('/');

        let mut layers: Vec<(
            &LayerNameVO,
            &crate::shared_common::taxonomy_definition_vo::LayerDefinition,
        )> = self.config.layers.iter().collect();
        layers.sort_by_key(|b| std::cmp::Reverse(b.1.path.value.len()));

        for (name, def) in layers {
            if rel.starts_with(&def.path.value) {
                return Some(name.value.clone());
            }
        }
        None
    }

    fn is_contract_import(name: &str, module: &str) -> bool {
        let contract_suffixes = ["_port", "_protocol", "_aggregate"];
        let has_suffix = contract_suffixes.iter().any(|s| name.ends_with(s));
        let _is_interface = name.starts_with('I') && name.len() > 1;
        let _from_contract = module.contains("contract");

        has_suffix
    }

    fn extract_contract_imports(content: &str) -> Vec<String> {
        let mut imports: Vec<String> = Vec::new();
        // Match `from x import Y, Z` or `from x import (Y, Z)`
        let from_re = match Regex::new(r"from\s+(\S+)\s+import\s+(.+)") {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(caps) = from_re.captures(trimmed) {
                let module = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let names_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                // Parse imported names (handle parentheses on single line)
                let names = names_str
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| s.split_whitespace().next().unwrap_or("").to_string())
                    .filter(|s| !s.is_empty());

                for name in names {
                    if Self::is_contract_import(&name, module) {
                        imports.push(name);
                    }
                }
            }
        }
        imports
    }

    fn extract_class_bases(content: &str) -> Vec<String> {
        let class_re = match Regex::new(r"class\s+\w+\s*\(([^)]*)\)") {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };
        let mut bases: Vec<String> = Vec::new();

        for line in content.lines() {
            if let Some(caps) = class_re.captures(line) {
                let base_str = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                for base in base_str.split(',') {
                    let base_name = base.trim().to_string();
                    if !base_name.is_empty() {
                        bases.push(base_name);
                    }
                }
            }
        }
        bases
    }

    /// Check mandatory inheritance for a list of files.
    pub fn check_mandatory_inheritance(
        &self,
        files: &[String],
        root_dir: &str,
        violations: &mut Vec<LintResult>,
    ) {
        for file in files {
            let basename = file.split('/').next_back().unwrap_or("");

            // Skip barrel files
            if basename == "__init__.py" || basename == "mod.rs" {
                continue;
            }

            let layer = match self.detect_file_layer(file, root_dir) {
                Some(l) => l,
                None => continue,
            };

            // Only check agent/capabilities/infrastructure layers
            let _layer_suffix = match LAYER_CONTRACT_SUFFIX
                .iter()
                .find(|(l, _)| *l == layer.as_str())
            {
                Some((_, s)) => *s,
                None => continue,
            };

            let Ok(content) = fs::read_to_string(file) else {
                continue;
            };

            let contract_imports = Self::extract_contract_imports(&content);
            if contract_imports.is_empty() {
                continue; // No contract imported → surface-like, skip
            }

            let class_bases = Self::extract_class_bases(&content);

            // Check: does any class base match a contract import?
            let inherited = contract_imports
                .iter()
                .any(|ci| class_bases.iter().any(|base| base.contains(ci.as_str())));

            if !inherited {
                let imported_list = contract_imports.join(", ");
                let msg = aes034_mandatory_inheritance(&imported_list);
                violations.push(Self::make_result(file, &msg));
            }
        }
    }
}

#[async_trait]
impl IArchInheritanceProtocol for MandatoryInheritanceChecker {
    async fn check_mandatory_inheritance(
        &self,
        _analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        let file_strs: Vec<String> = files.iter().map(|f| f.value().to_string()).collect();
        self.check_mandatory_inheritance(&file_strs, root_dir.value(), &mut results.values);
    }
}
