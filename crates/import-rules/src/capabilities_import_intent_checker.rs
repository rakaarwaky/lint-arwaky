// PURPOSE: ImportIntentChecker — AES002X: enforce mandatory import intent, not just syntax
//
// AES002 saat ini hanya cek apakah import ADA di file.
// Tapi intent-nya adalah:
//   1. taxonomy imports → surface harus pakai Value Objects di function signatures
//   2. contract(aggregate) imports → surface harus delegate ke aggregate, bukan implement sendiri
//
// Rule ini cek apakah import BENAR-BENAR digunakan sesuai intent.

use crate::import_rules::contract_rule_protocol::IAnalyzer;
use crate::output_report::taxonomy_result_vo::{LintResult, LintResultList};
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violation_message_rs_error::AesViolation;
use crate::shared_common::{Identity, LayerNameVO, SymbolName};
use crate::source_parsing::taxonomy_path_vo::FilePath;
use crate::source_parsing::taxonomy_paths_vo::FilePathList;
use async_trait::async_trait;
use std::sync::Arc;

pub struct ImportIntentChecker {}

impl ImportIntentChecker {
    pub fn new(
        _parser: Arc<dyn crate::import_rules::contract_import_parser_port::IImportParserPort>,
    ) -> Self {
        Self {}
    }

    /// Check if taxonomy imports are used in function signatures (not just in dummy functions).
    /// Intent: surface files harus pakai Value Objects di signatures, bukan primitives.
    fn check_taxonomy_intent(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut has_dummy_function = false;
        let mut dummy_function_line = 0;

        // Detect dummy function pattern: fn _use_mandatory_imports() { let _ = ... }
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("fn _use_") && trimmed.contains("()") {
                has_dummy_function = true;
                dummy_function_line = i + 1;
                break;
            }
        }

        if !has_dummy_function {
            return;
        }

        // Check if taxonomy imports (LineNumber, ColumnNumber, etc.) are used in actual function signatures
        let taxonomy_primitives = [
            "LineNumber",
            "ColumnNumber",
            "Count",
            "Score",
            "String",
            "bool",
            "i32",
            "u32",
            "f64",
            "usize",
        ];

        // Get all public function signatures
        let mut has_real_usage = false;
        let mut in_dummy_function = false;
        let mut brace_count = 0;

        for line in &lines {
            let trimmed = line.trim();

            // Track dummy function scope
            if trimmed.starts_with("fn _use_") {
                in_dummy_function = true;
                brace_count = 0;
                continue;
            }

            if in_dummy_function {
                brace_count += trimmed.matches('{').count();
                brace_count = brace_count.saturating_sub(trimmed.matches('}').count());
                if brace_count == 0 && trimmed.contains('}') {
                    in_dummy_function = false;
                }
                continue;
            }

            // Check if any public function uses primitive types in signature
            if trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ") {
                for primitive in &taxonomy_primitives {
                    if trimmed.contains(primitive) {
                        has_real_usage = true;
                        break;
                    }
                }
            }
        }

        // If taxonomy imports exist but are only used in dummy function → VIOLATION
        if !has_real_usage {
            // Check if file actually imports taxonomy types
            let has_taxonomy_import = lines.iter().any(|l| {
                l.contains("use crate::shared_common::taxonomy_")
                    || l.contains("use crate::output_report::taxonomy_")
            });

            if has_taxonomy_import {
                violations.push(LintResult::new_arch(
                    file,
                    dummy_function_line,
                    "AES002X",
                    Severity::HIGH,
                    AesViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new("surfaces".to_string()),
                        import_type: SymbolName::new("taxonomy".to_string()),
                        intent: SymbolName::new(
                            "Use taxonomy Value Objects in function signatures instead of primitives".to_string()
                        ),
                        reason: None,
                    },
                ));
            }
        }
    }

    /// Check if contract(aggregate) imports are actually called (not just PhantomData).
    /// Intent: surface files harus delegate ke aggregate, bukan implement logic sendiri.
    fn check_aggregate_intent(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();

        // Find PhantomData usage of aggregate types
        let aggregate_types = [
            "ReportCommandsAggregate",
            "DevCommandsAggregate",
            "LintFixOrchestratorAggregate",
            "PipelineActionDispatcherAggregate",
            "PluginCommandsAggregate",
            "OutputClientAggregate",
            "MaintenanceCommandsAggregate",
            "GitCommandsAggregate",
        ];

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            // Check for PhantomData usage (dummy import pattern)
            if trimmed.contains("PhantomData") {
                for agg_type in &aggregate_types {
                    if trimmed.contains(agg_type) {
                        // Found PhantomData::<dyn AggregateType> — this is a dummy import
                        // Check if this aggregate type is actually USED in the file (not just PhantomData)
                        let type_name = agg_type.to_string();
                        let real_usage_count = lines
                            .iter()
                            .filter(|l| {
                                let t = l.trim();
                                t.contains(&type_name)
                                    && !t.contains("PhantomData")
                                    && !t.contains("fn _use_")
                                    && !t.starts_with("//")
                            })
                            .count();

                        if real_usage_count == 0 {
                            violations.push(LintResult::new_arch(
                                file,
                                i + 1,
                                "AES002X",
                                Severity::HIGH,
                                AesViolation::ImportIntentViolation {
                                    source_layer: LayerNameVO::new("surfaces".to_string()),
                                    import_type: SymbolName::new(agg_type.to_string()),
                                    intent: SymbolName::new(
                                        "Call aggregate functions instead of using PhantomData"
                                            .to_string(),
                                    ),
                                    reason: None,
                                },
                            ));
                        }
                    }
                }
            }
        }
    }

    /// Check if surface file implements logic that should be in aggregate.
    /// Intent: surface harus delegate, bukan implement.
    fn check_surface_logic(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();

        // Patterns that indicate surface is implementing logic instead of delegating
        let logic_patterns = [
            "lint_path(",     // direct call instead of through aggregate
            "compute_score(", // direct call instead of through aggregate
            "has_critical(",  // direct call instead of through aggregate
            "walk_rs_files(", // direct call instead of through aggregate
        ];

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("//") || trimmed.starts_with("fn _use_") {
                continue;
            }

            for pattern in &logic_patterns {
                if trimmed.contains(pattern) {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES002X",
                        Severity::MEDIUM,
                        AesViolation::ImportIntentViolation {
                            source_layer: LayerNameVO::new("surfaces".to_string()),
                            import_type: SymbolName::new(pattern.to_string()),
                            intent: SymbolName::new(format!(
                                "Delegate to aggregate instead of calling '{}' directly",
                                pattern
                            )),
                            reason: None,
                        },
                    ));
                }
            }
        }
    }
}

#[async_trait]
impl crate::import_rules::contract_rule_protocol::IArchRuleProtocol for ImportIntentChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES002X")
    }
}

#[async_trait]
impl crate::import_rules::contract_rule_protocol::IArchImportProtocol for ImportIntentChecker {
    async fn check_mandatory_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();

            // Only check surface files (command, controller, handler)
            let basename = std::path::Path::new(&f_str)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            let is_surface = basename.contains("_command")
                || basename.contains("_controller")
                || basename.contains("_handler");

            if !is_surface {
                continue;
            }

            let Ok(content) = std::fs::read_to_string(&f_str) else {
                continue;
            };

            self.check_taxonomy_intent(&f_str, &content, &mut results.values);
            self.check_aggregate_intent(&f_str, &content, &mut results.values);
            self.check_surface_logic(&f_str, &content, &mut results.values);
        }
    }

    async fn check_forbidden_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
    }

    async fn check_legacy_import_rules(
        &self,
        _analyzer: &dyn IAnalyzer,
        _files: &FilePathList,
        _root_dir: &FilePath,
        _results: &mut LintResultList,
    ) {
    }
}
