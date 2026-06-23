// PURPOSE: UnusedImportRuleChecker — IUnusedImportProtocol for AES203: detect imports that are never used in the code (Rust/Python/JS)
// AES203 rule: Every import must be used at least once in the file that declares it.
// Detection strategies:
//   - Python/standard imports: extract imported aliases → find used symbols → diff.
//   - Rust/JS imports: extract named imports → check `is_name_used` for each.
//   - Respects __all__ exports (Python) and self-use patterns.

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::common::taxonomy_message_vo::LintMessage;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

/// Identifies imports that are declared but never used in the file (AES203).
///
/// Algorithm:
///   1. Extract all imported aliases/symbols from the source (Python `import X` / `from Y import Z`,
///      Rust `use X::Y`, JS `import X from Y`).
///   2. Extract all used symbols by scanning the file content (call the parser's `extract_used_symbols`).
///   3. If a symbol is exported (e.g., Python `__all__`), it is NOT unused (re-export pattern).
///   4. For Rust/JS: additional extraction of named imports + per-name usage check.
///   5. Each unused import becomes an AES203 MEDIUM violation.
pub struct UnusedImportRuleChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl UnusedImportRuleChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }
}

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    /// Find all unused imports in a file (returns list of symbol names).
    ///
    /// Steps:
    ///   1. Read file content. Return empty if file can't be read.
    ///   2. Extract all imported symbols/aliases (Python standard `import X`, `from Y import Z`).
    ///   3. Extract all exported symbols (Python `__all__`, Rust `pub use`, JS `export`).
    ///   4. Analyze which imported aliases are actually used in the code body.
    ///   5. For each alias: if it's NOT used AND NOT exported → add to unused list.
    ///   6. For Rust/JS named imports (e.g., `use foo::Bar`, `import { Bar }`):
    ///      extract and check each name individually via `is_name_used`.
    ///   7. Return the collected list of unused symbol names.
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        // Step 1: Read file content
        let Ok(content_msg) = self.parser.read_file_to_message(path) else {
            return vec![];
        };
        let content = content_msg.value().to_string();

        // Step 2: Get imported symbols/aliases from the source file
        let imported_aliases = self.parser.extract_imported_aliases(&content);

        // Step 3: Get exported symbols (like __all__ in Python)
        let exported_symbols = self.parser.extract_exported_symbols(&content);

        // Step 4: Find which of these imported aliases are actually used in the code
        let used_symbols = self
            .parser
            .extract_used_symbols(&content, &imported_aliases);

        let mut unused: Vec<String> = Vec::new();

        // Step 5: Identify unused Python/standard imports
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias.value().to_string());
            }
        }

        // Step 6: Handle Rust/JS specific imports
        let rust_js_imports = self.parser.extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value();
            if !self.parser.is_name_used(&name_str, &content, line_idx) {
                unused.push(name_str.to_string());
            }
        }

        // AES402: return VOs, not raw strings.
        unused
            .into_iter()
            .map(shared::taxonomy_message_vo::LintMessage::new)
            .collect()
    }

    /// Check for unused imports and record them as lint violations.
    ///
    /// Steps:
    ///   1. Extract all imported aliases (Python-style imports).
    ///   2. Extract all exported symbols.
    ///   3. Find which aliases are actually used in the code.
    ///   4. For each unused alias (not used, not exported): find its line number and emit MEDIUM violation.
    ///   5. For Rust/JS named imports: check each name and emit MEDIUM violation if unused.
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        // 1. Get imported symbols/aliases from the source file
        let imported_aliases = self.parser.extract_imported_aliases(content);

        // 2. Get exported symbols (like __all__ in Python)
        let exported_symbols = self.parser.extract_exported_symbols(content);

        // 3. Find which of these imported aliases are actually used in the code
        let used_symbols = self.parser.extract_used_symbols(content, &imported_aliases);

        // 4. Identify unused Python/standard imports and record violations
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                let line_num = self
                    .parser
                    .find_import_line_number(content, &alias.value())
                    .value() as usize;
                violations.push(LintResult::new_arch(
                    file,
                    line_num,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            alias
                        ))),
                    }
                    .to_string(),
                ));
            }
        }

        // 5. Handle Rust/JS specific imports and record violations
        let rust_js_imports = self.parser.extract_rust_js_imports(content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value().to_string();
            let line_no_us = line_idx.value() as usize;
            if !self.parser.is_name_used(&name_str, content, line_idx) {
                violations.push(LintResult::new_arch(
                    file,
                    line_no_us,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name_str
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
    }
}
