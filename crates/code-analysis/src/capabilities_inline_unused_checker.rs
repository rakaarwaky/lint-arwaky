// PURPOSE: InlineUnusedChecker — IInlineUnusedProtocol for AES023: detect cross-language unused imports (Rust/Python/JS)
use crate::IInlineUnusedProtocol;
use shared::taxonomy_result_vo::LintResult;
use shared::taxonomy_severity_vo::Severity;

pub struct InlineUnusedChecker {}

impl Default for InlineUnusedChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl InlineUnusedChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IInlineUnusedProtocol for InlineUnusedChecker {
    fn check_unused_imports(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
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
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES023",
                    Severity::MEDIUM,
                    aes023_unused_import(name),
                ));
            }
        }
    }
}

fn aes023_unused_import(name: &str) -> String {
    format!("AES023 UNUSED_IMPORT: '{}' imported but never used.", name)
}
