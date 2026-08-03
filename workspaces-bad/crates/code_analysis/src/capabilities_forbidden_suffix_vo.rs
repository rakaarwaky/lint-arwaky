// AES011: capabilities file with forbidden _vo suffix
pub struct ForbiddenSuffixVo;
impl ForbiddenSuffixVo {
    pub fn check(&self) -> bool { true }
}
