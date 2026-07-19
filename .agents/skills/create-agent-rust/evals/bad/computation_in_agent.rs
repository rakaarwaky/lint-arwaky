// BAD: Computation in agent layer
pub struct OrphanOrchestrator;

impl OrphanOrchestrator {
    fn process(&self, files: &[FilePath]) {
        let total = files.len(); // BAD: domain/technical computation
        let sum: usize = files.iter().map(|f| f.size()).sum(); // BAD
    }
}
