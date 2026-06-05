// This taxonomy file violates AES001 Import Layer Violation by importing from capabilities layer
// Also, it is less than 10 lines, which violates AES005 file too short.

use crate::capabilities::removal_usecase::RemovalUsecase;

pub struct InvalidImportVo {
    pub message: String,
}
