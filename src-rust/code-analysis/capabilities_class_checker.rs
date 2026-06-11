// PURPOSE: ArchClassChecker — IMandatoryClassProtocol for AES024: enforce struct/enum/trait definitions per file
use std::fs;
use std::path::Path;

use crate::code_analysis::contract_class_protocol::IMandatoryClassProtocol;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::output_report::taxonomy_severity_vo::Severity;
use crate::shared_common::taxonomy_definition_vo::LayerDefinition;
use crate::shared_common::taxonomy_violation_message_rs_error::AesViolation;

pub struct ArchClassChecker {}

impl Default for ArchClassChecker {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchClassChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl IMandatoryClassProtocol for ArchClassChecker {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    ) {
        let basename = Path::new(file)
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("")
            .to_string();

        if matches!(
            basename.as_str(),
            "__init__.py" | "main.py" | "py.typed" | "mod.rs" | "lib.rs"
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
        if !def.mandatory_class_definition.value {
            return;
        }
        if def.exceptions.values.contains(&basename) {
            return;
        }

        let has_class = if let Ok(content) = fs::read_to_string(file) {
            content.contains("\nclass ")
                || content.starts_with("class ")
                || content.contains("\npub struct ")
                || content.starts_with("pub struct ")
                || content.contains("\nstruct ")
                || content.starts_with("struct ")
                || content.contains("\npub trait ")
                || content.starts_with("pub trait ")
                || content.contains("\ntrait ")
                || content.starts_with("trait ")
                || content.contains("\npub enum ")
                || content.starts_with("pub enum ")
                || content.contains("\nenum ")
                || content.starts_with("enum ")
                || content.contains("\nexport class ")
                || content.contains("\nexport default class ")
        } else {
            false
        };

        if !has_class {
            violations.push(LintResult::new_arch(
                file,
                0,
                "AES024",
                Severity::HIGH,
                AesViolation::MandatoryClassDefinition { reason: None },
            ));
        }
    }
}
