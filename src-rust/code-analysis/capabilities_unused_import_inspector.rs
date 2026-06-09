//! Inline unused import check.

use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_violationrs_constant::aes023_unused_import;

/// Create a LintResult — shared by all inline checkers.
fn mk_result(file: &str, line: usize, code: &str, sev: Severity, msg: &str) -> LintResult {
    LintResult::new_arch(file, line, code, sev, msg)
}

/// Check for imported names that are never referenced in the file body.
/// Handles `use` (Rust), `import` (JS/TS), and `from ... import` (Python).
/// Skips: std/core/alloc, trait-like imports (I*Port, *Protocol, etc.),
/// glob imports (`*`), and underscore imports (`_`).
pub fn check_unused_imports(file: &str, content: &str, violations: &mut Vec<LintResult>) {
    for (i, line) in content.lines().enumerate() {
        let t = line.trim();

        let names: Vec<String> = if t.starts_with("use ") {
            let target = t.trim_end_matches(';').trim_start_matches("use ").trim();
            if target.starts_with("std::")
                || target.starts_with("core::")
                || target.starts_with("alloc::")
            {
                continue;
            }
            if let Some(brace_pos) = target.find("::{") {
                let inner = target[brace_pos + 3..].trim_end_matches('}').trim();
                inner
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .split(" as ")
                            .last()
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    })
                    .filter(|n| !n.is_empty() && n != "_")
                    .collect()
            } else {
                let name = target
                    .split("::")
                    .last()
                    .unwrap_or("")
                    .split(" as ")
                    .last()
                    .unwrap_or("")
                    .trim()
                    .to_string();
                if name.is_empty() || name == "_" || name == "*" {
                    continue;
                }
                vec![name]
            }
        } else if t.starts_with("import ") {
            let name = t
                .trim_start_matches("import ")
                .split_whitespace()
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if name.is_empty() || name == "_" {
                continue;
            }
            vec![name]
        } else if t.starts_with("from ") {
            let after_from = t.trim_start_matches("from ");
            let module = after_from.split_whitespace().next().unwrap_or("");
            if module.is_empty() {
                continue;
            }
            if let Some(import_pos) = after_from.find(" import ") {
                let names_part = after_from[import_pos + 8..].trim();
                let extracted: Vec<String> = names_part
                    .split(',')
                    .map(|s| {
                        s.trim()
                            .split(" as ")
                            .last()
                            .unwrap_or("")
                            .trim()
                            .to_string()
                    })
                    .filter(|n| !n.is_empty() && n != "_")
                    .collect();
                if extracted.is_empty() {
                    vec![module.to_string()]
                } else {
                    extracted
                }
            } else {
                continue;
            }
        } else {
            continue;
        };

        for name in &names {
            // Skip trait imports (start with 'I' or end with common trait suffixes)
            if (name.starts_with('I')
                && name.len() > 1
                && name.chars().nth(1).unwrap_or(' ').is_uppercase())
                || name.ends_with("Protocol")
                || name.ends_with("Port")
                || name.ends_with("Trait")
                || name.ends_with("Aggregate")
                || name == "Parser"
            {
                continue;
            }
            let rest = content
                .lines()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, l)| l)
                .collect::<Vec<_>>()
                .join("\n");
            if rest.contains(name) {
                continue;
            }
            violations.push(mk_result(
                file,
                i + 1,
                "AES023",
                Severity::MEDIUM,
                &aes023_unused_import(name),
            ));
        }
    }
}
