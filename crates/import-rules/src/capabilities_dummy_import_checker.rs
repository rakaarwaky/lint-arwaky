// PURPOSE: DummyImportChecker — AES204: detect dummy imports, dummy functions, and dummy trait implementations

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

pub struct DummyImportChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl DummyImportChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    fn check_dummy_imports(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let dummy_ranges = self.parser.get_dummy_function_ranges(&lines, lang);
        let dummy_impl_traits: Vec<String> = self
            .parser
            .get_dummy_impl_traits_with_lines(&lines)
            .into_iter()
            .map(|(trait_name, _)| trait_name)
            .collect();

        // Detect the layer for this file
        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        for (symbol, line_no) in self.parser.get_imported_symbols(&lines, lang) {
            if self
                .parser
                .is_symbol_used_real(&lines, &symbol, &dummy_ranges, &dummy_impl_traits)
            {
                continue;
            }

            violations.push(LintResult::new_arch(
                file,
                line_no,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(symbol),
                    intent: SymbolName::new(
                        "Use imported symbols in real logic, not only in dummy functions or stubs"
                            .to_string(),
                    ),
                    reason: None,
                }
                .to_string(),
            ));
        }
    }

    fn check_dummy_functions(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        for (start, end) in self.parser.get_dummy_function_ranges(&lines, lang) {
            violations.push(LintResult::new_arch(
                file,
                start,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new("_use_mandatory_imports".to_string()),
                    intent: SymbolName::new(
                        "Remove dummy functions that exist only to silence unused import checks"
                            .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                        "Dummy function range ends at line {}",
                        end
                    ))),
                }
                .to_string(),
            ));
        }
    }

    fn check_dummy_impls(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();

        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        for (trait_name, start) in self.parser.get_dummy_impl_traits_with_lines(&lines) {
            violations.push(LintResult::new_arch(
                file,
                start,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(trait_name),
                    intent: SymbolName::new(
                        "Implement contract methods with real behavior instead of empty/todo/panic stubs"
                            .to_string(),
                    ),
                    reason: None,
                }.to_string(),
            ));
        }
    }

    /// Check if taxonomy imports are used in function signatures (not just in dummy functions).
    fn check_taxonomy_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let _layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        let mut has_dummy_function = false;
        let mut dummy_function_line = 0;

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_dummy = match lang {
                LanguageVO::Rust => trimmed.starts_with("fn _use_") && trimmed.contains("()"),
                LanguageVO::Python => trimmed.starts_with("def _use_") && trimmed.contains("()"),
                LanguageVO::JavaScript => {
                    trimmed.starts_with("function _use") && trimmed.contains("()")
                }
                LanguageVO::Unknown => false,
            };
            if is_dummy {
                has_dummy_function = true;
                dummy_function_line = i + 1;
                break;
            }
        }

        if !has_dummy_function {
            return;
        }

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

        let mut has_real_usage = false;
        let mut in_dummy_function = false;
        let mut brace_count = 0;

        for line in &lines {
            let trimmed = line.trim();

            let is_dummy_start = match lang {
                LanguageVO::Rust => trimmed.starts_with("fn _use_"),
                LanguageVO::Python => trimmed.starts_with("def _use_"),
                LanguageVO::JavaScript => trimmed.starts_with("function _use"),
                LanguageVO::Unknown => false,
            };
            if is_dummy_start {
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

            let is_fn = match lang {
                LanguageVO::Rust => trimmed.starts_with("pub fn ") || trimmed.starts_with("fn "),
                LanguageVO::Python => trimmed.starts_with("def "),
                LanguageVO::JavaScript => {
                    trimmed.starts_with("function ")
                        || trimmed.starts_with("const ") && trimmed.contains("=>")
                        || trimmed.starts_with("export ")
                }
                LanguageVO::Unknown => false,
            };
            if is_fn {
                for primitive in &taxonomy_primitives {
                    if trimmed.contains(primitive) {
                        has_real_usage = true;
                        break;
                    }
                }
            }
        }

        if !has_real_usage {
            let has_taxonomy_import = lines.iter().any(|l| {
                let t = l.trim();
                match lang {
                    LanguageVO::Rust => {
                        t.contains("use shared::taxonomy_")
                            || t.contains("use output_report::taxonomy_")
                            || t.contains("use crate::common::taxonomy_")
                            || t.contains("use crate::taxonomy_")
                    }
                    LanguageVO::Python => {
                        t.contains("import taxonomy_") || t.contains("from taxonomy_")
                    }
                    LanguageVO::JavaScript => {
                        t.contains("from 'taxonomy_") || t.contains("from \"taxonomy_")
                    }
                    LanguageVO::Unknown => false,
                }
            });

            if has_taxonomy_import {
                violations.push(LintResult::new_arch(
                    file,
                    dummy_function_line,
                    "AES204",
                    Severity::HIGH,
                    AesImportViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new("surfaces".to_string()),
                        import_type: SymbolName::new("taxonomy".to_string()),
                        intent: SymbolName::new(
                            "Use taxonomy Value Objects in function signatures instead of primitives"
                                .to_string(),
                        ),
                        reason: None,
                    }.to_string(),
                ));
            }
        }
    }

    fn check_aggregate_intent(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let aggregate_types = [
            "DevCommandsAggregate",
            "LintFixOrchestratorAggregate",
            "PluginCommandsAggregate",
            "MaintenanceCommandsAggregate",
            "GitCommandsAggregate",
        ];

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();

            let is_phantom = match lang {
                LanguageVO::Rust => trimmed.contains("PhantomData"),
                LanguageVO::Python => trimmed.contains("TYPE_CHECKING"),
                LanguageVO::JavaScript => {
                    trimmed.contains("@ts-ignore") || trimmed.contains("@ts-expect")
                }
                LanguageVO::Unknown => false,
            };

            if is_phantom {
                for agg_type in &aggregate_types {
                    if trimmed.contains(agg_type) {
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
                                "AES204",
                                Severity::HIGH,
                                AesImportViolation::ImportIntentViolation {
                                    source_layer: LayerNameVO::new("surfaces".to_string()),
                                    import_type: SymbolName::new(agg_type.to_string()),
                                    intent: SymbolName::new(
                                        "Call aggregate functions instead of using PhantomData"
                                            .to_string(),
                                    ),
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

    fn check_surface_logic(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let logic_patterns = [
            "lint_path(",
            "compute_score(",
            "has_critical(",
            "walk_rs_files(",
        ];

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_skip = match lang {
                LanguageVO::Rust => trimmed.starts_with("//") || trimmed.starts_with("fn _use_"),
                LanguageVO::Python => trimmed.starts_with("#") || trimmed.starts_with("def _use_"),
                LanguageVO::JavaScript => {
                    trimmed.starts_with("//") || trimmed.starts_with("function _use")
                }
                LanguageVO::Unknown => false,
            };
            if is_skip {
                continue;
            }

            for pattern in &logic_patterns {
                if trimmed.contains(pattern) {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES204",
                        Severity::MEDIUM,
                        AesImportViolation::ImportIntentViolation {
                            source_layer: LayerNameVO::new("surfaces".to_string()),
                            import_type: SymbolName::new(pattern.to_string()),
                            intent: SymbolName::new(format!(
                                "Delegate to aggregate instead of calling '{}' directly",
                                pattern
                            )),
                            reason: None,
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }
}

#[async_trait]
impl shared::import_rules::contract_rule_protocol::IArchRuleProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }
}

#[async_trait]
impl shared::import_rules::contract_rule_protocol::IArchImportProtocol for DummyImportChecker {
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            // Skip self-check - this file contains hardcoded violation message strings
            if f_str.contains("capabilities_dummy_import_checker") {
                continue;
            }

            let Ok(content) = self.parser.read_file_to_string(f) else {
                continue;
            };

            self.check_dummy_imports(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_functions(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_impls(&f_str, &content, &mut results.values, analyzer, root_dir);

            let basename = std::path::Path::new(&f_str)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            let lang = self.parser.get_language_from_path(&f_str);

            let is_surface = match lang {
                LanguageVO::Rust => {
                    basename.contains("_command")
                        || basename.contains("_controller")
                        || basename.contains("_handler")
                }
                LanguageVO::Python => {
                    basename.contains("command")
                        || basename.contains("controller")
                        || basename.contains("handler")
                }
                LanguageVO::JavaScript => {
                    basename.contains("command")
                        || basename.contains("controller")
                        || basename.contains("handler")
                }
                LanguageVO::Unknown => false,
            };

            if !is_surface {
                continue;
            }

            self.check_taxonomy_intent(&f_str, &content, &mut results.values, analyzer, root_dir);
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
}
