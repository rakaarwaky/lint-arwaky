// PURPOSE: AES405 — agent that carries block markers beyond Block 3
//
// Fixture for `check_agent_block_markers`: the structure is Block 1 (types and
// injected deps) -> Block 2 (aggregate impl) -> Block 3 (constructors, std
// traits, helpers). Block 4 and Block 5 mean the file has outgrown that shape
// and the behaviour they hold belongs in a capability or utility.
use shared::filesystem::taxonomy_filesystem_vo::FileEntry;
use std::sync::Arc;

// ─── Block 1: Struct Definitions ───────────────────
pub struct ExtraBlockAgent {
    scanner: Arc<dyn IFileScanProtocol>,
}

// ─── Block 2: Aggregate Trait Implementation ───────
impl IExtraBlockAggregate for ExtraBlockAgent {
    fn execute(&self, files: &[FileEntry]) -> usize {
        self.scanner.scan(files).len()
    }
}

// ─── Block 3: Constructors, Std Traits, Helpers ────
impl ExtraBlockAgent {
    pub fn new(scanner: Arc<dyn IFileScanProtocol>) -> Self {
        Self { scanner }
    }
}

// ─── Block 4: Extra Seams ───────────────────
// AES405: no fourth block — fold this into Block 3 or move it out.
impl ExtraBlockAgent {
    fn collect(&self, files: &[FileEntry]) -> Vec<FileEntry> {
        files.to_vec()
    }
}

// ─── Block 5: Reporting ───────────────────
// AES405: no fifth block either.
impl ExtraBlockAgent {
    fn summarise(&self) -> String {
        String::new()
    }
}