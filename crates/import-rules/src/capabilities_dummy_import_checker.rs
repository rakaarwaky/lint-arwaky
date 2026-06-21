// PURPOSE: DummyImportChecker — AES204: detect dummy imports, dummy functions, and dummy trait implementations
// AES204 rule: Symbols imported solely to silence unused-import warnings (via dummy/stub functions
// or PhantomData markers) are violations. Additionally, surface-layer files must use taxonomy VOs
// in real function signatures and must not phantom-reference aggregate types without calling them.

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
use std::collections::HashSet;
use std::sync::Arc;

/// Checks AES204 rules: dummy imports, dummy functions, dummy trait impls,
/// taxonomy intent violations, aggregate phantom usage, and surface-layer logic bypass.
///
/// Workflow:
///   1. `check_dummy_imports` — Parse all imported symbols; skip those used in real (non-dummy) code;
///      flag the rest as dummy-only imports (they exist only to silence unused-import warnings).
///   2. `check_dummy_functions` — Find functions named _use_* (e.g. `fn _use_imports()`) that exist
///      solely to consume imported symbols; flag each as a dummy function violation.
///   3. `check_dummy_impls` — Find trait implementations that are stubs (empty body);
///      flag each as a dummy impl violation.
///   4. `check_taxonomy_intent` — For surface-layer files: if a dummy function exists but the real
///      function signatures use only primitive types (i32, String, bool) instead of taxonomy VOs,
///      flag as taxonomy intent violation.
///   5. `check_aggregate_intent` — For surface-layer files: if aggregate types appear only inside
///      PhantomData (never called in real code), flag each as aggregate intent violation.
///   6. `check_surface_logic` — For surface-layer files: if business logic (lint_path, compute_score)
///      is called directly instead of being delegated to the aggregate, flag each occurrence.
pub struct DummyImportChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl DummyImportChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }

    /// Sub-check 1: Detect symbols imported but only used inside dummy functions or stub impls.
    ///
    /// Steps:
    ///   1. Split content into lines and detect the programming language.
    ///   2. Get all dummy function line ranges (functions named _use_*).
    ///   3. Get all dummy trait impls (empty/todo stub implementations).
    ///   4. Detect the file's architectural layer.
    ///   5. Extract every imported symbol with its line number.
    ///   6. For each symbol, check if it is used in real (non-dummy, non-stub) code.
    ///   7. If it's only used in dummy/stub contexts, flag it as AES204 HIGH violation.
    fn check_dummy_imports(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        // Step 1: Split content into lines and detect language
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        // Step 2: Find all dummy function ranges (fn/def/function _use_*)
        let dummy_ranges = self.parser.get_dummy_function_ranges(&lines, lang);
        // Step 3: Find all dummy/stub trait implementations
        let dummy_impl_traits: Vec<String> = self
            .parser
            .get_dummy_impl_traits_with_lines(&lines)
            .into_iter()
            .map(|(trait_name, _)| trait_name)
            .collect();

        // Step 4: Detect the architectural layer for this file
        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        // Step 5-7: Iterate imported symbols and check if they have real usage
        for (symbol, line_no) in self.parser.get_imported_symbols(&lines, lang) {
            // Step 6: Skip symbols that are actually used outside dummy/stub contexts
            if self
                .parser
                .is_symbol_used_real(&lines, &symbol, &dummy_ranges, &dummy_impl_traits)
            {
                continue;
            }

            // Step 7: Symbol is only used in dummy/stub — flag as violation
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

    /// Sub-check 2: Detect dummy functions (named _use_*) that serve only to suppress unused-import warnings.
    ///
    /// Steps:
    ///   1. Split content into lines and detect language.
    ///   2. Detect the file's architectural layer.
    ///   3. Find all dummy function ranges via parser.
    ///   4. For each dummy function, emit a violation with its start line and end line info.
    fn check_dummy_functions(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        // Step 1: Parse lines and detect language
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        // Step 2: Detect file layer
        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        // Step 3-4: Flag each dummy function as violation
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

    /// Sub-check 3: Detect trait implementations that are stubs (empty body).
    ///
    /// Steps:
    ///   1. Split content into lines.
    ///   2. Detect the file's architectural layer.
    ///   3. Use parser to find all dummy/stub trait impls (e.g. empty functions).
    ///   4. Flag each as an AES204 HIGH violation — contract methods must have real behavior.
    fn check_dummy_impls(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        // Step 1: Split content into lines
        let lines: Vec<&str> = content.lines().collect();

        // Step 2: Detect file layer
        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        // Step 3-4: Flag each dummy/stub trait implementation
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
                        "Implement contract methods with real behavior instead of empty/todo stubs"
                            .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                        "Trait implementations with empty bodies, todo!(), or unimplemented!() \
                         violate the contract abstraction — the import exists to fulfill a \
                         dependency, but no real behavior is provided. Every method must have \
                         meaningful logic; otherwise the contract becomes untestable and masks \
                         missing functionality."
                            .to_string(),
                    )),
                }
                .to_string(),
            ));
        }
    }

    /// Sub-check 4: Verify taxonomy VO imports are used in real function signatures (not only in dummy functions).
    ///
    /// Steps:
    ///   1. Parse lines and detect language. Detect file layer.
    ///   2. Scan for the presence of a dummy function (_use_*) — if none, skip (no violation possible).
    ///   3. Walk all lines, skipping dummy function bodies via brace counting.
    ///   4. In non-dummy function signatures, check if taxonomy primitives (LineNumber, Score, etc.)
    ///      appear — if at least one real function uses them, the intent is satisfied.
    ///   5. If no real function uses taxonomy primitives but a taxonomy import exists, the file
    ///      imports taxonomy VOs but only uses them inside dummy functions → violation.
    fn check_taxonomy_intent(
        &self,
        file: &str,
        content: &str,
        violations: &mut Vec<LintResult>,
        analyzer: &dyn IAnalyzer,
        root_dir: &FilePath,
    ) {
        // Step 1: Parse lines, detect language and layer
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        let _layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        // Step 2: Check if file has a dummy function — essential precondition
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

        // No dummy function → no intent violation possible
        if !has_dummy_function {
            return;
        }

        // Step 3: Define the "taxonomy primitive" types that should be replaced by VOs
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

        // Step 4: Walk all lines skipping dummy function bodies
        let mut has_real_usage = false;
        let mut in_dummy_function = false;
        let mut brace_count = 0;

        for line in &lines {
            let trimmed = line.trim();

            // Track when we enter/exit a dummy function body
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

            // Skip lines inside dummy function bodies (brace counting)
            if in_dummy_function {
                brace_count += trimmed.matches('{').count();
                brace_count = brace_count.saturating_sub(trimmed.matches('}').count());
                if brace_count == 0 && trimmed.contains('}') {
                    in_dummy_function = false;
                }
                continue;
            }

            // Check if real function signatures use taxonomy primitives
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

        // Step 5: If no real function uses taxonomy primitives but taxonomy imports exist → violation
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

    /// Sub-check 5: Detect aggregate types used only as PhantomData (never called in real code).
    ///
    /// Steps:
    ///   1. Split content into lines and detect language.
    ///   2. Define known aggregate types (DevCommandsAggregate, etc.).
    ///   3. For each line, check if it contains a phantom marker (PhantomData, TYPE_CHECKING, @ts-ignore)
    ///      combined with an aggregate type name.
    ///   4. Count how many times the aggregate type appears outside phantom/dummy/comment contexts.
    ///   5. If count == 0, the aggregate is imported only as PhantomData → violation.
    fn check_aggregate_intent(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        // Step 2: Known aggregate types that should be called, not phantom-referenced
        let aggregate_types = [
            "DevCommandsAggregate",
            "LintFixOrchestratorAggregate",
            "PluginCommandsAggregate",
            "MaintenanceCommandsAggregate",
            "GitCommandsAggregate",
        ];

        // Step 3-5: Scan lines for phantom + aggregate type combinations
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
                        // Step 4: Count real (non-phantom, non-dummy, non-comment) usages
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

                        // Step 5: Zero real usage → PhantomData-only → violation
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
                                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(
                                        "Aggregate types placed only inside PhantomData{} are \
                                         never instantiated or called — the import is effectively \
                                         dead code. Aggregates exist to be invoked; using them as \
                                         mere type markers bypasses the contract layer and hides \
                                         missing orchestration logic."
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
    }

    /// Sub-check 6: Detect surface-layer files calling business logic directly (bypassing the aggregate layer).
    ///
    /// Steps:
    ///   1. Split content into lines and detect language.
    ///   2. Define known logic function patterns that should only be called from aggregates.
    ///   3. For each line, skip comments and dummy functions.
    ///   4. If a non-skipped line contains a logic pattern, flag as MEDIUM violation —
    ///      surface code should delegate to aggregates, not call logic directly.
    fn check_surface_logic(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = self.parser.get_language_from_path(file);

        // Step 2: Functions that belong in the aggregate layer, not in surfaces
        let logic_patterns = [
            "lint_path(",
            "compute_score(",
            "has_critical(",
            "walk_rs_files(",
        ];

        // Step 3-4: Scan for logic bypass
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            // Skip comments and dummy functions
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

#[async_trait]
impl shared::import_rules::contract_rule_protocol::IArchRuleProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }
}

#[async_trait]
impl shared::import_rules::contract_rule_protocol::IArchImportProtocol for DummyImportChecker {
    /// Run all AES204 sub-checks on every file.
    ///
    /// Steps:
    ///   1. Iterate all files in the project.
    ///   2. Skip the checker's own file (contains unavoidable violation message strings).
    ///   3. Read file content.
    ///   4. Run sub-checks 1-3 (dummy imports, dummy functions, dummy impls) on every file.
    ///   5. Detect if the file is a surface-layer file (command/controller/handler).
    ///   6. Only for surface files: run sub-checks 4-6 (taxonomy intent, aggregate intent, surface logic).
    async fn check_mandatory_imports(
        &self,
        analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();
            // Skip self-check — this file contains hardcoded violation message strings
            if f_str.contains("capabilities_dummy_import_checker") {
                continue;
            }

            // Step 3: Read file content
            let Ok(content) = self.parser.read_file_to_string(f) else {
                continue;
            };

            // Step 4: Run universal sub-checks (every file type)
            self.check_dummy_imports(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_functions(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_impls(&f_str, &content, &mut results.values, analyzer, root_dir);

            // Step 5: Detect if this is a surface-layer file
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

            // Step 6: Surface-only sub-checks
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
