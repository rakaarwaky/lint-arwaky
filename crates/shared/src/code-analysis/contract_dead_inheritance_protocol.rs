// PURPOSE: IDeadInheritanceProtocol — port trait for AES303 sub-check 2: detect empty struct/impl blocks
use crate::cli_commands::taxonomy_result_vo::LintResult;
use crate::common::taxonomy_path_vo::FilePath;
use crate::common::taxonomy_source_vo::ContentString;

pub trait IDeadInheritanceProtocol: Send + Sync {
    fn check_dead_inheritance(
        &self,
        file: &FilePath,
        content: &ContentString,
        violations: &mut Vec<LintResult>,
    );
}
