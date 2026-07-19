// BAD: Std trait in Block 2 (wrong block order)
pub struct ArchLineChecker;

impl Default for ArchLineChecker {
    fn default() -> Self {
        Self
    }
}

impl ILineCheckerProtocol for ArchLineChecker {
    fn check_line_counts(&self, file: &FilePath) {
        // protocol method should be in Block 2, not after Default
    }
}
