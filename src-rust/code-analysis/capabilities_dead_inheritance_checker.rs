// PURPOSE: DeadInheritanceChecker — IDeadInheritanceProtocol for AES024: detect empty struct/trait impl blocks
use crate::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;

pub struct DeadInheritanceChecker {}

impl Default for DeadInheritanceChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl DeadInheritanceChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IDeadInheritanceProtocol for DeadInheritanceChecker {
    fn check_dead_inheritance(&self, file: &str, content: &str, violations: &mut Vec<LintResult>) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;
        while i < lines.len() {
            let t = lines[i].trim();
            // Rust: unit struct `struct Foo;`
            if t.starts_with("struct ") && t.ends_with(';') {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES024",
                    Severity::MEDIUM,
                    aes024_dead_inheritance("unit struct"),
                ));
                i += 1;
                continue;
            }
            // Rust: empty impl block `impl X for Y {}`
            if t.starts_with("impl ") {
                let mut impl_str = t.to_string();
                let mut j = i;
                while !impl_str.contains(" for ") && j + 1 < lines.len() {
                    j += 1;
                    impl_str.push_str(lines[j].trim());
                }
                if impl_str.contains(" for ") {
                    if impl_str.trim().ends_with("{}") {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES024",
                            Severity::MEDIUM,
                            aes024_dead_inheritance("impl block"),
                        ));
                    } else {
                        let mut k = j;
                        while k < lines.len() && !impl_str.contains('{') {
                            k += 1;
                            if k < lines.len() {
                                impl_str.push(' ');
                                impl_str.push_str(lines[k].trim());
                            }
                        }
                        if impl_str.trim().ends_with("{}") {
                            violations.push(LintResult::new_arch(
                                file,
                                i + 1,
                                "AES024",
                                Severity::MEDIUM,
                                aes024_dead_inheritance("impl block (multi-line)"),
                            ));
                        }
                    }
                }
            }
            // Python: empty class `class Foo: pass` (single line or multi-line)
            if t.starts_with("class ") || t.starts_with("class\t") {
                if t.ends_with(": pass") || t.ends_with(":pass") {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES024",
                        Severity::MEDIUM,
                        aes024_dead_inheritance("empty class (Python)"),
                    ));
                } else if t.ends_with(':') && i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next == "pass" || next == "..." || next == "Ellipsis" {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES024",
                            Severity::MEDIUM,
                            aes024_dead_inheritance("empty class (Python)"),
                        ));
                    }
                }
            }
            // JS/TS: empty class `class Foo {}` or `class Foo extends Bar {}`
            if t.starts_with("class ") && t.ends_with("{}") {
                violations.push(LintResult::new_arch(
                    file,
                    i + 1,
                    "AES024",
                    Severity::MEDIUM,
                    aes024_dead_inheritance("empty class (JS/TS)"),
                ));
            }
            i += 1;
        }
    }
}

fn aes024_dead_inheritance(type_name: &str) -> String {
    format!(
        "AES024 DEAD_INHERITANCE: Empty struct/trait '{}' detected.",
        type_name
    )
}
