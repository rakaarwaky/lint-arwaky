// BAD: No port trait implementation (AES404)
pub struct FileCache;

impl FileCache {
    pub fn read(&self) {
        // public behavior without port trait
    }
}
