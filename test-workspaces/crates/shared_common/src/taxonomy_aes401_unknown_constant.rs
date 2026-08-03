// AES401: constant file contains non-constant declaration (struct)
pub struct NonConstantEntity;

impl NonConstantEntity {
    pub fn method(&self) -> i32 {
        42
    }
}
