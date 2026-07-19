// PURPOSE: ILineCheckerProtocol — port trait for AES301/AES302: check file line count limits
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_definition_vo::LayerDefinition;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;

pub trait ILineCheckerProtocol: Send + Sync {
    fn check_line_counts(
        &self,
        file: &FilePath,
        definition: Option<&LayerDefinition>,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
    );
}
