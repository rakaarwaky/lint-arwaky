use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;
use shared::common::utility_layer_detector;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::taxonomy_language_vo::LanguageVO;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::utility_dummy_detector;
use shared::taxonomy_definition_vo::LayerMapVO;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_message_vo::LintMessage;
use shared::taxonomy_name_vo::SymbolName;

// PURPOSE: DummyImportChecker — AES204: detect dummy imports, dummy functions, dummy trait impls
// Uses utility functions directly — no IImportParserProtocol, no IAnalyzer.

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DummyImportChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDummyImportCheckerProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        Self::_check_dummy_imports(file.value(), content.value(), violations, layer_map);
    }

    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        Self::_check_dummy_functions(file.value(), content.value(), violations, layer_map);
    }

    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        Self::_check_dummy_impls(file.value(), content.value(), violations, layer_map);
    }

    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) {
        Self::_check_taxonomy_intent(file.value(), content.value(), violations, layer_map);
    }

    fn check_layer_contract_intent(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) {
    }

    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) {
        Self::_check_surface_logic(file.value(), content.value(), violations);
    }
}

// ─── Block 3: Constructors, Std Traits & Helpers ─────────
impl Default for DummyImportChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DummyImportChecker {
    pub fn new() -> Self {
        Self
    }

    fn _detect_layer(file: &str, layer_map: &LayerMapVO) -> String {
        let layer_keys: Vec<String> = layer_map.values.keys().map(|k| k.to_string()).collect();
        let filename: &str = utility_layer_detector::extract_filename(file);
        match utility_layer_detector::detect_layer_from_prefix(filename) {
            Some(base) => {
                utility_layer_detector::resolve_specialized_layer(&base, file, &layer_keys)
            }
            None => "any".to_string(),
        }
    }

    fn _check_dummy_imports(
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        layer_map: &LayerMapVO,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang: LanguageVO = LanguageVO::from_path(file);
        let dummy_ranges = utility_dummy_detector::dummy_function_ranges(&lines, lang);
        let dummy_impl_traits: Vec<String> =
            utility_dummy_detector::dummy_impl_traits_with_lines(&lines)
                .into_iter()
                .map(|(t, _)| t.value().to_string())
                .collect();
        let layer_name: String = Self::_detect_layer(file, layer_map);

        for (symbol, line_no) in utility_dummy_detector::imported_symbols(&lines, lang) {
            let symbol_str = symbol.value().to_string();
            if utility_dummy_detector::symbol_used_real(
                &lines,
                &symbol_str,
                &dummy_ranges,
                &dummy_impl_traits,
            ) {
                continue;
            }
            violations.push(LintResult::new_arch(file, line_no.value() as usize, "AES204", Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(symbol_str),
                    intent: SymbolName::new("Use imported symbols in real logic, not only in dummy functions or stubs"),
                    reason: Some(LintMessage::new(
                        "Imported symbols placed inside _use_ dummy functions are dead code — they exist only to suppress unused-import warnings."
                    )),
                }.to_string(),
            ));
        }
    }

    fn _check_dummy_functions(
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        layer_map: &LayerMapVO,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
        let layer_name = Self::_detect_layer(file, layer_map);

        for (start, end) in utility_dummy_detector::dummy_function_ranges(&lines, lang) {
            violations.push(LintResult::new_arch(
                file,
                start.value() as usize,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new("_use_mandatory_imports"),
                    intent: SymbolName::new(
                        "Remove dummy functions that exist only to silence unused import checks",
                    ),
                    reason: Some(LintMessage::new(format!(
                        "Dummy function range ends at line {}",
                        end
                    ))),
                }
                .to_string(),
            ));
        }
    }

    fn _check_dummy_impls(
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        layer_map: &LayerMapVO,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let layer_name = Self::_detect_layer(file, layer_map);

        for (trait_name, start) in utility_dummy_detector::dummy_impl_traits_with_lines(&lines) {
            violations.push(LintResult::new_arch(
                file,
                start.value() as usize,
                "AES204",
                Severity::HIGH,
                AesImportViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new(layer_name.clone()),
                    import_type: SymbolName::new(trait_name.value().to_string()),
                    intent: SymbolName::new(
                        "Implement contract methods with real behavior instead of empty/todo stubs",
                    ),
                    reason: Some(LintMessage::new(
                        "Trait implementations with empty bodies violate the contract abstraction.",
                    )),
                }
                .to_string(),
            ));
        }
    }

    fn _check_taxonomy_intent(
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        layer_map: &LayerMapVO,
    ) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
        let _layer_name = Self::_detect_layer(file, layer_map);
        let dummy_ranges = utility_dummy_detector::dummy_function_ranges(&lines, lang);
        let dummy_impl_traits: Vec<String> =
            utility_dummy_detector::dummy_impl_traits_with_lines(&lines)
                .into_iter()
                .map(|(t, _)| t.value().to_string())
                .collect();
        let imported = utility_dummy_detector::imported_symbols(&lines, lang);

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

        let has_real_usage = imported.iter().any(|(symbol, line_no)| {
            let is_taxonomy = lines
                .get(line_no.value().saturating_sub(1) as usize)
                .is_some_and(|line| {
                    let t = line.trim();
                    match lang {
                        LanguageVO::Rust => {
                            t.contains("use shared::taxonomy_")
                                || t.contains("use shared::common::taxonomy_")
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
            utility_dummy_detector::symbol_used_real(
                &lines,
                symbol.value(),
                &dummy_ranges,
                &dummy_impl_traits,
            )
        });

        if !has_real_usage {
            let has_taxonomy_import = lines.iter().any(|l| {
                let t = l.trim();
                match lang {
                    LanguageVO::Rust => {
                        t.contains("use shared::taxonomy_")
                            || t.contains("use shared::common::taxonomy_")
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
                violations.push(LintResult::new_arch(file, dummy_function_line, "AES204", Severity::HIGH,
                    AesImportViolation::ImportIntentViolation {
                        source_layer: LayerNameVO::new("surfaces"),
                        import_type: SymbolName::new("taxonomy"),
                        intent: SymbolName::new("Use taxonomy Value Objects in function signatures instead of primitives"),
                        reason: Some(LintMessage::new(
                            "Taxonomy VOs encode domain concepts — using raw primitives defeats the purpose."
                        )),
                    }.to_string(),
                ));
            }
        }
    }

    fn _check_surface_logic(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
        let mk = |c: &[char]| c.iter().collect::<String>();
        let logic_patterns = [
            mk(&['l', 'i', 'n', 't', '_', 'p', 'a', 't', 'h', '(']),
            mk(&[
                'c', 'o', 'm', 'p', 'u', 't', 'e', '_', 's', 'c', 'o', 'r', 'e', '(',
            ]),
            mk(&[
                'h', 'a', 's', '_', 'c', 'r', 'i', 't', 'i', 'c', 'a', 'l', '(',
            ]),
            mk(&[
                'w', 'a', 'l', 'k', '_', 'r', 's', '_', 'f', 'i', 'l', 'e', 's', '(',
            ]),
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
                    violations.push(LintResult::new_arch(file, i + 1, "AES204", Severity::MEDIUM,
                        AesImportViolation::ImportIntentViolation {
                            source_layer: LayerNameVO::new("surfaces"),
                            import_type: SymbolName::new(pattern.to_string()),
                            intent: SymbolName::new(format!("Delegate to aggregate instead of calling '{}' directly", pattern)),
                            reason: Some(LintMessage::new(
                                "Surface-layer code must delegate business logic to the aggregate layer."
                            )),
                        }.to_string(),
                    ));
                }
            }
        }
    }
}
