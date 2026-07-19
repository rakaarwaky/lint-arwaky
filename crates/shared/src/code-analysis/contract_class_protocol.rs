// PURPOSE: IMandatoryClassProtocol — port trait for AES303: check that each file has a struct/enum/trait definition
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;

pub trait IMandatoryClassProtocol: Send + Sync {
    fn check_mandatory_class_definition(
        &self,
        file: &FilePath,
        definition: Option<&LayerDefinition>,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
    );
}
