use std::fs;
use std::path::Path;

use crate::code_analysis::contract_constant_protocol::IConstantPurityProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;

pub struct ArchConstantChecker {}

impl ArchConstantChecker {
    pub fn new() -> Self { Self {} }
}

impl IConstantPurityProtocol for ArchConstantChecker {
    fn check_constant_purity(&self, file: &str, violations: &mut Vec<LintResult>) {
        let basename = Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("");

        if !basename.ends_with("_constant.rs") && !basename.ends_with("_constant.py") {
            return;
        }

        if let Ok(content) = fs::read_to_string(file) {
            for (i, line) in content.lines().enumerate() {
                let trimmed = line.trim();
                if trimmed.is_empty()
                    || trimmed.starts_with("//")
                    || trimmed.starts_with('#')
                    || trimmed.starts_with("#[")
                { continue; }

                if trimmed.starts_with("pub const ") || trimmed.starts_with("pub static ") { continue; }
                if trimmed.starts_with("use ") || trimmed.starts_with("pub(crate) use ") { continue; }

                if trimmed.starts_with("pub struct ") || trimmed.starts_with("struct ")
                    || trimmed.starts_with("pub enum ") || trimmed.starts_with("enum ")
                    || trimmed.starts_with("pub fn ") || trimmed.starts_with("fn ")
                    || trimmed.starts_with("impl ") || trimmed.starts_with("pub mod ")
                    || trimmed.starts_with("mod ") || trimmed.starts_with("pub trait ")
                    || trimmed.starts_with("trait ") || trimmed.starts_with("pub use ")
                    || trimmed.starts_with("pub type ") || trimmed.starts_with("type ")
                {
                    let msg = "AES033 CONSTANT_PURITY: Taxonomy _constant file contains non-constant declaration.\n\
                        WHY? _constant files must contain ONLY pub const / pub static declarations.\n\
                        FIX: Move non-constant declarations to the appropriate _vo or _entity file."
                        .to_string();
                    violations.push(LintResult::new_arch(file, i + 1, "AES033", Severity::HIGH, &msg));
                }
            }
        }
    }
}
