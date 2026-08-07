// PURPOSE: IMandatoryClassProtocol — protocol trait for AES303: check that each file has a struct/enum/trait definition
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_lint_result_vo::LintResult;

pub trait IMandatoryClassProtocol: Send + Sync {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        content: &str,
        violations: &mut Vec<LintResult>,
    );
}
