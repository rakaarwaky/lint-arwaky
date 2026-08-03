// AES006 negative test fixture — VO file with primitives is EXEMPT.
// VO files legitimately wrap primitives, so they must NOT flag AES006.

use crate::taxonomy::DescriptionVO;

pub struct PrimitivePassVo {
    pub name: String,
    pub age: i32,
    pub active: bool,
}

impl PrimitivePassVo {
    pub fn new(name: String, age: i32, active: bool) -> Self {
        Self { name, age, active }
    }
}
