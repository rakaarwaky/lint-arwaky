// PURPOSE: MandatoryDefinitionChecker — AES303: enforce struct/enum/trait/class definitions exist AND are non-empty.
// Sub-check 1: file must define at least one struct/trait/enum/class (IMandatoryClassProtocol).
// Sub-check 2: empty unit struct (struct Foo;) and empty class (class Foo: pass, class Foo {}) flagged as dead inheritance.
use std::path::Path;

use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::common::taxonomy_definition_vo::LayerDefinition;
use shared::common::taxonomy_path_vo::FilePath;
use shared::common::taxonomy_source_vo::ContentString;

// Block 1: struct Definition
pub struct MandatoryDefinitionChecker {}

// Block 2: impl Trait for Struct (Public Contract)
impl IMandatoryClassProtocol for MandatoryDefinitionChecker {
    fn check_mandatory_class_definition(
        &self,
        file: &FilePath,
        definition: Option<&LayerDefinition>,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
    ) {
        let file = file.value();
        let content = content.value();
        let basename = match Path::new(file).file_name().and_then(|f| f.to_str()) {
            Some(name) => name.to_string(),
            None => return,
        };

        if matches!(
            basename.as_str(),
            "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs" | "main.rs"
        ) {
            return;
        }
        if basename.ends_with("_constant.rs") || basename.ends_with("_constant.py") {
            return;
        }

        let def = match definition {
            Some(d) => d,
            None => return,
        };
        if !def.code_analysis.mandatory_class_definition.value {
            return;
        }
        if def.exceptions.values.contains(&basename) {
            return;
        }

        let mut has_class = false;
        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("class ")
                || trimmed.starts_with("export class ")
                || trimmed.starts_with("export default class ")
                || Self::rust_declares_type(trimmed)
            {
                has_class = true;
                break;
            }
        }

        if !has_class {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES303",
                Severity::HIGH,
                AesCodeAnalysisViolation::MandatoryClassDefinition { reason: None }.to_string(),
            ));
        }
    }
}

impl IDeadInheritanceProtocol for MandatoryDefinitionChecker {
    fn check_dead_inheritance(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
    ) {
        let lines: Vec<&str> = content.value.lines().collect();
        let mut i = 0;
        let mut in_test_module = false;
        while i < lines.len() {
            let t = lines[i].trim();
            if t.starts_with("#[cfg(test)]") {
                in_test_module = true;
                i += 1;
                continue;
            }
            if in_test_module {
                i += 1;
                continue;
            }
            if t.starts_with("struct ") && t.ends_with(';') && !t.contains('(') {
                let mut next_idx = i + 1;
                while next_idx < lines.len() {
                    let next_t = lines[next_idx].trim();
                    if next_t.is_empty() || next_t.starts_with('#') {
                        next_idx += 1;
                    } else {
                        break;
                    }
                }
                let next_is_impl = match lines.get(next_idx) {
                    Some(l) => l.trim().starts_with("impl "),
                    None => false,
                };
                if !next_is_impl {
                    violations.push(LintResult::new_arch(
                        file.value(),
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                }
                i += 1;
                continue;
            }
            if t.starts_with("class ") || t.starts_with("class\t") {
                if t.ends_with(": pass") || t.ends_with(":pass") {
                    violations.push(LintResult::new_arch(
                        file.value(),
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                } else if t.ends_with(':') && i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next == "pass" || next == "..." || next == "Ellipsis" {
                        violations.push(LintResult::new_arch(
                            file.value(),
                            i + 1,
                            "AES303",
                            Severity::MEDIUM,
                            AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                        ));
                    }
                }
            }
            if t.starts_with("class ") && t.ends_with("{}") {
                violations.push(LintResult::new_arch(
                    file.value(),
                    i + 1,
                    "AES303",
                    Severity::MEDIUM,
                    AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                ));
            }
            i += 1;
        }
    }
}

// Block 3: Constructors, Std Traits & Helpers
impl Default for MandatoryDefinitionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryDefinitionChecker {
    pub fn new() -> Self {
        Self {}
    }

    pub fn rust_declares_type(line: &str) -> bool {
        let keywords = ["struct", "enum", "trait"];
        for kw in keywords {
            if line.contains(kw) && !line.contains('(') {
                return true;
            }
        }
        false
    }
}
