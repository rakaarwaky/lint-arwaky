use crate::layer_rules::taxonomy_definition_vo::LayerDefinition;
use crate::naming_rules::taxonomy_symbols_vo::PrimitiveTypeList;
use crate::output_report::taxonomy_result_vo::LintResult;
use crate::shared_common::taxonomy_common_vo::ColumnNumber;
use crate::shared_common::taxonomy_common_vo::LineNumber;
use crate::shared_common::taxonomy_name_vo::AdapterName;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub struct PrimitiveViolation {
    pub line: LineNumber,
    pub column: ColumnNumber,
    pub type_name: AdapterName,
}

pub trait IPrimitiveCheckerProtocol: Send + Sync {
    fn check_primitive_usage(
        &self,
        file: &FilePath,
        content: &str,
        filename: &str,
        def: &LayerDefinition,
        violations: &mut Vec<LintResult>,
    );

    fn find_python_primitive_violations(
        &self,
        file_path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> Vec<PrimitiveViolation>;
}
