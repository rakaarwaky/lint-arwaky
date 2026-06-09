use crate::code_analysis::taxonomy_import_source_vo::PrimitiveViolationList;
use crate::naming_rules::taxonomy_naming_list_vo::PrimitiveTypeList;
use crate::source_parsing::taxonomy_path_vo::FilePath;

pub trait IDomainTypeProtocol: Send + Sync {
    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList;
}
