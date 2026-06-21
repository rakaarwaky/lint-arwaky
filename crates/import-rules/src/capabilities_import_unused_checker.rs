// PURPOSE: UnusedImportRuleChecker — IUnusedImportProtocol for AES203: detect imports that are never used in the code (Rust/Python/JS)

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::import_rules::contract_import_parser_port::IImportParserPort;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use std::sync::Arc;

/// Business logic for identifying imports that are not utilized in the code.
pub struct UnusedImportRuleChecker {
    parser: Arc<dyn IImportParserPort>,
}

impl UnusedImportRuleChecker {
    pub fn new(parser: Arc<dyn IImportParserPort>) -> Self {
        Self { parser }
    }
}

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<String> {
        let Ok(content) = self.parser.read_file_to_string(path) else {
            return vec![];
        };

        // 1. Get imported symbols/aliases from the source file
        let imported_aliases = self.parser.extract_imported_aliases(&content);

        // 2. Get exported symbols (like __all__ in Python)
        let exported_symbols = self.parser.extract_exported_symbols(&content);

        // 3. Find which of these imported aliases are actually used in the code
        let used_symbols = self
            .parser
            .extract_used_symbols(&content, &imported_aliases);

        let mut unused = Vec::new();

        // 4. Identify unused Python/standard imports
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias.clone());
            }
        }

        // 5. Handle Rust/JS specific imports
        let rust_js_imports = self.parser.extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            if !self.parser.is_name_used(&name, &content, line_idx) {
                unused.push(name);
            }
        }

        unused
    }

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
                let line_num = self.parser.find_import_line_number(content, alias);
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
            if !self.parser.is_name_used(&name, content, line_idx) {
                violations.push(LintResult::new_arch(
                    file,
                    line_idx + 1,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            name
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
    }
}
