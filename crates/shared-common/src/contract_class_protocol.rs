// PURPOSE: IMandatoryClassProtocol — port trait for AES011: check that each file has a struct/enum/trait definition
use output_report::taxonomy_result_vo::LintResult;
use shared_common::taxonomy_definition_vo::LayerDefinition;

pub trait IMandatoryClassProtocol: Send + Sync {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    );
}
