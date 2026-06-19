// PURPOSE: DummyImportChecker — AES204: detect dummy imports, dummy functions, and dummy trait implementations

// Rule ini cek apakah import BENAR-BENAR digunakan sesuai intent.
// Supports: Rust, Python, JavaScript/TypeScript

use async_trait::async_trait;
use shared::cli_commands::taxonomy_result_vo::{LintResult, LintResultList};
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::import_rules::taxonomy_violation_import_vo::AesImportViolation;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Language {
    Rust,
    Python,
    JavaScript,
    Unknown,
}

impl Language {
    fn from_path(path: &str) -> Self {
        let ext = std::path::Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        match ext {
            "rs" => Language::Rust,
            "py" => Language::Python,
            "js" | "ts" | "jsx" | "tsx" => Language::JavaScript,
            _ => Language::Unknown,
        }
    }
}

pub struct DummyImportChecker {}

impl DummyImportChecker {
    pub fn new(
        _parser: Arc<dyn shared::import_rules::contract_import_parser_port::IImportParserPort>,
    ) -> Self {
        Self {}
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
        let lang = Language::from_path(file);

        let dummy_ranges = dummy_function_ranges(&lines, lang);
        let dummy_impl_traits = dummy_impl_traits(&lines);

        // Detect the layer for this file
        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        for (symbol, line_no) in imported_symbols(&lines, lang) {
            if symbol_used_real(&lines, &symbol, &dummy_ranges, &dummy_impl_traits) {
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
        let lang = Language::from_path(file);

        let layer_name = analyzer
            .detect_layer(
                &FilePath::new(file.to_string()).unwrap_or_default(),
                root_dir,
            )
            .map(|l| l.to_string())
            .unwrap_or_else(|| "any".to_string());

        for (start, end) in dummy_function_ranges(&lines, lang) {
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

        for (trait_name, start) in dummy_impl_traits_with_lines(&lines) {
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
        let lang = Language::from_path(file);

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
                Language::Rust => trimmed.starts_with("fn _use_") && trimmed.contains("()"),
                Language::Python => trimmed.starts_with("def _use_") && trimmed.contains("()"),
                Language::JavaScript => {
                    trimmed.starts_with("function _use") && trimmed.contains("()")
                }
                Language::Unknown => false,
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
                Language::Rust => trimmed.starts_with("fn _use_"),
                Language::Python => trimmed.starts_with("def _use_"),
                Language::JavaScript => trimmed.starts_with("function _use"),
                Language::Unknown => false,
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
                Language::Rust => trimmed.starts_with("pub fn ") || trimmed.starts_with("fn "),
                Language::Python => trimmed.starts_with("def "),
                Language::JavaScript => {
                    trimmed.starts_with("function ")
                        || trimmed.starts_with("const ") && trimmed.contains("=>")
                        || trimmed.starts_with("export ")
                }
                Language::Unknown => false,
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
                    Language::Rust => {
                        t.contains("use shared::taxonomy_")
                            || t.contains("use output_report::taxonomy_")
                            || t.contains("use crate::common::taxonomy_")
                            || t.contains("use crate::taxonomy_")
                    }
                    Language::Python => {
                        t.contains("import taxonomy_") || t.contains("from taxonomy_")
                    }
                    Language::JavaScript => {
                        t.contains("from 'taxonomy_") || t.contains("from \"taxonomy_")
                    }
                    Language::Unknown => false,
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
        let lang = Language::from_path(file);

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
                Language::Rust => trimmed.contains("PhantomData"),
                Language::Python => trimmed.contains("TYPE_CHECKING"),
                Language::JavaScript => {
                    trimmed.contains("@ts-ignore") || trimmed.contains("@ts-expect")
                }
                Language::Unknown => false,
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
        let lang = Language::from_path(file);

        let logic_patterns = [
            "lint_path(",
            "compute_score(",
            "has_critical(",
            "walk_rs_files(",
        ];

        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            let is_skip = match lang {
                Language::Rust => trimmed.starts_with("//") || trimmed.starts_with("fn _use_"),
                Language::Python => trimmed.starts_with("#") || trimmed.starts_with("def _use_"),
                Language::JavaScript => {
                    trimmed.starts_with("//") || trimmed.starts_with("function _use")
                }
                Language::Unknown => false,
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

// ─── Language-specific dummy function ranges ───

fn dummy_function_ranges(lines: &[&str], lang: Language) -> Vec<(usize, usize)> {
    match lang {
        Language::Rust => rust_dummy_function_ranges(lines),
        Language::Python => python_dummy_function_ranges(lines),
        Language::JavaScript => js_dummy_function_ranges(lines),
        Language::Unknown => Vec::new(),
    }
}

fn rust_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn _use_") || trimmed.starts_with("fn dummy_") {
            let start = i + 1;
            let mut depth = 0usize;
            let mut end = i + 1;

            for (idx, line) in lines.iter().enumerate().skip(i) {
                let t = line.trim();
                depth = depth.saturating_add(t.matches('{').count());
                depth = depth.saturating_sub(t.matches('}').count());
                end = idx + 1;
                if depth == 0 && t.contains('}') {
                    break;
                }
            }

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

fn python_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("def _use_") || trimmed.starts_with("def dummy_") {
            let start = i + 1;
            let mut end = i + 1;
            let indent = lines[i].len() - lines[i].trim_start().len();

            for (idx, line) in lines.iter().enumerate().skip(i + 1) {
                let t = line.trim();
                if t.is_empty() || t.starts_with('#') {
                    end = idx + 1;
                    continue;
                }
                let line_indent = line.len() - line.trim_start().len();
                if line_indent <= indent && !t.is_empty() {
                    break;
                }
                end = idx + 1;
            }

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

fn js_dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
    let mut ranges = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("function _use")
            || trimmed.starts_with("function dummy")
            || trimmed.starts_with("const _use")
            || trimmed.starts_with("const dummy")
        {
            let start = i + 1;
            let mut depth = 0usize;
            let mut end = i + 1;

            for (idx, line) in lines.iter().enumerate().skip(i) {
                let t = line.trim();
                depth = depth.saturating_add(t.matches('{').count());
                depth = depth.saturating_sub(t.matches('}').count());
                end = idx + 1;
                if depth == 0 && t.contains('}') {
                    break;
                }
            }

            ranges.push((start, end));
            i = end;
        }
        i += 1;
    }

    ranges
}

// ─── Language-specific imported symbols ───

fn imported_symbols(lines: &[&str], lang: Language) -> Vec<(String, usize)> {
    match lang {
        Language::Rust => rust_imported_symbols(lines),
        Language::Python => python_imported_symbols(lines),
        Language::JavaScript => js_imported_symbols(lines),
        Language::Unknown => Vec::new(),
    }
}

fn rust_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") || !trimmed.ends_with(';') {
            continue;
        }

        // Exempt test module patterns: `use super::*;` is standard in #[cfg(test)] modules
        if trimmed == "use super::*;" {
            continue;
        }

        let body = trimmed
            .trim_start_matches("use ")
            .trim_end_matches(';')
            .trim();

        if body.contains('{') {
            if let Some(open) = body.find('{') {
                if let Some(close) = body.rfind('}') {
                    let inside = &body[open + 1..close];
                    for part in inside.split(',') {
                        if let Some(symbol) = rust_imported_symbol_from_part(part.trim()) {
                            symbols.push((symbol, idx + 1));
                        }
                    }
                }
            }
            continue;
        }

        if let Some(symbol) = rust_imported_symbol_from_part(body) {
            symbols.push((symbol, idx + 1));
        }
    }

    symbols
}

fn rust_imported_symbol_from_part(part: &str) -> Option<String> {
    let part = part.trim();
    if part.is_empty() || part == "self" || part.starts_with('*') {
        return None;
    }

    if let Some((_, alias)) = part.split_once(" as ") {
        return Some(alias.trim().to_string());
    }

    let name = part.split("::").last().unwrap_or(part).trim();
    if name.is_empty() || name.contains('{') || name.contains('}') {
        return None;
    }

    Some(name.to_string())
}

fn python_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // `from module import Name1, Name2`
        if trimmed.starts_with("from ") && trimmed.contains(" import ") {
            if let Some(import_part) = trimmed.split_once(" import ").map(|(_, p)| p) {
                for name in import_part.split(',') {
                    let name = name.split_whitespace().next().unwrap_or("");
                    if !name.is_empty() && name != "*" {
                        symbols.push((name.to_string(), idx + 1));
                    }
                }
            }
            continue;
        }

        // `import module_name`
        if trimmed.starts_with("import ") {
            let module = trimmed
                .trim_start_matches("import ")
                .split_whitespace()
                .next()
                .unwrap_or("");
            if !module.is_empty() {
                let name = module.rsplit('.').next().unwrap_or(module);
                symbols.push((name.to_string(), idx + 1));
            }
        }
    }

    symbols
}

fn js_imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();

        // `import { Name1, Name2 } from 'module'`
        if trimmed.starts_with("import ") && trimmed.contains('{') && trimmed.contains("from") {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = part.split_whitespace().next().unwrap_or("");
                        if !name.is_empty() && name != "type" {
                            symbols.push((name.to_string(), idx + 1));
                        }
                    }
                }
            }
            continue;
        }

        // `import Name from 'module'`
        if trimmed.starts_with("import ") && trimmed.contains(" from ") {
            if let Some(import_part) = trimmed.split_once("import ").map(|(_, p)| p) {
                let name = import_part
                    .split_once(" from ")
                    .map(|(n, _)| n)
                    .unwrap_or("");
                let name = name.trim();
                if !name.is_empty() && name != "default" {
                    symbols.push((name.to_string(), idx + 1));
                }
            }
            continue;
        }

        // `const { Name } = require('module')`
        if trimmed.starts_with("const ") && trimmed.contains("require(") && trimmed.contains('{') {
            if let Some(open) = trimmed.find('{') {
                if let Some(close) = trimmed.find('}') {
                    let inside = &trimmed[open + 1..close];
                    for part in inside.split(',') {
                        let name = part.trim().split(':').next().unwrap_or("").trim();
                        if !name.is_empty() {
                            symbols.push((name.to_string(), idx + 1));
                        }
                    }
                }
            }
        }
    }

    symbols
}

// ─── Symbol usage check (language-agnostic) ───

fn symbol_used_real(
    lines: &[&str],
    symbol: &str,
    dummy_ranges: &[(usize, usize)],
    dummy_impl_traits: &[String],
) -> bool {
    // Interface/protocol/trait-like imports are used for method resolution
    // even though their name never appears in the file body (e.g.,
    // PermissionsExt for .permissions(), Parser for Cli::parse()).
    // Match AES203's skip list to avoid false positives.
    if (symbol.starts_with('I')
        && symbol.len() > 1
        && symbol.chars().nth(1).unwrap_or(' ').is_uppercase())
        || symbol.ends_with("Protocol")
        || symbol.ends_with("Port")
        || symbol.ends_with("Trait")
        || symbol.ends_with("Aggregate")
        || symbol.ends_with("Ext")
        // Common Rust traits used implicitly via method calls
        || symbol == "Default"
        || symbol == "Debug"
        || symbol == "Display"
        || symbol == "Clone"
        || symbol == "Copy"
        || symbol == "From"
        || symbol == "Into"
        || symbol == "TryFrom"
        || symbol == "TryInto"
        || symbol == "AsRef"
        || symbol == "AsMut"
        || symbol == "Deref"
        || symbol == "DerefMut"
        || symbol == "Iterator"
        || symbol == "IntoIterator"
        || symbol == "Future"
        || symbol == "Stream"
        || symbol == "Read"
        || symbol == "Write"
        || symbol == "BufRead"
        || symbol == "Seek"
        || symbol == "Hash"
        || symbol == "PartialEq"
        || symbol == "Eq"
        || symbol == "PartialOrd"
        || symbol == "Ord"
        || symbol == "Send"
        || symbol == "Sync"
        || symbol == "Unpin"
        || symbol == "Sized"
        || symbol == "Drop"
        || symbol == "Fn"
        || symbol == "FnMut"
        || symbol == "FnOnce"
        || symbol == "async_trait"
        || symbol == "Parser"
        // Common third-party traits
        || symbol == "Digest"
        || symbol == "Manager"
        || symbol == "Emitter"
        || symbol == "Serialize"
        || symbol == "Deserialize"
    {
        return true;
    }

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_dummy_range(line_no, dummy_ranges)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("import ")
            || trimmed.starts_with("from ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*")
            || trimmed.starts_with("*/")
            || (trimmed.starts_with("#") && !trimmed.starts_with("#["))
            || trimmed.contains("PhantomData")
        {
            continue;
        }

        if !trimmed.contains(symbol) {
            continue;
        }

        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                if dummy_impl_traits.contains(&trait_name) {
                    continue;
                }
            }
        }

        return true;
    }

    false
}

fn in_dummy_range(line_no: usize, ranges: &[(usize, usize)]) -> bool {
    ranges
        .iter()
        .any(|(start, end)| line_no >= *start && line_no <= *end)
}

// ─── Dummy impl detection (Rust only) ───

fn dummy_impl_traits(lines: &[&str]) -> Vec<String> {
    dummy_impl_traits_with_lines(lines)
        .into_iter()
        .map(|(trait_name, _)| trait_name)
        .collect()
}

fn dummy_impl_traits_with_lines(lines: &[&str]) -> Vec<(String, usize)> {
    let mut traits = Vec::new();
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("impl ") && trimmed.contains(" for ") {
            if let Some(trait_name) = impl_trait_name(trimmed) {
                let (end, body_lines) = impl_block(lines, i);
                if trait_impl_is_dummy(&body_lines) {
                    traits.push((trait_name, i + 1));
                }
                i = end;
            } else {
                i += 1;
            }
        } else {
            i += 1;
        }
    }

    traits
}

fn impl_trait_name(line: &str) -> Option<String> {
    let after_impl = line.strip_prefix("impl ")?.trim();
    let (trait_part, _) = after_impl.split_once(" for ")?;
    let trait_name = trait_part.split("::").last().unwrap_or(trait_part).trim();
    if trait_name.is_empty() {
        return None;
    }
    Some(trait_name.to_string())
}

fn impl_block<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn trait_impl_is_dummy(lines: &[&str]) -> bool {
    let mut method_count = 0usize;
    let mut dummy_count = 0usize;
    let mut i = 0usize;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("fn ") || trimmed.starts_with("async fn ") {
            method_count += 1;
            let (end, body) = function_body(lines, i);
            if function_body_is_dummy(&body) {
                dummy_count += 1;
            }
            i = end;
        } else {
            i += 1;
        }
    }

    method_count > 0 && dummy_count == method_count
}

fn function_body<'a>(lines: &'a [&'a str], start: usize) -> (usize, Vec<&'a str>) {
    let mut depth = 0usize;
    let mut body = Vec::new();
    let mut end = start;

    for (idx, line) in lines.iter().enumerate().skip(start) {
        let trimmed = line.trim();
        depth = depth.saturating_add(trimmed.matches('{').count());
        depth = depth.saturating_sub(trimmed.matches('}').count());
        body.push(*line);
        end = idx;
        if depth == 0 && trimmed.contains('}') {
            break;
        }
    }

    (end + 1, body)
}

fn function_body_is_dummy(lines: &[&str]) -> bool {
    // Skip function signature line, only look at body
    let body: String = lines
        .iter()
        .skip(1)
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ");

    let trimmed = body.trim();
    // Truly empty: `{}` or `{ }`
    if trimmed == "{}" || trimmed == "{ }" {
        return true;
    }

    // Short body that's just `{ marker!() }`
    if trimmed.len() < 100 {
        let inner = trimmed.trim_start_matches('{').trim_end_matches('}').trim();
        let short_markers = ["todo!(", "unimplemented!(", "panic!(", "unreachable!("];
        if inner.is_empty() || short_markers.iter().any(|m| inner.starts_with(m)) {
            return true;
        }
    }

    let panic_marker = format!("{}!(", "panic");
    let dummy_markers = [
        "todo!(",
        "unimplemented!(",
        &panic_marker,
        "unreachable!(",
        "return Err(Default::default())",
        "return Ok(Default::default())",
    ];

    dummy_markers.iter().any(|marker| body.contains(marker))
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

            let Ok(content) = std::fs::read_to_string(&f_str) else {
                continue;
            };

            self.check_dummy_imports(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_functions(&f_str, &content, &mut results.values, analyzer, root_dir);
            self.check_dummy_impls(&f_str, &content, &mut results.values, analyzer, root_dir);

            let basename = std::path::Path::new(&f_str)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            let lang = Language::from_path(&f_str);

            let is_surface = match lang {
                Language::Rust => {
                    basename.contains("_command")
                        || basename.contains("_controller")
                        || basename.contains("_handler")
                }
                Language::Python => {
                    basename.contains("command")
                        || basename.contains("controller")
                        || basename.contains("handler")
                }
                Language::JavaScript => {
                    basename.contains("command")
                        || basename.contains("controller")
                        || basename.contains("handler")
                }
                Language::Unknown => false,
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
