// PURPOSE: DeadInheritanceChecker — IDeadInheritanceProtocol for AES024: detect empty struct/trait impl blocks
use shared::code_analysis::contract_dead_inheritance_protocol::IDeadInheritanceProtocol;
use shared::common::taxonomy_violation_message::AesViolation;
use shared::output_report::taxonomy_result_vo::LintResult;
use shared::output_report::taxonomy_severity_vo::Severity;

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
            // Rust: unit struct `struct Foo;`
            if t.starts_with("struct ") && t.ends_with(';') {
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
                let next_is_impl = lines.get(next_idx).map(|l| l.trim().starts_with("impl ")).unwrap_or(false);
                if !next_is_impl {
                    violations.push(LintResult::new_arch(
                        file,
                        i + 1,
                        "AES024",
                        Severity::MEDIUM,
                        AesViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                }
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
                            AesViolation::DeadInheritance { reason: None }.to_string(),
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
                                AesViolation::DeadInheritance { reason: None }.to_string(),
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
                        AesViolation::DeadInheritance { reason: None }.to_string(),
                    ));
                } else if t.ends_with(':') && i + 1 < lines.len() {
                    let next = lines[i + 1].trim();
                    if next == "pass" || next == "..." || next == "Ellipsis" {
                        violations.push(LintResult::new_arch(
                            file,
                            i + 1,
                            "AES024",
                            Severity::MEDIUM,
                            AesViolation::DeadInheritance { reason: None }.to_string(),
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
                    AesViolation::DeadInheritance { reason: None }.to_string(),
                ));
            }
            i += 1;
        }
    }
}
