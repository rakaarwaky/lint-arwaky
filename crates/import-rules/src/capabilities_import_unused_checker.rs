use shared::common::taxonomy_message_vo::LintMessage;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_severity_vo::Severity;
use shared::common::utility_file_handler;
use shared::import_rules::contract_unused_import_protocol::IUnusedImportProtocol;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::import_rules::{utility_import_resolver, utility_import_symbol_extractor};

// PURPOSE: UnusedImportRuleChecker — AES203: detect unused imports (Rust/Python/JS)
// Uses utility functions directly — no IImportParserProtocol.
use shared::cli_commands::taxonomy_result_vo::LintResult;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct UnusedImportRuleChecker;

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IUnusedImportProtocol for UnusedImportRuleChecker {
    fn find_unused_imports(&self, path: &FilePath) -> Vec<LintMessage> {
        let Some(content) = utility_file_handler::read_file_generic(path.value()).ok() else {
            return vec![];
        };
        let imported_aliases = utility_import_symbol_extractor::extract_imported_aliases(&content);
        let exported_symbols = utility_import_symbol_extractor::extract_exported_symbols(&content);
        let used_symbols =
            utility_import_symbol_extractor::extract_used_symbols(&content, &imported_aliases);

        let mut unused: Vec<String> = Vec::new();
        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                unused.push(alias.value().to_string());
            }
        }
        let rust_js_imports = utility_import_symbol_extractor::extract_rust_js_imports(&content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value();
            if !utility_import_symbol_extractor::is_name_used(
                name_str,
                &content,
                line_idx.value() as usize,
            ) {
                unused.push(name_str.to_string());
            }
        }
        unused.into_iter().map(LintMessage::new).collect()
    }

    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let imported_aliases = utility_import_symbol_extractor::extract_imported_aliases(content);
        let exported_symbols = utility_import_symbol_extractor::extract_exported_symbols(content);
        let used_symbols =
            utility_import_symbol_extractor::extract_used_symbols(content, &imported_aliases);

        for alias in imported_aliases.keys() {
            if !used_symbols.contains(alias) && !exported_symbols.contains(alias) {
                let line_num = utility_import_resolver::find_import_line_number(
                    content,
                    alias.value(),
                )
                .value() as usize;
                violations.push(LintResult::new_arch(
                    file,
                    line_num,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
                            "Import '{}' is declared but never used in this file.",
                            alias
                        ))),
                    }
                    .to_string(),
                ));
            }
        }
        let rust_js_imports = utility_import_symbol_extractor::extract_rust_js_imports(content);
        for (name, line_idx) in rust_js_imports {
            let name_str = name.value().to_string();
            if !utility_import_symbol_extractor::is_name_used(
                &name_str,
                content,
                line_idx.value() as usize,
            ) {
                violations.push(LintResult::new_arch(
                    file,
                    line_idx.value() as usize,
                    "AES203",
                    Severity::MEDIUM,
                    AesImportViolation::FixUnusedImport {
                        reason: Some(LintMessage::new(format!(
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for UnusedImportRuleChecker {
    fn default() -> Self {
        Self
    }
}

impl UnusedImportRuleChecker {
    pub fn new() -> Self {
        Self
    }
}
