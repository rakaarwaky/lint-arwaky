/// python_primitive_checker — Analyzer for primitive type usage (AES006).
use crate::taxonomy::{ColumnNumber, FilePath, LineNumber, PrimitiveTypeList, PrimitiveTypeName, PrimitiveViolation, PrimitiveViolationList};

pub struct PrimitiveChecker;

impl PrimitiveChecker {
    pub fn new() -> Self {
        Self
    }
}
