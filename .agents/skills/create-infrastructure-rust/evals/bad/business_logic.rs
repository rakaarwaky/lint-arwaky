// BAD: Business logic in infrastructure
pub struct OrphanFileCache;

impl OrphanFileCache {
    fn analyze(&self, content: &FileContent) -> bool {
        // BAD: domain logic
        content.value().contains("orphan")
    }
}
