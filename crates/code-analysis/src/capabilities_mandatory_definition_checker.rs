
use shared::cli_commands::taxonomy_result_vo::LintResult;
use shared::cli_commands::taxonomy_severity_vo::Severity;
use shared::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::code_analysis::taxonomy_violation_code_analysis_vo::AesCodeAnalysisViolation;
use shared::taxonomy_definition_vo::LayerDefinition;

// PURPOSE: MandatoryDefinitionChecker — AES303: enforce struct/enum/trait/class definitions exist AND are non-empty.
// Sub-check 1: file must define at least one struct/trait/enum/class (IMandatoryClassProtocol).
// Sub-check 2: empty unit struct (struct Foo;) and empty class (class Foo: pass, class Foo {}) flagged as dead inheritance.
// ALGORITHM (check_mandatory_class_definition):
//   1. Skip barrel/constant files (mod.rs, __init__.py, _constant.*)
//   2. If no LayerDefinition or mandatory_class_definition disabled → skip
//   3. Check if filename is in exception list
//   4. Scan passed content for class/struct/trait/enum keyword declarations
//   5. If none found → AES303 MANDATORY_DEFINITION
// ALGORITHM (check_dead_inheritance):
//   1. Iterate lines; skip #[cfg(test)] blocks
//   2. For each `struct Foo;` (unit struct) → flag unless followed by impl block
//   3. For each `class Foo: pass` (Python empty class) → flag
//   4. For each `class Foo {}` (JS/TS empty class) → flag
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MandatoryDefinitionChecker {}

/// AES303 sub-check 2: detect empty struct/impl blocks (dead/empty definitions)
impl IDeadInheritanceProtocol for MandatoryDefinitionChecker {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        let mut in_test_module = false;
        while i < lines.len() {
            let t = lines[i].trim();
            // Skip test modules
            if t.starts_with("#[cfg(test)]") {
                in_test_module = true;
                i += 1;
                continue;
            }
            if in_test_module {
                i += 1;
                continue;
            }
            // Rust: unit struct `struct Foo;` (tuple structs `struct Foo(i32);` excluded)
            if t.starts_with("struct ") && t.ends_with(';') && !t.contains('(') {
                // Skip if followed by impl block or attribute (intentional placeholder)
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
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                }
                i += 1;
                continue;
            }
            // Python: empty class `class Foo: pass` (single line or multi-line)
            if t.starts_with("class ") || t.starts_with("class\t") {
                if t.ends_with(": pass") || t.ends_with(":pass") {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES303",
                        Severity::MEDIUM,
                        AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                } else if t.ends_with(':') && i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next == "pass" || next == "..." || next == "Ellipsis" {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES303",
                            Severity::MEDIUM,
                            AesCodeAnalysisViolation::DeadInheritance { reason: None }.to_string(),
                        ));
                    }
                }
            }
            // JS/TS: empty class `class Foo {}` or `class Foo extends Bar {}`
            if t.starts_with("class ") && t.ends_with("{}") {
                violations.push(LintResult::new_arch(
                    file,
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

// ─── Block 2: Protocol Trait Implementation ───────────────

/// AES303 sub-check 1: file must have at least one struct/enum/trait/class definition
impl IMandatoryClassProtocol for MandatoryDefinitionChecker {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    ) {
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
                || rust_declares_type(trimmed)
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

// ─── Block 3: Constructors, Helpers, Private Methods ──────

impl Default for MandatoryDefinitionChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl MandatoryDefinitionChecker {
    pub fn new() -> Self {
        Self {}
    }
}

/// Helper: check if a line declares a Rust struct/enum/trait (handles visibility modifiers).
fn rust_declares_type(line: &str) -> bool {
    let keywords = ["struct", "enum", "trait"];
    for kw in keywords {
        if line.contains(kw) && !line.contains('(') {
            return true;
        }
    }
    false
}

