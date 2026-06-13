// PURPOSE: DummyImportChecker — AES002X: detect dummy imports, dummy functions, and dummy trait implementations

// Rule ini cek apakah import BENAR-BENAR digunakan sesuai intent.

use async_trait::async_trait;
use shared::import_rules::contract_rule_protocol::IAnalyzer;
use shared::output_report::taxonomy_result_vo::{LintResult, LintResultList};
use shared::output_report::taxonomy_severity_vo::Severity;
use shared::source_parsing::taxonomy_path_vo::FilePath;
use shared::source_parsing::taxonomy_paths_vo::FilePathList;
use shared::taxonomy_layer_vo::{Identity, LayerNameVO};
use shared::taxonomy_name_vo::SymbolName;
use shared::taxonomy_violation_message::AesViolation;
use std::sync::Arc;

pub struct DummyImportChecker {}

impl DummyImportChecker {
    pub fn new(
        _parser: Arc<dyn shared::import_rules::contract_import_parser_port::IImportParserPort>,
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
                l.contains("use shared::taxonomy_") || l.contains("use output_report::taxonomy_")
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

        let logic_patterns = [
            "lint_path(",
            "compute_score(",
            "has_critical(",
            "walk_rs_files(",
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

    fn check_dummy_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let dummy_ranges = dummy_function_ranges(&lines);
        let dummy_impl_traits = dummy_impl_traits(&lines);

        for (symbol, line_no) in imported_symbols(&lines) {
            if symbol_used_real(&lines, &symbol, &dummy_ranges, &dummy_impl_traits) {
                continue;
            }

            violations.push(LintResult::new_arch(
                file,
                line_no,
                "AES002X",
                Severity::HIGH,
                AesViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new("any".to_string()),
                    import_type: SymbolName::new(symbol),
                    intent: SymbolName::new(
                        "Use imported symbols in real logic, not only in dummy functions, PhantomData, or dummy trait impls"
                            .to_string(),
                    ),
                    reason: None,
                },
            ));
        }
    }

    fn check_dummy_functions(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        for (start, end) in dummy_function_ranges(&lines) {
            violations.push(LintResult::new_arch(
                file,
                start,
                "AES002X",
                Severity::HIGH,
                AesViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new("any".to_string()),
                    import_type: SymbolName::new("_use_mandatory_imports".to_string()),
                    intent: SymbolName::new(
                        "Remove dummy functions that exist only to silence unused import checks"
                            .to_string(),
                    ),
                    reason: Some(shared::taxonomy_message_vo::LintMessage::new(format!(
                        "Dummy function range ends at line {}",
                        end
                    ))),
                },
            ));
        }
    }

    fn check_dummy_impls(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        for (trait_name, start) in dummy_impl_traits_with_lines(&lines) {
            violations.push(LintResult::new_arch(
                file,
                start,
                "AES002X",
                Severity::HIGH,
                AesViolation::ImportIntentViolation {
                    source_layer: LayerNameVO::new("any".to_string()),
                    import_type: SymbolName::new(trait_name),
                    intent: SymbolName::new(
                        "Implement contract methods with real behavior instead of empty/todo/panic stubs"
                            .to_string(),
                    ),
                    reason: None,
                },
            ));
        }
    }
}

fn dummy_function_ranges(lines: &[&str]) -> Vec<(usize, usize)> {
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

fn imported_symbols(lines: &[&str]) -> Vec<(String, usize)> {
    let mut symbols = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if !trimmed.starts_with("use ") || !trimmed.ends_with(';') {
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
                        if let Some(symbol) = imported_symbol_from_part(part.trim()) {
                            symbols.push((symbol, idx + 1));
                        }
                    }
                }
            }
            continue;
        }

        if let Some(symbol) = imported_symbol_from_part(body) {
            symbols.push((symbol, idx + 1));
        }
    }

    symbols
}

fn imported_symbol_from_part(part: &str) -> Option<String> {
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

fn symbol_used_real(
    lines: &[&str],
    symbol: &str,
    dummy_ranges: &[(usize, usize)],
    dummy_impl_traits: &[String],
) -> bool {
    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        let trimmed = line.trim();

        if in_dummy_range(line_no, dummy_ranges)
            || trimmed.starts_with("use ")
            || trimmed.starts_with("//")
            || trimmed.starts_with("/*")
            || trimmed.starts_with("*")
            || trimmed.starts_with("*/")
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
    let body = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty() && !line.starts_with("//"))
        .collect::<Vec<_>>()
        .join(" ");

    if body == "{}" || body.contains("{}") && !body.contains("Self::") {
        return true;
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
        Identity::new("AES002X")
    }
}

#[async_trait]
impl shared::import_rules::contract_rule_protocol::IArchImportProtocol for DummyImportChecker {
    async fn check_mandatory_imports(
        &self,
        _analyzer: &dyn IAnalyzer,
        files: &FilePathList,
        _root_dir: &FilePath,
        results: &mut LintResultList,
    ) {
        for f in &files.values {
            let f_str = f.to_string();

            let Ok(content) = std::fs::read_to_string(&f_str) else {
                continue;
            };

            self.check_dummy_imports(&f_str, &content, &mut results.values);
            self.check_dummy_functions(&f_str, &content, &mut results.values);
            self.check_dummy_impls(&f_str, &content, &mut results.values);

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
}
