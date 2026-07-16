use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_layer_detection_protocol::ILayerDetectionProtocol;
use shared::common::taxonomy_path_vo::FilePath;

use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

fn filepath_or_default(result: Result<FilePath, impl std::fmt::Debug>) -> FilePath {
    result.unwrap_or_default()
}

pub struct DummyImportChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl DummyImportChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }
}

#[async_trait]
impl IDummyImportCheckerProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }
    fn check_dummy_imports(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let dummy_ranges = self.parser.get_dummy_function_ranges(&lines, lang);
        let dummy_impl_traits: Vec<String> = self
            .parser
            .get_dummy_impl_traits_with_lines(&lines)
            .into_iter()
            .map(|(trait_name, _)| trait_name.value().to_string())
            .collect();

        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let layer_name = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => "any".to_string(),
        };

        for (symbol, line_no) in self.parser.get_imported_symbols(&lines, lang) {
            let symbol_str = symbol.value().to_string();
            if self.parser.is_symbol_used_real(
                &lines,
                &symbol_str,
                &dummy_ranges,
                &dummy_impl_traits,
            ) {
                continue;
            }

            violations.push(LintResult::new_arch(
                file,
                line_no.value() as usize,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(symbol_str),
                    intent: SymbolName::new(
                        "Use imported symbols in real logic, not only in dummy functions or stubs"
                            .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                        "Imported symbols placed inside _use_ dummy functions are dead code — \
                         they exist only to suppress unused-import warnings. Real business logic \
                         should consume the import directly; otherwise the dependency is misleading \
                         and creates maintenance burden when the import changes."
                            .to_string(),
                    )),
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
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let layer_name = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => "any".to_string(),
        };

        for (start, end) in self.parser.get_dummy_function_ranges(&lines, lang) {
            let start_us = start.value() as usize;
            let _end_us = end.value() as usize;
            violations.push(LintResult::new_arch(
                file,
                start_us,
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
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();

        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let layer_name = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => "any".to_string(),
        };

        for (trait_name, start) in self.parser.get_dummy_impl_traits_with_lines(&lines) {
            let trait_name_str = trait_name.value().to_string();
            let start_us = start.value() as usize;
            violations.push(LintResult::new_arch(
                file,
                start_us,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(trait_name_str),
                    intent: SymbolName::new(
                        "Implement contract methods with real behavior instead of empty, todo stubs"
                            .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                        concat!(
                            "Trait implementations with empty bodies, ",
                            "todo",
                            "!(), or ",
                            "unimplemented",
                            "!() \
                         violate the contract abstraction — the import exists to fulfill a \
                         dependency, but no real behavior is provided. Every method must have \
                         meaningful logic; otherwise the contract becomes untestable and masks \
                         missing functionality."
                        )
                        .to_string(),
                    )),
                }
                .to_string(),
            ));
        }
    }

    fn check_taxonomy_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let _layer_name = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => "any".to_string(),
        };

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

        let dummy_ranges = self.parser.get_dummy_function_ranges(&lines, lang);
        let dummy_impl_traits: Vec<String> = self
            .parser
            .get_dummy_impl_traits_with_lines(&lines)
            .into_iter()
            .map(|(trait_name, _)| trait_name.value().to_string())
            .collect();

        let imported = self.parser.get_imported_symbols(&lines, lang);
        let has_real_usage = imported.iter().any(|(symbol, line_no)| {
            let is_taxonomy = lines
                .get(line_no.value().saturating_sub(1) as usize)
                .is_some_and(|line| {
                    let t = line.trim();
                    match lang {
                        LanguageVO::Rust => {
                            t.contains("use shared::taxonomy_")
                                || t.contains("use output_report::taxonomy_")
                                || t.contains("use crate::common::taxonomy_")
                                || t.contains("use crate::taxonomy_")
                        }
                        LanguageVO::Python => {
                            t.contains("from taxonomy_") || t.contains("from shared.taxonomy_")
                        }
                        LanguageVO::JavaScript => {
                            t.contains("from 'taxonomy_") || t.contains("from \"taxonomy_")
                        }
                        LanguageVO::Unknown => false,
                    }
                });
            if !is_taxonomy {
                return false;
            }
            let symbol_str = symbol.value();
            self.parser
                .is_symbol_used_real(&lines, symbol_str, &dummy_ranges, &dummy_impl_traits)
        });

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
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                            "Taxonomy Value Objects (VO) encode domain concepts with type safety — \
                             using raw primitives (i32, String, bool) in surface-layer signatures \
                             defeats the purpose of the taxonomy layer. VOs ensure consistent \
                             validation, formatting, and semantic meaning across all layers."
                                .to_string(),
                        )),
                    }.to_string(),
                ));
            }
        }
    }

    fn check_layer_contract_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn ILayerDetectionProtocol,
        root_dir: &FilePath,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let file_path = filepath_or_default(FilePath::new(file.to_string()));
        let layer = match analyzer.detect_layer(&file_path, root_dir) {
            Some(l) => l.to_string(),
            None => return,
        };

        let base_layer = match layer.split('(').next() {
            Some(s) => s.trim(),
            None => layer.as_str(),
        };

        let (target_suffix, role_name) = match base_layer {
            "agent" => ("Aggregate", "aggregate contract"),
            "capabilities" => ("Protocol", "protocol contract"),
            "infrastructure" => ("Port", "port contract"),
            "surfaces" => ("Aggregate", "aggregate contract"),
            _ => return,
        };

        let imported = self.parser.get_imported_symbols(&lines, lang);
        let contract_types: Vec<String> = imported
            .into_iter()
            .filter(|(symbol, _)| symbol.value().ends_with(target_suffix))
            .map(|(symbol, _)| symbol.value().to_string())
            .collect();

        let mut type_checking_imports = Vec::new();
        if lang == LanguageVO::Python {
            let mut in_type_checking = false;
            let mut type_checking_indent = 0;
            for line in &lines {
                let trimmed = line.trim();
                if trimmed.is_empty() || trimmed.starts_with('#') {
                    continue;
                }
                let indent = line.len() - line.trim_start().len();
                if in_type_checking {
                    if indent <= type_checking_indent {
                        in_type_checking = false;
                    } else {
                        for c_type in &contract_types {
                            if trimmed.contains(c_type)
                                && (trimmed.starts_with("from ") || trimmed.starts_with("import "))
                            {
                                type_checking_imports.push(c_type.clone());
                            }
                        }
                    }
                }
                if trimmed.starts_with("if TYPE_CHECKING:")
                    || trimmed.starts_with("if typing.TYPE_CHECKING:")
                {
                    in_type_checking = true;
                    type_checking_indent = indent;
                }
            }
        }

        for c_type in &contract_types {
            let is_imported_in_phantom = if lang == LanguageVO::Python {
                type_checking_imports.contains(c_type)
            } else {
                lines.iter().any(|line| {
                    let trimmed = line.trim();
                    let has_phantom = match lang {
                        LanguageVO::Rust => trimmed.contains("PhantomData"),
                        LanguageVO::JavaScript => {
                            trimmed.contains("@ts-ignore") || trimmed.contains("@ts-expect")
                        }
                        _ => false,
                    };
                    has_phantom && trimmed.contains(c_type)
                })
            };

            if (base_layer == "agent"
                || base_layer == "capabilities"
                || base_layer == "infrastructure")
                && lang == LanguageVO::Python
            {
                let inherits = lines
                    .iter()
                    .any(|line| Self::python_class_inherits(line, c_type));

                if !inherits {
                    let class_line = lines
                        .iter()
                        .position(|l| {
                            let t = l.trim();
                            t.starts_with("class ")
                        })
                        .map(|idx| idx + 1)
                        .unwrap_or(1);

                    violations.push(LintResult::new_arch(
                        file,
                        class_line,
                        "AES204",
                        Severity::HIGH,
                        AesImportViolation::ImportIntentViolation {
                            source_layer: LayerNameVO::new(layer.clone()),
                            import_type: SymbolName::new(c_type.to_string()),
                            intent: SymbolName::new(
                                format!(
                                    "Inherit from or implement the {} '{}' in your class definition",
                                    role_name, c_type
                                )
                            ),
                            reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                                format!(
                                    "Classes in the '{}' layer must implement/inherit the contract ('{}') rather than just using it for type annotations. \
                                    This enforces architectural contracts and ensures type safety at runtime.",
                                    layer, c_type
                                )
                            )),
                        }
                        .to_string(),
                    ));
                }
            } else if base_layer == "surfaces" && is_imported_in_phantom {
                let real_usage_count = lines
                    .iter()
                    .filter(|l| {
                        let t = l.trim();
                        t.contains(c_type)
                            && !t.contains("PhantomData")
                            && !t.contains("TYPE_CHECKING")
                            && !t.contains("fn _use_")
                            && !t.starts_with("//")
                            && !t.starts_with("#")
                            && !t.starts_with("use ")
                            && !t.starts_with("pub use ")
                            && !t.starts_with("pub(crate) use ")
                            && !t.starts_with("import ")
                            && !t.starts_with("from ")
                            && !(lang == LanguageVO::Python
                                && (t.contains(&format!("\"{}\"", c_type))
                                    || t.contains(&format!("'{}'", c_type))))
                    })
                    .count();

                if real_usage_count == 0 {
                    let error_line = lines
                        .iter()
                        .position(|l| {
                            let t = l.trim();
                            t.contains(c_type)
                                && (t.contains("PhantomData")
                                    || t.contains("TYPE_CHECKING")
                                    || t.contains("@ts-")
                                    || t.starts_with("from ")
                                    || t.starts_with("import "))
                        })
                        .map(|idx| idx + 1)
                        .unwrap_or(1);

                    violations.push(LintResult::new_arch(
                        file,
                        error_line,
                        "AES204",
                        Severity::HIGH,
                        AesImportViolation::ImportIntentViolation {
                            source_layer: LayerNameVO::new("surfaces".to_string()),
                            import_type: SymbolName::new(c_type.to_string()),
                            intent: SymbolName::new(
                                "Call aggregate functions instead of using PhantomData or TYPE_CHECKING stubs".to_string()
                            ),
                            reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                                "Aggregate types placed only inside PhantomData, TYPE_CHECKING blocks, or type annotations are never instantiated or called. \
                                Aggregates exist to be invoked; using them as mere type markers bypasses the contract layer and hides missing orchestration logic."
                            )),
                    }
                    .to_string(),
                    ));
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
                            reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                                "Surface-layer code must delegate all business logic to the \
                                 aggregate layer — calling domain/analysis functions directly \
                                 from a command/controller bypasses the aggregate abstraction, \
                                 couples I/O handling to domain logic, and makes the system \
                                 harder to test, swap implementations, and evolve independently."
                                    .to_string(),
                            )),
                        }
                        .to_string(),
                    ));
                }
            }
        }
    }
}

impl DummyImportChecker {
    fn python_class_inherits(line: &str, agg_type: &str) -> bool {
        let trimmed = line.trim();
        if !trimmed.starts_with("class ") {
            return false;
        }
        if let Some(open) = trimmed.find('(') {
            if let Some(close) = trimmed.find(')') {
                let bases = &trimmed[open + 1..close];
                return bases.split(',').any(|b| b.trim() == agg_type);
            }
        }
        false
    }
}
