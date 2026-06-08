use crate::source_parsing::taxonomy_path_vo::FilePath;
/* UNKNOWN: PrimitiveTypeList */ use crate::naming_rules::taxonomy_symbols_vo::PrimitiveTypeList;
/* UNKNOWN: PrimitiveViolationList */ use crate::code_analysis::taxonomy_source_vo::PrimitiveViolationList;

pub trait IDomainTypeProtocol: Send + Sync {
    fn find_primitive_violations(
        &self,
        path: &FilePath,
        primitive_types: &PrimitiveTypeList,
    ) -> PrimitiveViolationList;
}
