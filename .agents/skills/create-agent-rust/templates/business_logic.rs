// BAD: Business logic in agent layer
pub struct OrphanOrchestrator;

impl OrphanOrchestrator {
    fn analyze(&self, content: &FileContent) -> bool {
        content.value().contains("orphan") // BAD: business rule
    }
}
