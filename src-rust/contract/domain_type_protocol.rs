use super::*;

pub trait IDomainTypeProtocol: Send + Sync {
    fn find_primitive_violations(&self, path: &FilePath, primitive_types: &PrimitiveTypeList) -> PrimitiveViolationList;
}
