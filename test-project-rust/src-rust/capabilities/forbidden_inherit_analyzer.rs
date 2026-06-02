// This capability file violates AES026 (forbidden-inheritance)
// because capability classes are prohibited from implementing/inheriting low-level infrastructure port contracts directly.
use crate::taxonomy::removal_types::RemovalType;
use crate::contract::removal_port::IRemovalPort; // Supposedly infrastructure port!

pub struct ForbiddenInheritAnalyzer;

impl IRemovalPort for ForbiddenInheritAnalyzer {
    fn remove_background(&self, img: Vec<u8>) -> Vec<u8> {
        // Direct implementation of technical Port in business capability is prohibited!
        img
    }
}
