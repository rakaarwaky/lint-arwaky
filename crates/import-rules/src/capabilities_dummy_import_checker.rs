use shared::cli_commands::LintResult;
use shared::common::taxonomy_definition_vo::LayerMapVO;
use shared::common::{ContentString, FilePath, Identity, LanguageVO, LineNumber, Severity};
use shared::filesystem::taxonomy_filesystem_vo::ImportEntry;

use crate::utility_dummy_detector;
use crate::utility_import_resolver;
use shared::import_rules::contract_dummy_import_protocol::IDummyImportCheckerProtocol;
use shared::import_rules::taxonomy_import_error::ImportError;

// PURPOSE: DummyImportChecker — AES204: detect dummy imports, dummy functions, dummy trait impls

// ─── Block 1: Struct Definition ───────────────────────────

pub struct DummyImportChecker;

impl DummyImportChecker {}

struct DummyFileContext {
    lines: Vec<String>,
    lang: LanguageVO,
    dummy_ranges: Vec<(LineNumber, LineNumber)>,
    dummy_impl_traits: Vec<String>,
}

impl DummyFileContext {
    fn compute(file: &str, content: &str, _layer_map: &LayerMapVO) -> Option<Self> {
        let basename = std::path::Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        if utility_import_resolver::is_barrel_file(basename) {
            return None;
        }
        let lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();
        let str_refs: Vec<&str> = lines.iter().map(|s| s.as_str()).collect();
        let lang = LanguageVO::from_path(file);
        let dummy_ranges = utility_dummy_detector::dummy_function_ranges(&str_refs, lang);
        let dummy_impl_traits: Vec<String> =
            utility_dummy_detector::dummy_impl_traits_with_lines(&str_refs)
                .into_iter()
                .map(|(t, _)| t.value().to_string())
                .collect();
        Some(Self {
            lines,
            lang,
            dummy_ranges,
            dummy_impl_traits,
        })
    }

    fn str_refs(&self) -> Vec<&str> {
        self.lines.iter().map(|s| s.as_str()).collect()
    }
}

// ─── Block 2: Protocol Trait Implementation ───────────────

impl IDummyImportCheckerProtocol for DummyImportChecker {
    fn rule_name(&self) -> Identity {
        Identity::new("AES204")
    }

    fn check_dummy_imports(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
        import_entries: &[ImportEntry],
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_dummy_imports(
            file.value(),
            &ctx,
            &mut violations,
            layer_map,
            import_entries,
        );
        Ok(violations)
    }

    fn check_dummy_functions(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_dummy_functions(file.value(), &ctx, &mut violations);
        Ok(violations)
    }

    fn check_dummy_impls(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_dummy_impls(file.value(), &ctx, &mut violations);
        Ok(violations)
    }

    fn check_taxonomy_intent(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
        import_entries: &[ImportEntry],
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let mut violations = Vec::new();
        Self::_check_taxonomy_intent(file.value(), &ctx, &mut violations, import_entries);
        Ok(violations)
    }

    // TODO(P0): Implement layer-contract-intent check per FRD specification.
    // This method is called by check_all_dummy (shared contract).
    // Current state: no-op — contract-intent violations are silently dropped.
    fn check_layer_contract_intent(
        &self,
        _file: &FilePath,
        _content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        Ok(Vec::new())
    }

    fn check_surface_logic(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        _layer_map: &LayerMapVO,
    ) -> Result<Vec<LintResult>, ImportError> {
        let mut violations = Vec::new();
        Self::_check_surface_logic(file.value(), content.value(), &mut violations);
        Ok(violations)
    }

    fn check_all_dummy(
        &self,
        file: &FilePath,
        content: &ContentString,
        _root_dir: &FilePath,
        layer_map: &LayerMapVO,
        imports_map: &std::collections::HashMap<String, Vec<ImportEntry>>,
    ) -> Result<Vec<LintResult>, ImportError> {
        let Some(ctx) = DummyFileContext::compute(file.value(), content.value(), layer_map) else {
            return Ok(Vec::new());
        };
        let import_entries = imports_map
            .get(file.value())
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
        let mut violations = Vec::new();
        Self::_check_dummy_imports(
            file.value(),
            &ctx,
            &mut violations,
            layer_map,
            import_entries,
        );
        Self::_check_dummy_functions(file.value(), &ctx, &mut violations);
        Self::_check_dummy_impls(file.value(), &ctx, &mut violations);
        Self::_check_taxonomy_intent(file.value(), &ctx, &mut violations, import_entries);
        Self::_check_surface_logic(file.value(), content.value(), &mut violations);
        Ok(violations)
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

    fn _check_dummy_imports(
        file: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
        _layer_map: &LayerMapVO,
        import_entries: &[ImportEntry],
    ) {
        // No dummy functions or trait impls in this file — nothing to flag as AES204.
        if ctx.dummy_ranges.is_empty() && ctx.dummy_impl_traits.is_empty() {
            return;
        }
        let lines = ctx.str_refs();
        // Use ImportEntry from filesystem's AST parser
        let imported = utility_dummy_detector::imported_symbols_from_entries(import_entries);

        for (symbol, line_no) in imported {
            let symbol_str = symbol.value().to_string();
            if is_future_import(&lines, &symbol_str) {
                continue;
            }
            if utility_dummy_detector::symbol_used_real(
                &lines,
                &symbol_str,
                &ctx.dummy_ranges,
                &ctx.dummy_impl_traits,
            ) {
                continue;
            }
            violations.push(LintResult::new_arch(file, line_no.value() as usize, "AES204", Severity::HIGH,
                "AES201 IMPORT_VIOLATION: Import intent mismatch.\n\
                     WHY? Imported symbols placed inside _use_ dummy functions are dead code — they exist only to suppress unused-import warnings.\n\
                     FIX: Ensure imports match the file's layer intent.".to_string(),
            ));
        }
    }

    fn _check_dummy_functions(
        file: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
    ) {
        for (start, end) in &ctx.dummy_ranges {
            violations.push(LintResult::new_arch(
                file,
                start.value() as usize,
                "AES204",
                Severity::HIGH,
                format!(
                    "AES201 IMPORT_VIOLATION: Import intent mismatch.\n\
                     WHY? Dummy function range ends at line {}\n\
                     FIX: Ensure imports match the file's layer intent.",
                    end
                ),
            ));
        }
    }

    fn _check_dummy_impls(file: &str, ctx: &DummyFileContext, violations: &mut Vec<LintResult>) {
        let lines = ctx.str_refs();
        for (_trait_name, start) in utility_dummy_detector::dummy_impl_traits_with_lines(&lines) {
            violations.push(LintResult::new_arch(
                file,
                start.value() as usize,
                "AES204",
                Severity::HIGH,
                "AES201 IMPORT_VIOLATION: Import intent mismatch.\n\
                     WHY? Trait implementations with empty bodies violate the contract abstraction.\n\
                     FIX: Ensure imports match the file's layer intent.".to_string(),
            ));
        }
    }

    fn _check_taxonomy_intent(
        file: &str,
        ctx: &DummyFileContext,
        violations: &mut Vec<LintResult>,
        import_entries: &[ImportEntry],
    ) {
        let lines = ctx.str_refs();
        // Use ImportEntry from filesystem's AST parser
        let imported = utility_dummy_detector::imported_symbols_from_entries(import_entries);

        let mut has_dummy_function = false;
        let mut dummy_function_line = 0;
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_dummy = match ctx.lang {
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
                    match ctx.lang {
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
                &ctx.dummy_ranges,
                &ctx.dummy_impl_traits,
            )
        });

        if !has_real_usage {
            let has_taxonomy_import = lines.iter().any(|l| {
                let t = l.trim();
                match ctx.lang {
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
                    "AES201 IMPORT_VIOLATION: Import intent mismatch.\n\
                         WHY? Taxonomy VOs encode domain concepts — using raw primitives defeats the purpose.\n\
                         FIX: Ensure imports match the file's layer intent.".to_string(),
                ));
            }
        }
    }

    fn _check_surface_logic(file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let lang = LanguageVO::from_path(file);
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
                let is_string_lit = trimmed.contains(&format!("\"{}", pattern))
                    || trimmed.contains(&format!("'{}", pattern));
                if trimmed.contains(pattern) && !is_string_lit {
                    violations.push(LintResult::new_arch(file, i + 1, "AES204", Severity::MEDIUM,
                        "AES201 IMPORT_VIOLATION: Import intent mismatch.\n\
                             WHY? Surface-layer code must delegate business logic to the aggregate layer.\n\
                             FIX: Ensure imports match the file's layer intent.".to_string(),
                    ));
                }
            }
        }
    }
}

/// Check if any line matches `from __future__ import ...symbol...`.
fn is_future_import(lines: &[&str], symbol: &str) -> bool {
    let content = lines.join("\n");
    utility_import_resolver::is_future_import(&content, symbol)
}
