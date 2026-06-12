// PURPOSE: IMandatoryClassProtocol — port trait for AES011: check that each file has a struct/enum/trait definition
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::output_report::taxonomy_result_vo::LintResult;

pub trait IMandatoryClassProtocol: Send + Sync {
    fn check_mandatory_class_definition(
        &self,
        file: &str,
        definition: Option<&LayerDefinition>,
        violations: &mut Vec<LintResult>,
    );
}
