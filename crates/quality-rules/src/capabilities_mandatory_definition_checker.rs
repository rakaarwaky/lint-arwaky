use shared_cli_commands::LintResult;
use shared_quality_rules::contract_quality_protocol::IMandatoryClassProtocol;

use shared_common::taxonomy_definition_vo::LayerDefinition;
use shared_common::taxonomy_severity_vo::Severity;
use shared_quality_rules::utility_mandatory_checker::rust_declares_type;

// PURPOSE: MandatoryDefinitionChecker — AES303 sub-check 1: enforce struct/enum/trait/class/interface/type definitions exist.
// ALGORITHM:
//   1. Skip barrel/constant files (mod.rs, __init__.py, _constant.*)
//   2. If no LayerDefinition or mandatory_class_definition disabled → skip
//   3. Check if filename is in exception list
//   4. Scan passed content for class/struct/trait/enum keyword declarations
//   5. If none found → AES303 MANDATORY_DEFINITION
use std::path::Path;

// ─── Block 1: Struct Definition ───────────────────────────

pub struct MandatoryDefinitionChecker {}

// ─── Block 2: Protocol Trait Implementation ───────────────

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

        // Skip barrel files + main.py (single source: shared_common::DEFAULT_RULE_EXCEPTIONS)
        if shared_common::DEFAULT_RULE_EXCEPTIONS.contains(&basename.as_str())
            || basename == "main.py"
            || basename == "py.typed"
            || basename == "main.rs"
        {
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
                || trimmed.starts_with("interface ")
                || trimmed.starts_with("export interface ")
                || trimmed.starts_with("type ")
                || trimmed.starts_with("export type ")
                || rust_declares_type(trimmed)
            {
                has_class = true;
                break;
            }
        }

        if !has_class {
            violations.push(LintResult::new_arch_with_name(
                file,
                0,
                "AES303",
                Severity::HIGH,
                "File is missing a struct, interface, or type definition.",
                "MANDATORY_DEFINITION",
                format!("File {} has no class/struct/enum/trait definition", file),
                "Group functions into a struct or implement an interface.",
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
