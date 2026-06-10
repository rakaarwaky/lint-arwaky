// PURPOSE: MandatoryInheritanceChecker — IMandatoryInheritanceProtocol for AES014: enforce contract implementation
use crate::code_analysis::contract_mandatory_inheritance_protocol::IMandatoryInheritanceProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use std::path::Path;

pub struct MandatoryInheritanceChecker {}

impl Default for MandatoryInheritanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryInheritanceChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IMandatoryInheritanceProtocol for MandatoryInheritanceChecker {
    fn check_mandatory_inheritance(
        &self,
        file: &str,
        content: &str,
        layer: &str,
        violations: &mut Vec<LintResult>,
    ) {
        let contract_suffix = if layer == "infrastructure" || layer.starts_with("infrastructure(") {
            "_port"
        } else if layer == "capabilities" || layer.starts_with("capabilities(") {
            "_protocol"
        } else if layer == "agent" || layer.starts_with("agent(") {
            "_aggregate"
        } else {
            return;
        };

        let filename = Path::new(file)
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");
        let stem = filename.rsplit('.').next_back().unwrap_or(filename);
        let own_suffix = stem.rsplit('_').next().unwrap_or("");
        let implementer_suffixes = [
            "adapter",
            "provider",
            "scanner",
            "client",
            "gateway",
            "repository",
            "connector",
            "cache",
            "loader",
            "writer",
            "reader",
            "driver",
            "analyzer",
            "checker",
            "processor",
            "evaluator",
            "resolver",
            "validator",
            "formatter",
            "executor",
            "transformer",
            "builder",
            "compiler",
            "aggregator",
            "classifier",
            "extractor",
            "reporter",
            "mapper",
            "filter",
            "collector",
            "comparator",
            "scorer",
            "inspector",
            "reviewer",
            "assessor",
            "actions",
        ];
        let is_implementer = implementer_suffixes.contains(&own_suffix);
        if !is_implementer {
            return;
        }

        let mut imported: Vec<String> = Vec::new();

        // Rust: `use ...::ContractName;`
        for line in content.lines() {
            let t = line.trim();
            if t.starts_with("use ") && t.contains(contract_suffix) {
                if let Some(name) = t.split("::").last() {
                    let c = name.trim_end_matches(';').trim();
                    if c.starts_with('I') || c.ends_with("Protocol") || c.ends_with("Port") {
                        imported.push(c.to_string());
                    }
                }
            }
            // Python: `from ...contract_suffix import ContractName` or `from ... import ContractName`
            if t.starts_with("from ") && t.contains(contract_suffix) && t.contains("import ") {
                if let Some(import_part) = t.split("import ").nth(1) {
                    for name in import_part.split(',') {
                        let c = name.trim().split(" as ").next().unwrap_or("").trim();
                        if !c.is_empty()
                            && (c.starts_with('I')
                                || c.ends_with("Protocol")
                                || c.ends_with("Port"))
                        {
                            imported.push(c.to_string());
                        }
                    }
                }
            }
            // JS/TS: `import { ContractName } from '...contract_suffix...'` or `import ContractName from '...'`
            if t.starts_with("import ") && t.contains("from") && t.contains(contract_suffix) {
                // Named imports: `import { Foo, Bar } from '...'`
                if let Some(brace_start) = t.find('{') {
                    if let Some(brace_end) = t.find('}') {
                        let names_part = &t[brace_start + 1..brace_end];
                        for name in names_part.split(',') {
                            let c = name.trim().split(" as ").next().unwrap_or("").trim();
                            if !c.is_empty()
                                && (c.starts_with('I')
                                    || c.ends_with("Protocol")
                                    || c.ends_with("Port"))
                            {
                                imported.push(c.to_string());
                            }
                        }
                    }
                }
                // Default import: `import ContractName from '...'`
                if !t.contains('{') {
                    let after_import = t.trim_start_matches("import ").trim();
                    if let Some(space_pos) = after_import.find(' ') {
                        let c = after_import[..space_pos].trim();
                        if !c.is_empty()
                            && (c.starts_with('I')
                                || c.ends_with("Protocol")
                                || c.ends_with("Port"))
                        {
                            imported.push(c.to_string());
                        }
                    }
                }
            }
        }

        let has_impl = imported
            .iter()
            .any(|t| content.contains(&format!("impl {} for ", t)));
        // Python: `class Y(ContractName):` with non-empty body
        let has_python_impl = imported.iter().any(|t| {
            let pattern = format!("({})", t);
            content.lines().any(|line| {
                let lt = line.trim();
                if lt.starts_with("class ") && lt.contains(&pattern) && lt.ends_with(':') {
                    // Check next non-empty line is not just `pass`
                    // We do a simple heuristic: if the class line itself ends with `: pass` skip,
                    // but here we check if there's a real implementation block
                    true
                } else {
                    false
                }
            })
        });
        // JS/TS: `class Y extends ContractName {` with non-empty body
        let has_js_impl = imported.iter().any(|t| {
            let pattern = format!("extends {} ", t);
            content.lines().any(|line| {
                let lt = line.trim();
                lt.starts_with("class ") && lt.contains(&pattern) && lt.contains('{')
            })
        });

        // For Python, filter out empty classes (class Y(X): pass)
        let has_real_python_impl = if has_python_impl {
            imported.iter().any(|t| {
                let pattern = format!("({})", t);
                let lines: Vec<&str> = content.lines().collect();
                for (idx, line) in lines.iter().enumerate() {
                    let lt = line.trim();
                    if lt.starts_with("class ") && lt.contains(&pattern) && lt.ends_with(':') {
                        // Check if body is only `pass`
                        let class_indent = line.len() - line.trim_start().len();
                        let mut body_lines: Vec<&str> = Vec::new();
                        for next_line in lines.iter().skip(idx + 1) {
                            if next_line.trim().is_empty() {
                                continue;
                            }
                            let next_indent = next_line.len() - next_line.trim_start().len();
                            if next_indent <= class_indent {
                                break;
                            }
                            body_lines.push(next_line.trim());
                        }
                        if body_lines.is_empty()
                            || (body_lines.len() == 1 && body_lines[0] == "pass")
                        {
                            continue; // empty class, not a real impl
                        }
                        return true;
                    }
                }
                false
            })
        } else {
            false
        };

        // For JS, filter out empty classes (class Y extends X {})
        let has_real_js_impl = if has_js_impl {
            imported.iter().any(|t| {
                let pattern = format!("extends {} ", t);
                content.lines().any(|line| {
                    let lt = line.trim();
                    if lt.starts_with("class ") && lt.contains(&pattern) && lt.contains('{') {
                        // Check if body is empty: `class Y extends X {}`
                        if let Some(brace_pos) = lt.find('{') {
                            let after_brace = lt[brace_pos + 1..].trim();
                            if after_brace == "}" {
                                return false; // empty class
                            }
                        }
                        true
                    } else {
                        false
                    }
                })
            })
        } else {
            false
        };

        if !has_impl && !has_real_python_impl && !has_real_js_impl {
            let all_are_deps: bool = imported.iter().all(|t| {
                // Rust dyn
                content.contains(&format!("Arc<dyn {}>", t))
                    || content.contains(&format!("Box<dyn {}>", t))
                    || content.contains(&format!("&dyn {}", t))
                    || content.contains(&format!("&dyn mut {}", t))
                    // Python: injected as parameter type hint or stored as attribute
                    || content.contains(&format!(": {}", t))
                    || content.contains(&format!(": Optional[{}]", t))
                    // JS/TS: typed parameters or properties
                    || content.contains(&format!(": {}", t))
                    || content.contains(&format!(": {} |", t))
                    || content.contains(&format!("| {}", t))
            });
            if !all_are_deps {
                for t in &imported {
                    if !content.contains(&format!("Arc<dyn {}>", t))
                        && !content.contains(&format!("Box<dyn {}>", t))
                        && !content.contains(&format!("&dyn {}", t))
                        && !content.contains(&format!("&dyn mut {}", t))
                        && !content.contains(&format!(": {}", t))
                        && !content.contains(&format!(": Optional[{}]", t))
                        && !content.contains(&format!(": {} |", t))
                        && !content.contains(&format!("| {}", t))
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            0,
                            "AES014",
                            Severity::HIGH,
                            &aes014_mandatory_inheritance(t),
                        ));
                    }
                }
            }
        }

        // Also flag empty inheritance bodies: Python `class Y(X): pass` and JS `class Y extends X {}`
        for t in &imported {
            let pattern_py = format!("({})", t);
            let lines: Vec<&str> = content.lines().collect();
            for (idx, line) in lines.iter().enumerate() {
                let lt = line.trim();
                // Python empty inheritance
                if lt.starts_with("class ") && lt.contains(&pattern_py) && lt.ends_with(':') {
                    let class_indent = line.len() - line.trim_start().len();
                    let mut body_lines: Vec<&str> = Vec::new();
                    for next_line in lines.iter().skip(idx + 1) {
                        if next_line.trim().is_empty() {
                            continue;
                        }
                        let next_indent = next_line.len() - next_line.trim_start().len();
                        if next_indent <= class_indent {
                            break;
                        }
                        body_lines.push(next_line.trim());
                    }
                    if body_lines.is_empty()
                        || (body_lines.len() == 1 && body_lines[0] == "pass")
                    {
                        violations.push(LintResult::new_arch(
                            file,
                            idx + 1,
                            "AES014",
                            Severity::HIGH,
                            &aes014_mandatory_inheritance(t),
                        ));
                    }
                }
                // JS/TS empty inheritance
                let pattern_js = format!("extends {} ", t);
                if lt.starts_with("class ") && lt.contains(&pattern_js) && lt.contains('{') {
                    if let Some(brace_pos) = lt.find('{') {
                        let after_brace = lt[brace_pos + 1..].trim();
                        if after_brace == "}" {
                            violations.push(LintResult::new_arch(
                                file,
                                idx + 1,
                                "AES014",
                                Severity::HIGH,
                                &aes014_mandatory_inheritance(t),
                            ));
                        }
                    }
                }
            }
        }
    }
}

fn aes014_mandatory_inheritance(contracts: &str) -> String {
    format!("AES014 MANDATORY_INHERITANCE: File imports contracts ({}) but no class inherits from them.\nWHY? Layers that import contracts must provide an implementation.\nFIX: Add impl TraitName for YourStruct.", contracts)
}
