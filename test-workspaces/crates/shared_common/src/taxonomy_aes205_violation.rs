// AES205: Circular Import violation - this file creates a direct cycle with no break
// This creates an intentional circular dependency that should be flagged
pub struct CyclicStruct1;

pub struct CyclicStruct2;

impl CyclicStruct1 {
    pub fn new() -> Self { Self }
    pub fn get_partner(&self) -> CyclicStruct2 { CyclicStruct2 }
}

impl CyclicStruct2 {
    pub fn new() -> Self { Self }
    pub fn get_partner(&self) -> CyclicStruct1 { CyclicStruct1 }
}