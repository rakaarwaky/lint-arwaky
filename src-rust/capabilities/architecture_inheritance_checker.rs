// mandatory_inheritance_checker — AES027: mandatory contract inheritance.
// Checks that files in agent/capabilities/infrastructure that import contracts
// also have a class inheriting from them.

use crate::contract::architecture_inheritance_protocol::IArchInheritanceProtocol;
use crate::contract::IAnalyzer;
use crate::taxonomy::{
    AdapterName, ArchitectureConfig, ColumnNumber, ErrorCode, FilePath, FilePathList, LayerNameVO,
    LineNumber, LintMessage, LintResult, LintResultList, LocationList, ScopeRef, Severity,
};
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
            code: ErrorCode::raw("AES027"),
            message: LintMessage::new(msg),
            source: Some(AdapterName::raw("architecture")),
            severity: Severity::CRITICAL,
            enclosing_scope: Some(ScopeRef {
                name: crate::taxonomy::DescriptionVO::new(String::new()),
                kind: crate::taxonomy::DescriptionVO::new(String::new()),
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

        let mut layers: Vec<(&LayerNameVO, &crate::taxonomy::LayerDefinition)> =
            self.config.layers.iter().collect();
        layers.sort_by(|a, b| b.1.path.value.len().cmp(&a.1.path.value.len()));

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
        let is_interface = name.starts_with('I') && name.len() > 1;
        let from_contract = module.contains("contract");

        (has_suffix || (is_interface && has_suffix)) || (from_contract && has_suffix)
    }

    fn extract_contract_imports(content: &str) -> Vec<String> {
        let mut imports: Vec<String> = Vec::new();
        // Match `from x import Y, Z` or `from x import (Y, Z)`
        let from_re = Regex::new(r"from\s+(\S+)\s+import\s+(.+)").expect("valid regex");

        for line in content.lines() {
            let trimmed = line.trim();
            if let Some(caps) = from_re.captures(trimmed) {
                let module = caps.get(1).map(|m| m.as_str()).unwrap_or("");
                let names_str = caps.get(2).map(|m| m.as_str()).unwrap_or("");
                // Parse imported names (handle parentheses on single line)
                let names = names_str
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|s| s.trim().split_whitespace().next().unwrap_or("").to_string())
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
        let class_re = Regex::new(r"class\s+\w+\s*\(([^)]*)\)").expect("valid regex");
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
            let basename = file.split('/').last().unwrap_or("");

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
                let msg = format!(
                    "AES027 MANDATORY_INHERITANCE_VIOLATION: File imports contracts ({}) but no class inherits from any of them. \
                    Layer '{}' must implement its contract via inheritance. \
                    FIX: Make at least one class in this file inherit from one of the imported contracts.",
                    imported_list, layer
                );
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
