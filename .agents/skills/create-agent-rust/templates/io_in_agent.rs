// BAD: I/O in agent layer
pub struct OrphanOrchestrator;

impl OrphanOrchestrator {
    fn execute(&self, path: &FilePath) {
        let content = std::fs::read_to_string(path.value()); // BAD
    }
}
