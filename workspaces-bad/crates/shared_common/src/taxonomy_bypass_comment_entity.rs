// AES014: #[allow(...)] triggers bypass comment detection
#[allow(unused)]
pub struct BypassCommentEntity;

impl BypassCommentEntity {
    pub fn dummy(&self) -> u32 {
        42
    }
}
