// AES020: File exceeds max line limit (700 for Python/JS, 1000 for Rust)
// This file is intentionally long to trigger AES020 file size violation
#![allow(dead_code)]
pub struct LargeEntity;

impl LargeEntity {
    pub fn dummy(&self) -> u32 { 1 }
}
