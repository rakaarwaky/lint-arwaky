use crate::taxonomy::FilePath;
use crate::taxonomy::PrimitiveTypeList;
use crate::taxonomy::PrimitiveViolationList;
use super::*;

pub trait IDomainTypeProtocol: Send + Sync {
    fn find_primitive_violations(&self, path: &FilePath, primitive_types: &PrimitiveTypeList) -> PrimitiveViolationList;
}
