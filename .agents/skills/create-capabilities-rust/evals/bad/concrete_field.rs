// BAD: Concrete service field instead of DI (AES403)
use std::sync::Arc;

pub struct CapabilitiesOrphanAnalyzer {
    extractor: FilenameExtractor, // BAD: concrete type
}

impl CapabilitiesOrphanAnalyzer {
    pub fn new(extractor: FilenameExtractor) -> Self {
        Self { extractor }
    }
}
